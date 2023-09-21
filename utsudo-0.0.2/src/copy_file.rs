/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

use crate::struct_macro::*;

use crate::S_ISREG;

use utsudo_util::debug_decl;
use utsudo_util::debug_decl_vars;
use utsudo_util::debug_return_bool;
use utsudo_util::debug_return_int;
use utsudo_util::sudo_debug_macro::sudo_debug_subsys;
use utsudo_util::sudo_debug_macro::SUDO_DEBUG_ERRNO;
use utsudo_util::sudo_debug_macro::SUDO_DEBUG_ERROR;
use utsudo_util::sudo_debug_macro::SUDO_DEBUG_INFO;
use utsudo_util::sudo_debug_macro::SUDO_DEBUG_LINENO;
use utsudo_util::sudo_debug_macro::SUDO_DEBUG_UTIL;

use utsudo_util::sudo_debug_printf;
use utsudo_util::sudo_warn;
use utsudo_util::sudo_warnx;


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

/*
 * Copy the contents of src_fd into dst_fd.
 * Returns 0 on success or -1 on error.
 */
#[no_mangle]
pub unsafe extern "C" fn sudo_copy_file(
    mut src: *const libc::c_char,
    mut src_fd: libc::c_int,
    mut src_len: off_t,
    mut dst: *const libc::c_char,
    mut dst_fd: libc::c_int,
    mut dst_len: off_t,
) -> libc::c_int {
    let mut buf: [libc::c_char; BUFSIZ as usize] = [0; BUFSIZ as usize];
    let mut nwritten: ssize_t = 0 as ssize_t;
    let mut nread: ssize_t = 0 as ssize_t;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    /* Extend the file to the new size if larger before copying. */
    'write_error: loop {
        if dst_len > 0 as off_t && src_len > dst_len {
            if sudo_extend_file(dst_fd, dst, src_len) == -1 {
                break 'write_error;
            }
        }
        loop {
            /* Overwrite the old file with the new contents.*/
            nread = read(
                src_fd,
                buf.as_mut_ptr() as *mut libc::c_void,
                std::mem::size_of::<[libc::c_char; BUFSIZ as usize]>() as size_t,
            );
            if !(nread > 0 as ssize_t) {
                break;
            }
            let mut off: ssize_t = 0 as ssize_t;
            loop {
                nwritten = write(
                    dst_fd,
                    buf.as_mut_ptr().offset(off as isize) as *const libc::c_void,
                    (nread - off) as size_t,
                );
                if nwritten == -(1 as libc::c_int) as libc::c_long {
                    break 'write_error;
                }
                off += nwritten;
                if !(nread > off) {
                    break;
                }
            }
        }

        if nread == 0 as ssize_t {
            /* success, read to EOF */
            if src_len < dst_len {
                /* We don't open with O_TRUNC so must truncate manually. */
                if ftruncate(dst_fd, src_len) == -1 {
                    sudo_debug_printf!(
                        SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                        b"unable to truncate %s to %lld\0" as *const u8 as *const libc::c_char,
                        dst,
                        src_len as libc::c_longlong
                    );
                    break 'write_error;
                }
            }
            debug_return_int!(0);
        } else if nread < 0 as ssize_t {
            sudo_warn!(
                b"unable to read from %s\0" as *const u8 as *const libc::c_char,
                src
            );
            debug_return_int!(-(1 as libc::c_int));
        } else {
            break 'write_error;
        }
        break;
    }
    sudo_warn!(
        b"unable to write to %s\0" as *const u8 as *const libc::c_char,
        dst
    );
    debug_return_int!(-(1 as libc::c_int));
}
