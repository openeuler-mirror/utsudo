/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */
#![allow(
    dead_code,
    unreachable_code,
    unused_variables,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn sudo_digest_free_v1(dig: *mut sudo_digest);
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn sudo_digest_getlen_v1(digest_type: libc::c_int) -> libc::c_int;
    fn sudo_warnx_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_digest_alloc_v1(digest_type: libc::c_int) -> *mut sudo_digest;
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn dup(__fd: libc::c_int) -> libc::c_int;
    fn sudo_digest_final_v1(dig: *mut sudo_digest, md: *mut libc::c_uchar);
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn sudo_digest_update_v1(dig: *mut sudo_digest, data: *const libc::c_void, len: size_t);
    static mut sudoers_subsystem_ids: [libc::c_uint; 0];
}
use crate::common::*;

#[no_mangle]
pub unsafe extern "C" fn sudo_filedigest(
    mut fd: libc::c_int,
    mut file: *const libc::c_char,
    mut digest_type: libc::c_int,
    mut digest_len: *mut size_t,
) -> *mut libc::c_uchar {
    let mut file_digest: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut buf: [libc::c_uchar; 32768] = [0; 32768];
    let mut dig: *mut sudo_digest = 0 as *mut sudo_digest;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut nread: size_t = 0;
    let mut fd2: libc::c_int = 0;
    let sudo_debug_subsys_1: libc::c_int = *sudoers_subsystem_ids
        .as_mut_ptr()
        .offset(17 as libc::c_int as isize)
        as libc::c_int;
    debug_decl!(SUDOERS_DEBUG_UTIL!());
    *digest_len = sudo_digest_getlen_v1(digest_type) as size_t;
    'bad: loop {
        if *digest_len == -(1 as libc::c_int) as size_t {
            sudo_warnx!(
                b"unsupported digest type %d for %s\0" as *const u8 as *const libc::c_char,
                digest_type,
                file
            );
            break 'bad;
        }
        dig = sudo_digest_alloc_v1(digest_type);
        if dig.is_null() {
            sudo_warnx!(
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                get_function_name!(),
                b"unable to allocate memory\0" as *const u8 as *const libc::c_char
            );
            break 'bad;
        }
        fd2 = dup(fd);
        if fd2 == -(1 as libc::c_int) {
            sudo_debug_printf!(
                SUDO_DEBUG_INFO,
                b"unable to dup %s: %s\n" as *const u8 as *const libc::c_char,
                file,
                strerror(*__errno_location())
            );
            break 'bad;
        }
        fp = fdopen(fd2, b"r\0" as *const u8 as *const libc::c_char);
        if fp.is_null() {
            sudo_debug_printf!(
                SUDO_DEBUG_INFO,
                b"unable to fdopen %s: %s\n" as *const u8 as *const libc::c_char,
                file,
                strerror(*__errno_location())
            );
            close(fd2);
            break 'bad;
        }
        file_digest = malloc(*digest_len as size_t) as *mut libc::c_uchar;
        if file_digest.is_null() {
            sudo_warnx!(
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                get_function_name!(),
                b"unable to allocate memory\0" as *const u8 as *const libc::c_char
            );
            break 'bad;
        }
        nread = fread(
            buf.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<[libc::c_uchar; 32768]>() as libc::c_ulong,
            fp,
        );
        while nread != 0 as libc::c_int as libc::c_ulong {
            sudo_digest_update_v1(dig, buf.as_mut_ptr() as *const libc::c_void, nread);
        }
        if ferror(fp) != 0 {
            sudo_warnx!(
                b"%s: read error\0" as *const u8 as *const libc::c_char,
                file
            );
            break 'bad;
        }
        sudo_digest_final_v1(dig, file_digest);
        sudo_digest_free_v1(dig);
        fclose(fp);
        debug_return_ptr!(file_digest);
        break 'bad;
    } //end of bad;
    sudo_digest_free_v1(dig);
    free(file_digest as *mut libc::c_void);
    if !fp.is_null() {
        fclose(fp);
    }
    debug_return_ptr!(0 as *mut libc::c_uchar);
}


