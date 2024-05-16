/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */
#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unused_attributes
)]
#![feature(extern_types)]
use crate::common::*;
extern "C" {
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE) -> *mut libc::c_char;
    fn rewind(__stream: *mut FILE);
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn sudo_strtoid_v2(str: *const libc::c_char, errstr: *mut *const libc::c_char) -> id_t;
}
pub type __gid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __id_t = libc::c_uint;
pub type gid_t = __gid_t;
pub type id_t = __id_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub const GRMEM_MAX: libc::c_int = 200;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub gr_name: *mut libc::c_char,
    pub gr_passwd: *mut libc::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut libc::c_char,
}
static mut grf: *mut FILE = 0 as *const FILE as *mut FILE;
static mut grfile: *const libc::c_char = b"/etc/group\0" as *const u8 as *const libc::c_char;
static mut gr_stayopen: libc::c_int = 0;

#[no_mangle]
pub unsafe extern "C" fn mysetgrfile(mut file: *const libc::c_char) {
    grfile = file;
    if !grf.is_null() {
        myendgrent();
    }
}
#[no_mangle]
pub unsafe extern "C" fn mysetgrent() {
    if grf.is_null() {
        grf = fopen(grfile, b"r\0" as *const u8 as *const libc::c_char);
        if !grf.is_null() {
            fcntl(fileno(grf), F_SETFD, FD_CLOEXEC);
        }
    } else {
        rewind(grf);
    }
    gr_stayopen = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn myendgrent() {
    if !grf.is_null() {
        fclose(grf);
        grf = 0 as *mut FILE;
    }
    gr_stayopen = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mygetgrent() -> *mut group {
    static mut gr: group = group {
        gr_name: 0 as *const libc::c_char as *mut libc::c_char,
        gr_passwd: 0 as *const libc::c_char as *mut libc::c_char,
        gr_gid: 0,
        gr_mem: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    };
    static mut grbuf: [libc::c_char; 2048] = [0; 2048];
    static mut gr_mem: [*mut libc::c_char; 201] =
        [0 as *const libc::c_char as *mut libc::c_char; 201];
    let mut len: size_t = 0;
    let mut id: id_t = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut colon: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut n: libc::c_int = 0;
    loop {
        colon = fgets(
            grbuf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong as libc::c_int,
            grf,
        );
        if colon.is_null() {
            return 0 as *mut group;
        }
        memset(
            &mut gr as *mut group as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<group>() as libc::c_ulong,
        );
        cp = colon;
        colon = strchr(cp, ':' as i32);
        if colon.is_null() {
            continue;
        }
        let fresh0 = colon;
        colon = colon.offset(1);
        *fresh0 = '\0' as i32 as libc::c_char;
        gr.gr_name = cp;
        cp = colon;
        colon = strchr(cp, ':' as i32);
        if colon.is_null() {
            continue;
        }
        let fresh1 = colon;
        colon = colon.offset(1);
        *fresh1 = '\0' as i32 as libc::c_char;
        gr.gr_passwd = cp;
        cp = colon;
        colon = strchr(cp, ':' as i32);
        if colon.is_null() {
            continue;
        }
        let fresh2 = colon;
        colon = colon.offset(1);
        *fresh2 = '\0' as i32 as libc::c_char;
        id = sudo_strtoid_v2(cp, &mut errstr);
        if errstr.is_null() {
            break;
        }
    }
    gr.gr_gid = id;
    len = strlen(colon);
    if len > 0 as libc::c_int as libc::c_ulong
        && *colon.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int
            == '\n' as i32
    {
        *colon.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize) =
            '\0' as i32 as libc::c_char;
    }
    if *colon as libc::c_int != '\0' as i32 {
        let mut last: *mut libc::c_char = 0 as *mut libc::c_char;
        gr.gr_mem = gr_mem.as_mut_ptr();
        cp = strtok_r(colon, b",\0" as *const u8 as *const libc::c_char, &mut last);
        n = 0 as libc::c_int;
        while !cp.is_null() && n < GRMEM_MAX as libc::c_int {
            let ref mut fresh3 = *(gr.gr_mem).offset(n as isize);
            *fresh3 = cp;
            cp = strtok_r(
                0 as *mut libc::c_char,
                b",\0" as *const u8 as *const libc::c_char,
                &mut last,
            );
            n += 1;
        }
        let fresh4 = n;
        n = n + 1;
        let ref mut fresh5 = *(gr.gr_mem).offset(fresh4 as isize);
        *fresh5 = 0 as *mut libc::c_char;
    } else {
        gr.gr_mem = 0 as *mut *mut libc::c_char;
    }
    return &mut gr;
}
#[no_mangle]
pub unsafe extern "C" fn mygetgrnam(mut name: *const libc::c_char) -> *mut group {
    let mut gr: *mut group = 0 as *mut group;
    if grf.is_null() {
        grf = fopen(grfile, b"r\0" as *const u8 as *const libc::c_char);
        if grf.is_null() {
            return 0 as *mut group;
        }
        fcntl(fileno(grf), F_SETFD, FD_CLOEXEC);
    } else {
        rewind(grf);
    }
    loop {
        gr = mygetgrent();
        if gr.is_null() {
            break;
        }
        if strcmp((*gr).gr_name, name) == 0 as libc::c_int {
            break;
        }
    }
    if gr_stayopen == 0 {
        fclose(grf);
        grf = 0 as *mut FILE;
    }
    return gr;
}

#[no_mangle]
pub unsafe extern "C" fn mygetgrgid(mut gid: gid_t) -> *mut group {
    let mut gr: *mut group = 0 as *mut group;
    if grf.is_null() {
        grf = fopen(grfile, b"r\0" as *const u8 as *const libc::c_char);
        if grf.is_null() {
            return 0 as *mut group;
        }
        fcntl(fileno(grf), 2 as libc::c_int, 1 as libc::c_int);
    } else {
        rewind(grf);
    }
    loop {
        gr = mygetgrent();
        if gr.is_null() {
            break;
        }
        if (*gr).gr_gid == gid {
            break;
        }
    }
    if gr_stayopen == 0 {
        fclose(grf);
        grf = 0 as *mut FILE;
    }
    return gr;
}


