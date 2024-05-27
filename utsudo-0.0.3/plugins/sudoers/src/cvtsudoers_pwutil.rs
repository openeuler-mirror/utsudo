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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cache_item {
    pub refcnt: libc::c_uint,
    pub type_0: libc::c_uint,
    pub registry: [libc::c_char; 16],
    pub k: C2RustUnnamed_1,
    pub d: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub pw: *mut passwd,
    pub gr: *mut group,
    pub grlist: *mut group_list,
    pub gidlist: *mut gid_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub uid: uid_t,
    pub gid: gid_t,
    pub name: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudoers_string {
    pub entries: C2RustUnnamed_2,
    pub str_0: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub stqe_next: *mut sudoers_string,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudoers_str_list {
    pub stqh_first: *mut sudoers_string,
    pub stqh_last: *mut *mut sudoers_string,
    pub refcnt: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cvtsudoers_filter {
    pub users: sudoers_str_list,
    pub groups: sudoers_str_list,
    pub hosts: sudoers_str_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cache_item_pw {
    pub cache: cache_item,
    pub pw: passwd,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cache_item_gr {
    pub cache: cache_item,
    pub gr: group,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cache_item_gidlist {
    pub cache: cache_item,
    pub gidlist: gid_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cache_item_grlist {
    pub cache: cache_item,
    pub grlist: group_list,
}


#[macro_export]
macro_rules! _PATH_BSHELL {
    () => {
        (b"/bin/sh\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
    };
}
#[macro_export]
macro_rules! FIELD_SIZE {
    ($src:expr, $size:expr, $total:expr) => {{
        if !($src).is_null() {
            $size = (strlen($src)).wrapping_add(1 as libc::c_ulong);
            $total = ($total as libc::c_ulong).wrapping_add($size) as size_t as size_t;
        }
    }};
}
#[macro_export]
macro_rules! FIELD_COPY {
    ($src:expr, $dst:expr, $size:expr, $cp:expr) => {{
        if !($src).is_null() {
            memcpy($cp as *mut libc::c_void, $src as *const libc::c_void, $size);
            $dst = $cp;
            $cp = $cp.offset($size as isize);
        }
    }};
}
#[macro_export]
macro_rules! MAX {
    ($a:expr, $b:expr) => {
        (if ($a) > ($b) { ($a) } else { ($b) })
    };
}


static mut gidlist_item: *mut cache_item_gidlist =
    0 as *const cache_item_gidlist as *mut cache_item_gidlist;

static mut grlist_item: *mut cache_item_gidlist =
    0 as *const cache_item_gidlist as *mut cache_item_gidlist;

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

/*
 * Dynamically allocate space for a struct item plus the key and data
 * elements.  If name is non-NULL it is used as the key, else the
 * gid is the key.  Fills in datum from the groups filter.
 * Returns NULL on calloc error or unknown name/id, setting errno
 * to ENOMEM or ENOENT respectively.
 */
#[no_mangle]
pub unsafe extern "C" fn cvtsudoers_make_gritem(
    mut gid: gid_t,
    mut name: *const libc::c_char,
) -> *mut cache_item {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut gidstr: [libc::c_char; MAX_UID_T_LEN + 2] = [0; MAX_UID_T_LEN + 2];
    let mut nsize: size_t = 0;
    let mut psize: size_t = 0;
    let mut nmem: size_t = 0;
    let mut total: size_t = 0;
    let mut len: size_t = 0;
    let mut gritem: *mut cache_item_gr = 0 as *mut cache_item_gr;
    let mut gr: group = group {
        gr_name: 0 as *mut libc::c_char,
        gr_passwd: 0 as *mut libc::c_char,
        gr_gid: 0,
        gr_mem: 0 as *mut *mut libc::c_char,
    };
    let mut newgr: *mut group = 0 as *mut group;
    let mut s: *mut sudoers_string = 0 as *mut sudoers_string;
    debug_decl!(SUDOERS_DEBUG_NSS!());

    /* Look up name or gid in filter list. */
    if !name.is_null() {
        s = (*filters).groups.stqh_first;
        while !s.is_null() {
            if strcasecmp(name, (*s).str_0) == 0 as libc::c_int {
                gid = -(1 as libc::c_int) as gid_t;
                break;
            } else {
                s = (*s).entries.stqe_next;
            }
        }
    } else {
        s = (*filters).groups.stqh_first;
        while !s.is_null() {
            let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
            let mut filter_gid: gid_t = 0;

            if !(*((*s).str_0).offset(0 as libc::c_int as isize) as libc::c_int != '#' as i32) {
                filter_gid =
                    sudo_strtoid_v2(((*s).str_0).offset(1 as libc::c_int as isize), &mut errstr);
                if errstr.is_null() {
                    if !(gid != filter_gid) {
                        snprintf(
                            gidstr.as_mut_ptr(),
                            ::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
                            b"#%u\0" as *const u8 as *const libc::c_char,
                            gid,
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

    /* Fake up a group struct with all filter users as members. */
    memset(
        &mut gr as *mut group as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<group>() as libc::c_ulong,
    );
    gr.gr_name = if !name.is_null() {
        (*s).str_0
    } else {
        gidstr.as_mut_ptr()
    };
    gr.gr_gid = gid;

    /* Allocate in one big chunk for easy freeing. */
    nmem = 0 as size_t;
    psize = nmem;
    nsize = psize;
    total = ::core::mem::size_of::<cache_item_gr>() as libc::c_ulong;
    FIELD_SIZE!(gr.gr_name, nsize, total);
    FIELD_SIZE!(gr.gr_passwd, psize, total);
    if !((*filters).users.stqh_first).is_null() {
        s = (*filters).users.stqh_first;
        while !s.is_null() {
            total = (total as libc::c_ulong)
                .wrapping_add((strlen((*s).str_0)).wrapping_add(1 as libc::c_int as libc::c_ulong))
                as size_t as size_t;
            nmem = nmem.wrapping_add(1);
            s = (*s).entries.stqe_next;
        }
        total = (total as libc::c_ulong).wrapping_add(
            (::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong).wrapping_mul(nmem),
        ) as size_t as size_t;
    }
    if !name.is_null() {
        total = (total as libc::c_ulong)
            .wrapping_add((strlen(name)).wrapping_add(1 as libc::c_int as libc::c_ulong))
            as size_t as size_t;
    }

    gritem = calloc(1 as libc::c_ulong, total) as *mut cache_item_gr;
    if gritem.is_null() {
        sudo_debug_printf!(
            SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
        debug_return_ptr!(0 as *mut cache_item);
    }

    /*
     * Copy in group contents and make strings relative to space
     * at the end of the buffer.  Note that gr_mem must come
     * immediately after struct group to guarantee proper alignment.
     */
    newgr = &mut (*gritem).gr;
    memcpy(
        newgr as *mut libc::c_void,
        &mut gr as *mut group as *const libc::c_void,
        ::core::mem::size_of::<group>() as libc::c_ulong,
    );
    cp = gritem.offset(1 as isize) as *mut libc::c_char;
    if nmem != 0 as libc::c_ulong {
        (*newgr).gr_mem = cp as *mut *mut libc::c_char;
        cp = cp.offset(
            (::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong).wrapping_mul(nmem)
                as isize,
        );
        nmem = 0 as size_t;
        s = (*filters).users.stqh_first;
        while !s.is_null() {
            len = (strlen((*s).str_0)).wrapping_add(1 as libc::c_ulong);
            memcpy(
                cp as *mut libc::c_void,
                (*s).str_0 as *const libc::c_void,
                len,
            );
            let fresh0 = nmem;
            nmem = nmem.wrapping_add(1);
            let ref mut fresh1 = *((*newgr).gr_mem).offset(fresh0 as isize);
            *fresh1 = cp;
            cp = cp.offset(len as isize);
            s = (*s).entries.stqe_next;
        }
        let ref mut fresh2 = *((*newgr).gr_mem).offset(nmem as isize);
        *fresh2 = 0 as *mut libc::c_char;
    }
    FIELD_COPY!(gr.gr_passwd, (*newgr).gr_passwd, psize, cp);
    FIELD_COPY!(gr.gr_name, (*newgr).gr_name, nsize, cp);

    /* Set key and datum. */
    if !name.is_null() {
        memcpy(
            cp as *mut libc::c_void,
            name as *const libc::c_void,
            (strlen(name)).wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        (*gritem).cache.k.name = cp;
    } else {
        (*gritem).cache.k.gid = gr.gr_gid;
    }
    (*gritem).cache.d.gr = newgr;
    (*gritem).cache.refcnt = 1 as libc::c_uint;

    debug_return_ptr!(&mut (*gritem).cache as *mut cache_item);
}

/*
 * Dynamically allocate space for a struct item plus the key and data
 * elements.  Fills in datum from the groups filter.
 */
#[no_mangle]
pub unsafe extern "C" fn cvtsudoers_make_gidlist_item(
    mut pw: *const passwd,
    mut unused1: *const *mut libc::c_char,
    mut type_0: libc::c_uint,
) -> *mut cache_item {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nsize: size_t = 0;
    let mut total: size_t = 0;
    let mut glitem: *mut cache_item_gidlist = 0 as *mut cache_item_gidlist;
    let mut s: *mut sudoers_string = 0 as *mut sudoers_string;
    let mut gidlist: *mut gid_list = 0 as *mut gid_list;
    let mut gids: *mut gid_t = 0 as *mut gid_t;
    let mut i: libc::c_int = 0;
    let mut ngids: libc::c_int = 0 as libc::c_int;
    debug_decl!(SUDOERS_DEBUG_NSS!());

    /*
     * There's only a single gid list.
     */
    if !gidlist_item.is_null() {
        (*gidlist_item).cache.refcnt = ((*gidlist_item).cache.refcnt).wrapping_add(1);
        debug_return_ptr!(&mut (*gidlist_item).cache as *mut cache_item);
    }

    /* Count number of possible gids in the filter. */
    s = (*filters).groups.stqh_first;
    while !s.is_null() {
        if *((*s).str_0).offset(0 as isize) as libc::c_int == '#' as i32 {
            ngids += 1;
        }
        s = (*s).entries.stqe_next;
    }

    /* Allocate gids[] array and fill it with parsed gids. */
    if ngids != 0 as libc::c_int {
        gids = reallocarray(
            0 as *mut libc::c_void,
            ngids as size_t,
            ::core::mem::size_of::<gid_t>() as libc::c_ulong,
        ) as *mut gid_t;
        if gids.is_null() {
            sudo_debug_printf!(
                SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
                b"unable to allocate memory\0" as *const u8 as *const libc::c_char
            );
            debug_return_ptr!(0 as *mut cache_item);
        }
        ngids = 0 as libc::c_int;
        s = (*filters).groups.stqh_first;
        while !s.is_null() {
            if *((*s).str_0).offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32 {
                let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
                let mut gid: gid_t =
                    sudo_strtoid_v2(((*s).str_0).offset(1 as libc::c_int as isize), &mut errstr);
                if errstr.is_null() {
                    ngids = ngids + 1;
                    /* Valid gid. */
                    *gids.offset(ngids as isize) = gid;
                }
            }
            s = (*s).entries.stqe_next;
        }
    }
    if ngids == 0 as libc::c_int {
        free(gids as *mut libc::c_void);
        *__errno_location() = ENOENT;
        debug_return_ptr!(0 as *mut cache_item);
    }

    /* Allocate in one big chunk for easy freeing. */
    nsize = (strlen((*pw).pw_name)).wrapping_add(1 as libc::c_ulong);
    total = (::core::mem::size_of::<cache_item_gidlist>() as libc::c_ulong).wrapping_add(nsize);
    total = (total as libc::c_ulong).wrapping_add(
        (::core::mem::size_of::<*mut gid_t>() as libc::c_ulong)
            .wrapping_mul(ngids as libc::c_ulong),
    ) as size_t as size_t;

    glitem = calloc(1 as libc::c_ulong, total) as *mut cache_item_gidlist;
    if glitem.is_null() {
        sudo_debug_printf!(
            SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
        free(gids as *mut libc::c_void);
        debug_return_ptr!(0 as *mut cache_item);
    }

    /*
     * Copy in group list and make pointers relative to space
     * at the end of the buffer.  Note that the groups array must come
     * immediately after struct group to guarantee proper alignment.
     */
    gidlist = &mut (*glitem).gidlist;
    cp = glitem.offset(1 as isize) as *mut libc::c_char;
    (*gidlist).gids = cp as *mut gid_t;
    cp = cp.offset(
        (::core::mem::size_of::<gid_t>() as libc::c_ulong).wrapping_mul(ngids as libc::c_ulong)
            as isize,
    );

    /* Set key and datum. */
    memcpy(
        cp as *mut libc::c_void,
        (*pw).pw_name as *const libc::c_void,
        nsize,
    );
    (*glitem).cache.k.name = cp;
    (*glitem).cache.d.gidlist = gidlist;
    (*glitem).cache.refcnt = 1 as libc::c_uint;
    (*glitem).cache.type_0 = type_0;

    /*
     * Store group IDs.
     */
    i = 0 as libc::c_int;
    while i < ngids {
        *((*gidlist).gids).offset(i as isize) = *gids.offset(i as isize);
        i += 1;
    }
    (*gidlist).ngids = ngids;
    free(gids as *mut libc::c_void);

    debug_return_ptr!(&mut (*glitem).cache as *mut cache_item);
}
