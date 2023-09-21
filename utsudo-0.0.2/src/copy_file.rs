/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

use crate::struct_macro::*;

use crate::S_ISREG;

/*
 * Extend the given fd to the specified size in bytes.
 * We do this to allocate disk space up-front before overwriting
 * the original file with the temporary.  Otherwise, we could
 * we run out of disk space after truncating the original file.
 */
#[inline]
unsafe extern "C" fn sudo_extend_file(
    mut fd: libc::c_int,
    mut name: *const libc::c_char,
    mut new_size: off_t,
) -> libc::c_int {
    let mut old_size: off_t = 0;
    let mut size: off_t = 0;
    let mut nwritten: ssize_t = 0;
    let mut zeroes: [libc::c_char; BUFSIZ as usize] = [0; BUFSIZ as usize];
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    old_size = lseek(fd, 0 as off_t, SEEK_END);
    if old_size == -(1 as libc::c_int) as libc::c_long {
        sudo_warn!(b"lseek\0" as *const u8 as *const libc::c_char,);
        debug_return_int!(-(1 as libc::c_int));
    }
    sudo_debug_printf!(
        SUDO_DEBUG_INFO,
        b"%s: extending %s from %lld to %lld %s\0" as *const u8 as *const libc::c_char,
        stdext::function_name!().as_ptr(),
        name,
        old_size,
        new_size
    );

    size = old_size;
    while size < new_size {
        let mut len: size_t = (new_size - size) as size_t;

        if len > std::mem::size_of::<[libc::c_char; BUFSIZ as usize]>() as size_t {
            len = std::mem::size_of::<[libc::c_char; BUFSIZ as usize]>() as size_t;
        }
        nwritten = write(fd, zeroes.as_mut_ptr() as *const libc::c_void, len);
        if nwritten == -1 as ssize_t {
            let mut serrno: libc::c_int = *__errno_location();
            if ftruncate(fd, old_size) == -1 {
                sudo_debug_printf!(
                    SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                    b"unable to truncate %s to %lld \0" as *const u8 as *const libc::c_char,
                    name,
                    old_size
                );
            }
            *__errno_location() = serrno;
            debug_return_int!(-(1 as libc::c_int));
        }
        size += nwritten
    }
    if lseek(fd, 0 as off_t, SEEK_SET) == -(1 as libc::c_int) as libc::c_long {
        sudo_warn!(b"lseek\0" as *const u8 as *const libc::c_char,);
        debug_return_int!(-(1 as libc::c_int));
    }
    debug_return_int!(0);
}
