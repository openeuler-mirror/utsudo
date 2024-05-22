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
    unused_mut
)]
use crate::common::*;
extern "C" {
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn reallocarray(__ptr: *mut libc::c_void, __nmemb: size_t, __size: size_t)
        -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn __errno_location() -> *mut libc::c_int;
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    static mut sudoers_subsystem_ids: [libc::c_uint; 0];
    fn sudo_strtoid_v2(str: *const libc::c_char, errstr: *mut *const libc::c_char) -> id_t;
    static mut filters: *mut cvtsudoers_filter;
}

pub const MAX_UID_T_LEN: usize = 10;
pub const _SC_LOGIN_NAME_MAX: libc::c_uint = 71;

/*
 * Dynamically allocate space for a struct item plus the key and data
 * elements.  If name is non-NULL it is used as the key, else the
 * uid is the key.  Fills in datum from the users filter.
 * Returns NULL on calloc error or unknown name/id, setting errno
 * to ENOMEM or ENOENT respectively.
 */
#[no_mangle]
pub unsafe extern "C" fn cvtsudoers_make_pwitem(
    mut uid: uid_t,
    mut name: *const libc::c_char,
) -> *mut cache_item {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut uidstr: [libc::c_char; MAX_UID_T_LEN + 2] = [0; MAX_UID_T_LEN + 2];
    let mut nsize: size_t = 0;
    let mut psize: size_t = 0;
    let mut csize: size_t = 0;
    let mut gsize: size_t = 0;
    let mut dsize: size_t = 0;
    let mut ssize: size_t = 0;
    let mut total: size_t = 0;
    let mut pwitem: *mut cache_item_pw = 0 as *mut cache_item_pw;
    let mut pw: passwd = passwd {
        pw_name: 0 as *mut libc::c_char,
        pw_passwd: 0 as *mut libc::c_char,
        pw_uid: 0,
        pw_gid: 0,
        pw_gecos: 0 as *mut libc::c_char,
        pw_dir: 0 as *mut libc::c_char,
        pw_shell: 0 as *mut libc::c_char,
    };
    let mut newpw: *mut passwd = 0 as *mut passwd;
    let mut s: *mut sudoers_string = 0 as *mut sudoers_string;
    debug_decl!(SUDOERS_DEBUG_NSS!());

    /* Look up name or uid in filter list. */
    if !name.is_null() {
        s = (*filters).users.stqh_first;
        while !s.is_null() {
            if strcasecmp(name, (*s).str_0) == 0 as libc::c_int {
                uid = -(1 as libc::c_int) as uid_t;
                break;
            } else {
                s = (*s).entries.stqe_next;
            }
        }
    } else {
        s = (*filters).users.stqh_first;
        while !s.is_null() {
            let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
            let mut filter_uid: uid_t = 0;

            if !(*((*s).str_0).offset(0 as isize) as libc::c_int != '#' as i32) {
                filter_uid = sudo_strtoid_v2(((*s).str_0).offset(1 as isize), &mut errstr);
                if errstr.is_null() {
                    if !(uid != filter_uid) {
                        snprintf(
                            uidstr.as_mut_ptr(),
                            ::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
                            b"#%u\0" as *const u8 as *const libc::c_char,
                            uid,
                        );
                        break;
                    }
                }
            }
            s = (*s).entries.stqe_next;
        }
    }
    if s.is_null() {
        *__errno_location() = ENOENT;
        debug_return_ptr!(0 as *mut cache_item);
    }

    /* Fake up a passwd struct. */
    memset(
        &mut pw as *mut passwd as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<passwd>() as libc::c_ulong,
    );
    pw.pw_name = if !name.is_null() {
        (*s).str_0
    } else {
        uidstr.as_mut_ptr()
    };
    pw.pw_passwd = b"*\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    pw.pw_uid = uid;
    pw.pw_gid = -(1 as libc::c_int) as gid_t;
    pw.pw_shell = _PATH_BSHELL!();
    pw.pw_dir = b"/\0" as *const u8 as *const libc::c_char as *mut libc::c_char;

    /* Allocate in one big chunk for easy freeing. */
    ssize = 0 as size_t;
    dsize = ssize;
    gsize = dsize;
    csize = gsize;
    psize = csize;
    nsize = psize;
    total = ::core::mem::size_of::<cache_item_pw>() as libc::c_ulong;
    FIELD_SIZE!(pw.pw_name, nsize, total);
    FIELD_SIZE!(pw.pw_passwd, psize, total);
    FIELD_SIZE!(pw.pw_gecos, gsize, total);
    FIELD_SIZE!(pw.pw_dir, dsize, total);
    FIELD_SIZE!(pw.pw_shell, ssize, total);
    if !name.is_null() {
        total = (total as libc::c_ulong)
            .wrapping_add((strlen(name)).wrapping_add(1 as libc::c_ulong)) as size_t
            as size_t;
    }

    /* Allocate space for struct item, struct passwd and the strings. */
    pwitem = calloc(1 as libc::c_ulong, total) as *mut cache_item_pw;
    if pwitem.is_null() {
        sudo_debug_printf!(
            SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
        debug_return_ptr!(0 as *mut cache_item);
    }
    newpw = &mut (*pwitem).pw;

    /*
     * Copy in passwd contents and make strings relative to space
     * at the end of the struct.
     */
    memcpy(
        newpw as *mut libc::c_void,
        &mut pw as *mut passwd as *const libc::c_void,
        ::core::mem::size_of::<passwd>() as libc::c_ulong,
    );
    cp = pwitem.offset(1 as isize) as *mut libc::c_char;
    FIELD_COPY!(pw.pw_name, (*newpw).pw_name, nsize, cp);
    FIELD_COPY!(pw.pw_passwd, (*newpw).pw_passwd, psize, cp);
    FIELD_COPY!(pw.pw_gecos, (*newpw).pw_gecos, gsize, cp);
    FIELD_COPY!(pw.pw_dir, (*newpw).pw_dir, dsize, cp);
    FIELD_COPY!(pw.pw_shell, (*newpw).pw_shell, ssize, cp);

    /* Set key and datum. */
    if !name.is_null() {
        memcpy(
            cp as *mut libc::c_void,
            name as *const libc::c_void,
            (strlen(name)).wrapping_add(1 as libc::c_ulong),
        );
        (*pwitem).cache.k.name = cp;
    } else {
        (*pwitem).cache.k.uid = pw.pw_uid;
    }
    (*pwitem).cache.d.pw = newpw;
    (*pwitem).cache.refcnt = 1 as libc::c_uint;

    debug_return_ptr!(&mut (*pwitem).cache as *mut cache_item);
}