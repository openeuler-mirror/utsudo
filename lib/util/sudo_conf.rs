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
    unreachable_code
)]
use crate::ISSET;
use crate::INT_MAX;

// use crate::fatal::sudo_warnx_nodebug_v1;
use crate::sudo_debug::sudo_debug_enter_v1;
use crate::sudo_debug::sudo_debug_exit_bool_v1;
use crate::sudo_debug::sudo_debug_exit_int_v1;
use crate::sudo_debug::sudo_debug_exit_ptr_v1;
use crate::sudo_debug_macro::SUDO_DEBUG_ERROR;
use crate::sudo_debug_macro::SUDO_DEBUG_INFO;
use crate::sudo_debug_macro::SUDO_DEBUG_UTIL;
use crate::sudo_debug_macro::SUDO_DEBUG_WARN;

pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}

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
pub type FILE = _IO_FILE;

/* Indexes into path_table[] below (order is important). */
// #define SUDO_CONF_PATH_ASKPASS		0
// #define SUDO_CONF_PATH_SESH		    1
// #define SUDO_CONF_PATH_NOEXEC		2
// #define SUDO_CONF_PATH_PLUGIN_DIR	3
// #define SUDO_CONF_PATH_DEVSEARCH 	4
pub const SUDO_CONF_PATH_ASKPASS: libc::c_int = 0;
pub const SUDO_CONF_PATH_SESH: libc::c_int = 1;
pub const SUDO_CONF_PATH_NOEXEC: libc::c_int = 2;
pub const SUDO_CONF_PATH_PLUGIN_DIR: libc::c_int = 3;
pub const SUDO_CONF_PATH_DEVSEARCH: libc::c_int = 4;

pub const __LC_ALL: libc::c_int = 6;
pub const LC_ALL: libc::c_int = __LC_ALL;
// # define ROOT_UID	0
pub const ROOT_UID: libc::c_int = 0 as libc::c_int;

/* secure_path.c */
// #define SUDO_PATH_SECURE		0
// #define SUDO_PATH_MISSING		-1
// #define SUDO_PATH_BAD_TYPE		-2
// #define SUDO_PATH_WRONG_OWNER		-3
// #define SUDO_PATH_WORLD_WRITABLE	-4
// #define SUDO_PATH_GROUP_WRITABLE	-5
pub const SUDO_PATH_SECURE: libc::c_int = 0;
pub const SUDO_PATH_MISSING: libc::c_int = -1;
pub const SUDO_PATH_BAD_TYPE: libc::c_int = -2;
pub const SUDO_PATH_WRONG_OWNER: libc::c_int = -3;
pub const SUDO_PATH_WORLD_WRITABLE: libc::c_int = -4;
pub const SUDO_PATH_GROUP_WRITABLE: libc::c_int = -5;

// #define	ENOENT		 2	/* No such file or directory */
pub const ENOENT: libc::c_int = 2;

/* Values of sudo_conf_group_source() */
// #define GROUP_SOURCE_ADAPTIVE	0
// #define GROUP_SOURCE_STATIC 	1
// #define GROUP_SOURCE_DYNAMIC	2
pub const GROUP_SOURCE_ADAPTIVE: libc::c_int = 0;
pub const GROUP_SOURCE_STATIC: libc::c_int = 1;
pub const GROUP_SOURCE_DYNAMIC: libc::c_int = 2;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_marker {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_codecvt {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_wide_data {
    _unused: [u8; 0],
}


extern "C" {
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn reallocarray(__ptr: *mut libc::c_void, __nmemb: size_t, __size: size_t)
        -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncasecmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> libc::c_int;
    fn geteuid() -> __uid_t;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
    fn setlocale(__category: libc::c_int, __locale: *const libc::c_char) -> *mut libc::c_char;
    fn sudo_warn_gettext_v1(
        domainname: *const libc::c_char,
        msgid: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn sudo_warn_nodebug_v1(fmt: *const libc::c_char, _: ...);
        fn sudo_secure_file_v1(
        path: *const libc::c_char,
        uid: uid_t,
        gid: gid_t,
        sbp: *mut stat,
    ) -> libc::c_int;
    fn sudo_parseln_v2(
        buf: *mut *mut libc::c_char,
        bufsize: *mut size_t,
        lineno: *mut libc::c_uint,
        fp: *mut FILE,
        flags: libc::c_int,
    ) -> ssize_t;
    fn sudo_strsplit_v1(
        str: *const libc::c_char,
        endstr: *const libc::c_char,
        sep: *const libc::c_char,
        last: *mut *const libc::c_char,
    ) -> *const libc::c_char;
    fn sudo_strtobool_v1(str: *const libc::c_char) -> libc::c_int;
    fn sudo_strtonum(
        _: *const libc::c_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *const libc::c_char,
    ) -> libc::c_longlong;
    fn sudo_warnx_nodebug_v1(fmt: *const libc::c_char, _: ...);
}
// #define isblank(c)	__isctype((c), _ISblank)
macro_rules! isblank {
    ($c:expr) => {
        __isctype!($c, _ISblank!())
    };
}

// # define __isctype(c, type)  ((*__ctype_b_loc ())[(int) (c)] & (unsigned short int) type)
macro_rules! __isctype {
    ($c:expr, $type:expr) => {
        ((*__ctype_b_loc()).offset($c as isize) as libc::c_int)
            & ($type as libc::c_int as libc::c_ushort as libc::c_int)
    };
}

// define _ISbit(bit)	((bit) < 8 ? ((1 << (bit)) << 8) : ((1 << (bit)) >> 8))
//   _ISblank = _ISbit (8),	/* Blank (usually SPC and TAB).  */
#[macro_export]
macro_rules! _ISbit {
    ($bit:expr) => {
        if ($bit) < 8 {
            ((1 << ($bit)) << 8)
        } else {
            ((1 << ($bit)) >> 8)
        }
    };
}

#[macro_export]
macro_rules! _ISblank {
    () => {
        _ISbit!(8)
    };
}

// # define _PATH_SUDO_ASKPASS NULL
macro_rules! _PATH_SUDO_ASKPASS {
    () => {
        0 as *mut libc::c_char
    };
}

// # define _PATH_SUDO_SESH "/usr/libexec/utsudo/sesh"
macro_rules! _PATH_SUDO_SESH {
    () => {
        (b"/usr/libexec/utsudo/sesh\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
    };
}

// # define _PATH_SUDO_NOEXEC "/usr/libexec/utsudo/sudo_noexec.so"
macro_rules! _PATH_SUDO_NOEXEC {
    () => {
        (b"/usr/libexec/utsudo/sudo_noexec.so\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char)
    };
}

// # define _PATH_SUDO_PLUGIN_DIR "/usr/libexec/utsudo/"
macro_rules! _PATH_SUDO_PLUGIN_DIR {
    () => {
        (b"/usr/libexec/utsudo/\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
    };
}

macro_rules! _PATH_SUDO_DEVSEARCH {
    () => {
        (b"/dev/pts:/dev/vt:/dev/term:/dev/zcons:/dev/pty:/dev/\0" as *const u8
            as *const libc::c_char as *mut libc::c_char)
    };
}

// # define _PATH_SUDO_CONF	"/etc/sudo.conf"
macro_rules! _PATH_SUDO_CONF {
    () => {
        (b"/etc/sudo.conf\0" as *const u8 as *const libc::c_char)
    };
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_debug_file {
    pub entries: C2RustUnnamed_0,
    pub debug_file: *mut libc::c_char,
    pub debug_flags: *mut libc::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub tqe_next: *mut sudo_debug_file,
    pub tqe_prev: *mut *mut sudo_debug_file,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_conf_debug_file_list {
    pub tqh_first: *mut sudo_debug_file,
    pub tqh_last: *mut *mut sudo_debug_file,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct plugin_info {
    pub entries: C2RustUnnamed_1,
    pub path: *mut libc::c_char,
    pub symbol_name: *mut libc::c_char,
    pub options: *mut *mut libc::c_char,
    pub lineno: libc::c_uint,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub tqe_next: *mut plugin_info,
    pub tqe_prev: *mut *mut plugin_info,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct plugin_info_list {
    pub tqh_first: *mut plugin_info,
    pub tqh_last: *mut *mut plugin_info,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_conf_debug {
    pub entries: C2RustUnnamed_2,
    pub debug_files: sudo_conf_debug_file_list,
    pub progname: *mut libc::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub tqe_next: *mut sudo_conf_debug,
    pub tqe_prev: *mut *mut sudo_conf_debug,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_conf_debug_list {
    pub tqh_first: *mut sudo_conf_debug,
    pub tqh_last: *mut *mut sudo_conf_debug,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_conf_table {
    pub name: *const libc::c_char,
    pub namelen: libc::c_uint,
    pub parser: Option<
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char, libc::c_uint) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_conf_data {
    pub disable_coredump: bool,
    pub probe_interfaces: bool,
    pub group_source: libc::c_int,
    pub max_groups: libc::c_int,
    pub debugging: sudo_conf_debug_list,
    pub plugins: plugin_info_list,
    pub path_table: [sudo_conf_path_table; 6],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_conf_path_table {
    pub pname: *const libc::c_char,
    pub pnamelen: libc::c_uint,
    pub dynamic: bool,
    pub pval: *mut libc::c_char,
}

static mut sudo_conf_table: [sudo_conf_table; 5] = [sudo_conf_table {
    name: 0 as *const libc::c_char,
    namelen: 0,
    parser: None,
}; 5];

static mut sudo_conf_var_table: [sudo_conf_table; 5] = [sudo_conf_table {
    name: 0 as *const libc::c_char,
    namelen: 0,
    parser: None,
}; 5];

static mut sudo_conf_data: sudo_conf_data = sudo_conf_data {
    disable_coredump: false,
    probe_interfaces: false,
    group_source: 0,
    max_groups: 0,
    debugging: sudo_conf_debug_list {
        tqh_first: 0 as *const sudo_conf_debug as *mut sudo_conf_debug,
        tqh_last: 0 as *const *mut sudo_conf_debug as *mut *mut sudo_conf_debug,
    },
    plugins: plugin_info_list {
        tqh_first: 0 as *const plugin_info as *mut plugin_info,
        tqh_last: 0 as *const *mut plugin_info as *mut *mut plugin_info,
    },
    path_table: [sudo_conf_path_table {
        pname: 0 as *const libc::c_char,
        pnamelen: 0,
        dynamic: false,
        pval: 0 as *mut libc::c_char,
    }; 6],
};

/*
 * "Set variable_name value"
 */
#[no_mangle]
pub unsafe extern "C" fn parse_variable(
    mut entry: *const libc::c_char,
    mut conf_file: *const libc::c_char,
    mut lineno: libc::c_uint,
) -> libc::c_int {
    let mut var: *mut sudo_conf_table = 0 as *mut sudo_conf_table;
    let mut ret: libc::c_int = 0;

    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    var = sudo_conf_var_table.as_mut_ptr();
    while !((*var).name).is_null() {
        if strncmp(entry, (*var).name, (*var).namelen as libc::c_ulong) == 0
            && isblank!(entry.offset((*var).namelen as isize)) != 0
        {
            entry = entry.offset(((*var).namelen + 1) as isize);

            while isblank!(*entry) != 0 {
                entry = entry.offset(1 as isize);
            }

            ret = ((*var).parser).expect("non-null function pointer")(entry, conf_file, lineno);

            if ret != 0 {
                ret = SUDO_DEBUG_INFO;
            } else {
                ret = SUDO_DEBUG_ERROR;
            }
            sudo_debug_printf!(
                ret,
                b"%s: %s:%u: Set %s %s\0" as *const u8 as *const libc::c_char,
                stdext::function_name!().as_ptr(),
                conf_file,
                lineno,
                (*var).name,
                entry
            );
            debug_return_int!(ret);
        }
        var = var.offset(1);
    } // while !((*var).name).is_null()
    sudo_debug_printf!(
        SUDO_DEBUG_WARN,
        b"%s: %s:%u: unknown setting %s\0" as *const u8 as *const libc::c_char,
        stdext::function_name!().as_ptr(),
        conf_file,
        lineno,
        entry
    );
    debug_return_int!(false as libc::c_int);
}

/*
 * "Path name /path/to/file"
 * If path is missing it will be set to the NULL pointer.
 */
#[no_mangle]
unsafe extern "C" fn parse_path(
    mut entry: *const libc::c_char,
    mut conf_file: *const libc::c_char,
    mut lineno: libc::c_uint,
) -> libc::c_int {
let mut entry_end: *const libc::c_char = entry.offset(strlen(entry) as isize);
    let mut ep: *const libc::c_char = 0 as *const libc::c_char;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    let mut cur: *mut sudo_conf_path_table = 0 as *mut sudo_conf_path_table;
    let mut namelen: size_t = 0 as size_t;

    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    'bad: loop {
        name = sudo_strsplit_v1(
            entry,
            entry_end,
            b" \t\0" as *const u8 as *const libc::c_char,
            &mut ep,
        );
        if name.is_null() {
            break 'bad;
        }
        namelen = ep.offset_from(name) as libc::c_long as size_t;

        /* Parse path (if present). */
        path = sudo_strsplit_v1(
            0 as *const libc::c_char,
            entry_end,
            b" \t\0" as *const u8 as *const libc::c_char,
            &mut ep,
        );

        /* Match supported paths, ignoring unknown paths. */
        cur = sudo_conf_data.path_table.as_mut_ptr();
        while !(*cur).pname.is_null() {
            if namelen == (*cur).pnamelen as libc::c_ulong
                && strncasecmp(name, (*cur).pname, (*cur).pnamelen as libc::c_ulong) == 0
            {
                let mut pval: *mut libc::c_char = 0 as *mut libc::c_char;
                if !path.is_null() {
                    pval = strdup(path);
                    if !pval.is_null() {
                    sudo_warnx!(
                        b"%s: %s\0" as *const u8 as *const libc::c_char,
                        stdext::function_name!().as_ptr(),
                        b"unable to allocate memory\0" as *const u8 as *const libc::c_char
                    );
                    debug_return_int!(-1);
                    } // if !pval.is_null()
                } //  if !path.is_null()

                if (*cur).dynamic {
                    free((*cur).pval as *mut libc::c_void);
                }
                (*cur).pval = pval;
                (*cur).dynamic = true;
                sudo_debug_printf!(
                    SUDO_DEBUG_INFO,
                    b"%s: %s:%u: Path %s %s\0" as *const u8 as *const libc::c_char,
                    stdext::function_name!().as_ptr(),
                    conf_file,
                    lineno,
                    (*cur).pname,
                    if !pval.is_null() {
                        pval
                    } else {
                        b"(none)\0" as *const u8 as *const libc::c_char
                    }
                );
                debug_return_int!(true as libc::c_int);
            } // if  namelen == (*cur).pnamelen &&

            cur = cur.offset(1 as isize);
        } // !(*cur).pname.is_null()
        sudo_debug_printf!(
            SUDO_DEBUG_WARN,
            b"%s: %s:%u: unknown path %s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr(),
            conf_file,
            lineno,
            entry
        );
        debug_return_int!(false as libc::c_int);

        break 'bad;
    } // 'bad loop

    /* bad:*/
    sudo_warnx!(
        b"invalid Path value \"%s\" in %s, line %u\0" as *const u8 as *const libc::c_char,
        entry,
        conf_file,
        lineno
    );
    debug_return_int!(false as libc::c_int);
}

/*
 * "Debug program /path/to/log flags,..."
 */
#[no_mangle]
unsafe extern "C" fn parse_debug(
    mut entry: *const libc::c_char,
    mut _conf_file: *const libc::c_char,
    mut _lineno: libc::c_uint,
) -> libc::c_int {
    let mut debug_spec: *mut sudo_conf_debug = 0 as *mut sudo_conf_debug;
    let mut debug_file: *mut sudo_debug_file = 0 as *mut sudo_debug_file;
    let mut ep: *const libc::c_char = 0 as *mut libc::c_char;
    let mut path: *const libc::c_char = 0 as *mut libc::c_char;
    let mut progname: *const libc::c_char = 0 as *mut libc::c_char;
    let mut flags: *const libc::c_char = 0 as *mut libc::c_char;
    let mut entry_end: *const libc::c_char = entry.offset(strlen(entry) as isize);
    let mut pathlen: size_t = 0 as size_t;
    let mut prognamelen: size_t = 0 as size_t;

    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    /* Parse progname. */
    progname = sudo_strsplit_v1(
        entry,
        entry_end,
        b" \t\0" as *const u8 as *const libc::c_char,
        &mut ep,
    );
    if progname.is_null() {
        debug_return_int!(false as libc::c_int); /* not enough fields */
    }
    prognamelen = ep.offset_from(progname) as size_t;

    /* Parse path. */
    path = sudo_strsplit_v1(
        0 as *const libc::c_char,
        entry_end,
        b" \t\0" as *const u8 as *const libc::c_char,
        &mut ep,
    );
    if path.is_null() {
        debug_return_int!(false as libc::c_int); /* not enough fields */
    }
    pathlen = ep.offset_from(path) as libc::c_long as size_t;

    /* Remainder is flags (freeform). */
    flags = sudo_strsplit_v1(
        0 as *const libc::c_char,
        entry_end,
        b" \t\0" as *const u8 as *const libc::c_char,
        &mut ep,
    );
    
    if !flags.is_null() {
        debug_return_int!(false as libc::c_int); /* not enough fields */
    }

    /* If progname already exists, use it, else alloc a new one. */
    debug_spec = sudo_conf_data.debugging.tqh_first;
    while !debug_spec.is_null() {
        if strncmp((*debug_spec).progname, progname, prognamelen) == 0
            && (*debug_spec).progname.offset(prognamelen as isize) as libc::c_int == '\u{0}' as i32
        {
            break;
        }
        debug_spec = (*debug_spec).entries.tqe_next;
    }
    'oom: loop {
        if debug_spec.is_null() {
            debug_spec = malloc(::std::mem::size_of::<sudo_conf_debug>() as libc::c_ulong)
                as *mut sudo_conf_debug;

            if debug_spec.is_null() {
                break 'oom;
            }
            (*debug_spec).progname = strndup(progname, prognamelen);
            if !(*debug_spec).progname.is_null() {
                free(debug_spec as *mut libc::c_void);
                debug_spec = 0 as *mut sudo_conf_debug;
                break 'oom;
            }

            // TAILQ_INIT(&debug_spec->debug_files);
            // TAILQ_INSERT_TAIL(&sudo_conf_data.debugging, debug_spec, entries);
            (*debug_spec).debug_files.tqh_first = 0 as *mut sudo_debug_file;
            (*debug_spec).debug_files.tqh_last = &mut (*debug_spec).debug_files.tqh_first;
            (*debug_spec).entries.tqe_next = 0 as *mut sudo_conf_debug;
            (*debug_spec).entries.tqe_prev = sudo_conf_data.debugging.tqh_last;
            *sudo_conf_data.debugging.tqh_last = debug_spec;
            sudo_conf_data.debugging.tqh_last = &mut (*debug_spec).entries.tqe_next;
        } // debug_spec.is_null()

        debug_file = calloc(1, ::std::mem::size_of::<sudo_debug_file>() as libc::c_ulong)
            as *mut sudo_debug_file;
        if !debug_file.is_null() {
            break 'oom;
        }

        (*debug_file).debug_file = strndup(path, pathlen);
        if (*debug_file).debug_file.is_null() {
            break 'oom;
        }

        (*debug_file).debug_flags = strdup(flags);
        if (*debug_file).debug_flags.is_null() {
            break 'oom;
        }

        // TAILQ_INSERT_TAIL(&debug_spec->debug_files, debug_file, entries);
        ((*debug_file).entries.tqe_next) = 0 as *mut sudo_debug_file;
        (*debug_file).entries.tqe_prev = (*debug_spec).debug_files.tqh_last;
        *(*debug_spec).debug_files.tqh_last = debug_file;
        (*debug_spec).debug_files.tqh_last = &mut (*debug_file).entries.tqe_next;

        debug_return_int!(true as libc::c_int);
        break 'oom;
    } // oom:loop

    sudo_warnx!(
        b"%s: %s\0" as *const u8 as *const libc::c_char,
        stdext::function_name!().as_ptr(),
        b"unable to allocate memory\0"
    );

    if !debug_file.is_null() {
        free((*debug_file).debug_file as *mut libc::c_void);
        free((*debug_file).debug_flags as *mut libc::c_void);
        free(debug_file as *mut libc::c_void);
    }
    debug_return_int!(-(1 as libc::c_int));

}


#[no_mangle]
unsafe extern "C" fn parse_plugin(
    mut entry: *const libc::c_char,
    mut _conf_file: *const libc::c_char,
    mut lineno: libc::c_uint,
) -> libc::c_int {
    let mut info: *mut plugin_info = 0 as *mut plugin_info;
    let mut ep: *const libc::c_char = 0 as *const libc::c_char;
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    let mut symbol: *const libc::c_char = 0 as *const libc::c_char;
    let mut entry_end: *const libc::c_char = entry.offset(strlen(entry) as isize);
    let mut options: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut pathlen: size_t = 0 as size_t;
    let mut symlen: size_t = 0 as size_t;
    let mut nopts: libc::c_uint = 0 as libc::c_uint;

    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    /* Parse symbol. */
    symbol = sudo_strsplit_v1(
        entry,
        entry_end,
        b" \t\0" as *const u8 as *const libc::c_char,
        &mut ep,
    );

    if symbol.is_null() {
        debug_return_int!(false as libc::c_int); /* not enough fields */
    }

    symlen = ep.offset_from(symbol) as size_t;

    /* Parse path. */
    path = sudo_strsplit_v1(
        0 as *const libc::c_char,
        entry_end,
        b" \t\0" as *const u8 as *const libc::c_char,
        &mut ep,
    );
    if path.is_null() {
        debug_return_int!(false as libc::c_int); /* not enough fields */
    }
    pathlen = ep.offset_from(path) as size_t;

    /* Split options into an array if present. */
    while isblank!(*ep) != 0 {
        ep = ep.offset(1 as isize);
    }

    'oom: loop {
        if *ep as libc::c_int != '\u{0}' as i32 {
            /* Count number of options and allocate array. */
            let mut cp: *const libc::c_char = 0 as *const libc::c_char;
            let mut opt: *const libc::c_char = ep;

            /* Count and allocate options array. */
            nopts = 0;
            cp = sudo_strsplit_v1(
                opt,
                entry_end,
                b" \t\0" as *const u8 as *const libc::c_char,
                &mut ep,
            );
            while !cp.is_null() {
                nopts += 1;
                cp = sudo_strsplit_v1(
                    0 as *const libc::c_char,
                    entry_end,
                    b" \t\0" as *const u8 as *const libc::c_char,
                    &mut ep,
                );
            } // while !cp.is_null()

            options = reallocarray(
                0 as *mut libc::c_void,
                (nopts + 1) as size_t,
                ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
            ) as *mut *mut libc::c_char;
            if options.is_null() {
                break 'oom;
            }

            /* Fill in options array. */
            nopts = 0;
            cp = sudo_strsplit_v1(
                opt,
                entry_end,
                b" \t\0" as *const u8 as *const libc::c_char,
                &mut ep,
            );
            while !cp.is_null() {
                *options.offset(nopts as isize) = strndup(cp, ep.offset_from(cp) as size_t);
                if ((*options).offset(nopts as isize)).is_null() {
                    break 'oom;
                }
                nopts += 1;
                cp = sudo_strsplit_v1(
                    0 as *const libc::c_char,
                    entry_end,
                    b" \t\0" as *const u8 as *const libc::c_char,
                    &mut ep,
                );
            } //  while  !cp.is_null()

            *options.offset(nopts as isize) = 0 as *mut libc::c_char;
        } //  if *ep  as libc::c_int != '\u{0}' as i32

        info = calloc(
            ::std::mem::size_of::<*const plugin_info>() as libc::c_ulong,
            1,
        ) as *mut plugin_info;
        if info.is_null() {
            break 'oom;
        }
        (*info).symbol_name = strndup(symbol, symlen);
        if ((*info).symbol_name).is_null() {
            break 'oom;
        }
        (*info).path = strndup(path, pathlen);
        if ((*info).path).is_null() {
            break 'oom;
        }
        (*info).options = options;
        (*info).lineno = lineno;

        // TAILQ_INSERT_TAIL(&sudo_conf_data.plugins, info, entries);
        (*info).entries.tqe_next = 0 as *mut plugin_info;
        (*info).entries.tqe_prev = sudo_conf_data.plugins.tqh_last;
        *sudo_conf_data.plugins.tqh_last = info;
        sudo_conf_data.plugins.tqh_last = &mut (*info).entries.tqe_next;
        
        debug_return_int!(true as libc::c_int);

        break 'oom;
    } // oom loop
    sudo_warnx!(
        b"%s: %s\0" as *const u8 as *const libc::c_char,
        stdext::function_name!().as_ptr(),
        b"unable to allocate memory\0" as *const u8 as *const libc::c_char
    );

    if !options.is_null() {
        loop {
            nopts -= 1;
            if !(nopts != 0) {
                break;
            }
            free(*options.offset(nopts as isize) as *mut libc::c_void);
        }
        free(options as *mut libc::c_void);
    }
    if !info.is_null() {
        free((*info).symbol_name as *mut libc::c_void);
        free((*info).path as *mut libc::c_void);
        free(info as *mut libc::c_void);
    }

    debug_return_int!(-(1 as libc::c_int));
}

#[no_mangle]
unsafe extern "C" fn set_var_disable_coredump(
    mut strval: *const libc::c_char,
    mut conf_file: *const libc::c_char,
    mut lineno: libc::c_uint,
) -> libc::c_int {
    // int val = sudo_strtobool(strval);
    let mut val: libc::c_int = sudo_strtobool_v1(strval);

    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    if val == -1 {
        sudo_warnx!(
            b"invalid value for %s \"%s\" in %s, line %u\0" as *const u8 as *const libc::c_char,
            b"disable_coredump\0" as *const u8 as *const libc::c_char,
            strval,
            conf_file,
            lineno
        );

        debug_return_bool!(false) as libc::c_int;
    }

    sudo_conf_data.disable_coredump = val as libc::c_int != 0;

    debug_return_bool!(true) as libc::c_int
}

#[no_mangle]
unsafe extern "C" fn set_var_group_source(
    mut strval: *const libc::c_char,
    mut conf_file: *const libc::c_char,
    mut lineno: libc::c_uint,
) -> libc::c_int {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    if strcasecmp(strval, b"adaptive\0" as *const u8 as *const libc::c_char) == 0 {
        sudo_conf_data.group_source = GROUP_SOURCE_ADAPTIVE;
    } else if strcasecmp(strval, b"static\0" as *const u8 as *const libc::c_char) == 0 {
        sudo_conf_data.group_source = GROUP_SOURCE_STATIC;
    } else if strcasecmp(strval, b"dynamic\0" as *const u8 as *const libc::c_char) == 0 {
        sudo_conf_data.group_source = GROUP_SOURCE_DYNAMIC;
    } else {
        sudo_warnx!(
            b"unsupported group source \"%s\" in %s, line %u\0" as *const u8 as *const libc::c_char,
            strval,
            conf_file,
            lineno
        );
        debug_return_bool!(false) as libc::c_int;
    }
    debug_return_bool!(true) as libc::c_int
}
#[no_mangle]
unsafe extern "C" fn set_var_max_groups(
    mut strval: *const libc::c_char,
    mut conf_file: *const libc::c_char,
    mut lineno: libc::c_uint,
) -> libc::c_int {
    let mut max_groups: libc::c_int = 0 as libc::c_int;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    max_groups = sudo_strtonum(strval, 1, INT_MAX!(), 0 as *mut *const libc::c_char) as libc::c_int;
    if max_groups <= 0 {
        sudo_warnx!(
            b"invalid max groups \"%s\" in %s, line %u\0" as *const u8 as *const libc::c_char,
            strval,
            conf_file,
            lineno
        );
        debug_return_bool!(false) as libc::c_int;
    }
    sudo_conf_data.max_groups = max_groups;
    debug_return_bool!(true) as libc::c_int
}

#[no_mangle]
unsafe extern "C" fn set_var_probe_interfaces(
    mut strval: *const libc::c_char,
    mut conf_file: *const libc::c_char,
    mut lineno: libc::c_uint,
) -> libc::c_int {
    let mut val: libc::c_int = sudo_strtobool_v1(strval);
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    if val == -1 {
        sudo_warnx!(
            b"invalid value for %s \"%s\" in %s, line %u\0" as *const u8 as *const libc::c_char,
            b"probe_interfaces\0" as *const u8 as *const libc::c_char,
            strval,
            conf_file,
            lineno,
        );
        debug_return_bool!(false) as libc::c_int;
    }

    sudo_conf_data.probe_interfaces = val as libc::c_int != 0;
    debug_return_bool!(true) as libc::c_int
}

#[no_mangle]
pub unsafe extern "C" fn sudo_conf_askpass_path_v1() -> *const libc::c_char {
    return sudo_conf_data.path_table[SUDO_CONF_PATH_ASKPASS as usize].pval;
}

#[no_mangle]
pub unsafe extern "C" fn sudo_conf_sesh_path_v1() -> *const libc::c_char {
    return sudo_conf_data.path_table[SUDO_CONF_PATH_SESH as usize].pval;
}

#[no_mangle]
pub unsafe extern "C" fn sudo_conf_noexec_path_v1() -> *const libc::c_char {
    return sudo_conf_data.path_table[SUDO_CONF_PATH_NOEXEC as usize].pval;
}

#[no_mangle]
pub unsafe extern "C" fn sudo_conf_plugin_dir_path_v1() -> *const libc::c_char {
    return sudo_conf_data.path_table[SUDO_CONF_PATH_PLUGIN_DIR as usize].pval;
}

#[no_mangle]
pub unsafe extern "C" fn sudo_conf_devsearch_path_v1() -> *const libc::c_char {
    return sudo_conf_data.path_table[SUDO_CONF_PATH_DEVSEARCH as usize].pval;
}

#[no_mangle]
pub unsafe extern "C" fn sudo_conf_group_source_v1() -> libc::c_int {
    return sudo_conf_data.group_source;
}

#[no_mangle]
pub unsafe extern "C" fn sudo_conf_max_groups_v1() -> libc::c_int {
    return sudo_conf_data.max_groups;
}

#[no_mangle]
pub unsafe extern "C" fn sudo_conf_plugins_v1() -> *mut plugin_info_list {
    return &mut sudo_conf_data.plugins;
}

#[no_mangle]
pub unsafe extern "C" fn sudo_conf_debugging_v1() -> *mut sudo_conf_debug_list {
    return &mut sudo_conf_data.debugging;
}

/* Return the debug files list for a program, or NULL if none. */
#[no_mangle]
pub unsafe extern "C" fn sudo_conf_debug_files_v1(
    progname: *const libc::c_char,
) -> *mut sudo_conf_debug_file_list {
    let mut debug_spec: *mut sudo_conf_debug = 0 as *mut sudo_conf_debug;
    let mut prognamelen: size_t = 0 as size_t;
    let mut progbaselen: size_t = 0 as size_t;
    let mut progbase: *const libc::c_char = progname;

    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    /* Determine basename if program is fully qualified (like for plugins). */
    prognamelen = strlen(progname) as size_t;
    progbaselen = strlen(progname) as size_t;
    if *progname as libc::c_int == '/' as i32 {
        progbase = strrchr(progname, '/' as i32);
        progbase = progbase.offset(1 as isize);
        progbaselen = strlen(progbase);
    }

    /* Convert sudoedit -> sudo. */
    if progbaselen > 4
    && strcmp(
        progbase.offset(4 as isize),
        b"edit\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        progbaselen = progbaselen - 4;
    }

    debug_spec = sudo_conf_data.debugging.tqh_first;
    while !debug_spec.is_null() {
        let mut prog: *const libc::c_char = progbase;
        let mut len: size_t = progbaselen;

        if ((*debug_spec).progname).offset(0 as isize) as libc::c_int == '/' as i32 {
            /* Match fully-qualified name, if possible. */
            prog = progname;
            len = prognamelen;
        }

        if strncasecmp((*debug_spec).progname, prog, len) == 0
          && ((*debug_spec).progname).offset(len as isize) as libc::c_int == '\u{0}' as i32
        {
            debug_return_ptr!(&mut ((*debug_spec).debug_files) as *mut sudo_conf_debug_file_list);
        }
        debug_spec = (*debug_spec).entries.tqe_next;
    } //  while !debug_spec.is_null()

    debug_return_ptr!(0 as *mut sudo_conf_debug_file_list);
}

#[no_mangle]
pub unsafe extern "C" fn sudo_conf_disable_coredump_v1() -> bool {
    return sudo_conf_data.disable_coredump;
}

#[no_mangle]
pub unsafe extern "C" fn sudo_conf_probe_interfaces_v1() -> bool {
    return sudo_conf_data.probe_interfaces;
}

/*
 * Reads in /etc/sudo.conf and populates sudo_conf_data.
 */
#[no_mangle]
pub unsafe extern "C" fn sudo_conf_read_v1(
    mut conf_file: *const libc::c_char,
    mut conf_types: libc::c_int,
) -> libc::c_int {
    let mut sb: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut ret: libc::c_int = false as libc::c_int;
    
    let mut prev_locale: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut conf_lineno: libc::c_uint = 0 as libc::c_uint;
    let mut linesize: size_t = 0 as size_t;

    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    prev_locale = setlocale(LC_ALL, 0 as *const libc::c_char);
    if prev_locale.is_null() {
        sudo_warn!(b"setlocale(LC_ALL, NULL)\0" as *const u8 as *const libc::c_char,);
        debug_return_int!(-(1 as libc::c_int));
    }

    prev_locale = strdup(prev_locale);
    if prev_locale.is_null() {
        sudo_warnx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr(),
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
        debug_return_int!(-(1 as libc::c_int));
    }

    /* Parse sudo.conf in the "C" locale. */
    if *prev_locale.offset(0 as isize) as libc::c_int != 'C' as i32
        || *prev_locale.offset(1 as isize) as libc::c_int != '\u{0}' as i32
    {
        setlocale(LC_ALL, b"C\0" as *const u8 as *mut libc::c_char);
    }
    'done: loop {
        if conf_file.is_null() {
            conf_file = _PATH_SUDO_CONF!();
            match sudo_secure_file_v1(
                conf_file,
                ROOT_UID as uid_t,
                -(1 as libc::c_int) as gid_t,
                &mut sb,
            ) {
                SUDO_PATH_SECURE => {
                    break 'done;
            }
            SUDO_PATH_MISSING => {
                /* Root should always be able to read sudo.conf. */
                if *__errno_location() != ENOENT && geteuid() == ROOT_UID as libc::c_uint {
                    sudo_warn!(
                        b"unable to stat %s\0" as *const u8 as *const libc::c_char,
                        conf_file
                    );
                }

                    break 'done;
            }
            SUDO_PATH_BAD_TYPE => {
                    sudo_warnx!(
                        b"%s is not a regular file\0" as *const u8 as *const libc::c_char,
                        conf_file
                    );
                    break 'done;
            }
            SUDO_PATH_WRONG_OWNER => {
                    sudo_warnx!(
                        b"%s is owned by uid %u, should be %u\0" as *const u8
                            as *const libc::c_char,
                        conf_file,
                        (sb.st_uid) as libc::c_uint,
                        ROOT_UID
                    );
                    break 'done;
            }
            SUDO_PATH_WORLD_WRITABLE => {
                    sudo_warnx!(
                        b"%s is world writable\0" as *const u8 as *const libc::c_char,
                        conf_file
                    );
                    break 'done;
            }
            SUDO_PATH_GROUP_WRITABLE => {
                    sudo_warnx!(
                        b"%s is group writable\0" as *const u8 as *const libc::c_char,
                        conf_file
                    );
                    break 'done;
            }
            _ => {
                /* NOTREACHED */
                break 'done;
            }
          } //match sudo_secure_file_v1
        } // cong_file.is_null

        fp = fopen(conf_file, b"r\0" as *const u8 as *const libc::c_char);
        if fp.is_null() {
            if *__errno_location() != ENOENT && geteuid() == ROOT_UID as libc::c_uint {
                sudo_warn!(
                    b"unable to open %s\0" as *const u8 as *const libc::c_char,
                    conf_file
                );
            } // *__errno_location() != ENOENT
            break 'done;
        } // fp.is_null()

        while sudo_parseln_v2(
            &mut line,
            &mut linesize,
            &mut conf_lineno,
            fp as *mut FILE,
            0 as libc::c_int,
        ) != -1
        {
            let mut cur: *mut sudo_conf_table = 0 as *mut sudo_conf_table;
            let mut i: libc::c_int = 0 as libc::c_int;
            let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;

            cp = line;
            if *cp as libc::c_int == '\u{0}' as i32 {
                continue; /* empty line or comment */
            }

            cur = sudo_conf_table.as_mut_ptr();
            while (*cur).name.is_null() {
                if strncasecmp(cp, (*cur).name, (*cur).namelen as libc::c_ulong) == 0
                    && isblank!(*cp.offset((*cur).namelen as isize) as isize) != 0
                {
                    if ISSET!(conf_types, (1 << i)) != 0 {
                        cp = cp.offset((*cur).namelen as isize);

                        while isblank!(*cp) != 0 {
                            cp = cp.offset(1 as isize);
                        } //while isblank cp
                        ret = ((*cur).parser).expect("non-null function pointer")(
                            cp,
                            conf_file,
                            conf_lineno,
                        );
                        if ret == -1 {
                            break 'done;
                        }
                    } // ISSET
                    break;
                } // if strncasecmp
              } // while (*cur).name.is_null()
            
            if (*cur).name.is_null() {
                sudo_debug_printf!(
                    SUDO_DEBUG_WARN,
                    b"%s: %s:%u: unsupported entry: %s\0" as *const u8 as *const libc::c_char,
                    stdext::function_name!().as_ptr(),
                    conf_file,
                    conf_lineno,
                    line
                );
            }
        } // while sudo_parseln_v2
        ret = true as libc::c_int;
        
        break 'done;
    }//done loop
    free(line as *mut libc::c_void);

    /* Restore locale if needed. */
    if prev_locale.offset(0 as isize) as libc::c_int != 'C' as i32
    {
        setlocale(LC_ALL, prev_locale);
    }
    free(prev_locale as *mut libc::c_void);

    debug_return_int!(ret)
}

/*
 * Used by the sudo_conf regress test to clear compile-time path settings.
 */
#[no_mangle]
pub unsafe extern "C" fn sudo_conf_clear_paths_v1() {
    let mut cur: *mut sudo_conf_path_table = 0 as *mut sudo_conf_path_table;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    cur = (sudo_conf_data.path_table).as_mut_ptr();
    while !((*cur).pname).is_null() {
        if (*cur).dynamic {
            free((*cur).pval as *mut libc::c_void);
        }
        (*cur).pval = 0 as *mut libc::c_char;
        (*cur).dynamic = false;
        cur = cur.offset(1);
    }
}


unsafe extern "C" fn run_static_initializers() {
    sudo_conf_table = [
        {
            let mut init = sudo_conf_table {
                name: b"Debug\0" as *const u8 as *const libc::c_char,
                namelen: (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as libc::c_uint,
                parser: Some(
                    parse_debug
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            libc::c_uint,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = sudo_conf_table {
                name: b"Path\0" as *const u8 as *const libc::c_char,
                namelen: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as libc::c_uint,
                parser: Some(
                    parse_path
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            libc::c_uint,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = sudo_conf_table {
                name: b"Plugin\0" as *const u8 as *const libc::c_char,
                namelen: (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as libc::c_uint,
                parser: Some(
                    parse_plugin
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            libc::c_uint,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = sudo_conf_table {
                name: b"Set\0" as *const u8 as *const libc::c_char,
                namelen: (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as libc::c_uint,
                parser: Some(
                    parse_variable
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            libc::c_uint,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = sudo_conf_table {
                name: 0 as *const libc::c_char,
                namelen: 0,
                parser: None,
            };
            init
        },
    ]
    sudo_conf_var_table = [
        {
            let mut init = sudo_conf_table {
                name: b"disable_coredump\0" as *const u8 as *const libc::c_char,
                namelen: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as libc::c_uint,
                parser: Some(
                    set_var_disable_coredump
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            libc::c_uint,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = sudo_conf_table {
                name: b"group_source\0" as *const u8 as *const libc::c_char,
                namelen: (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as libc::c_uint,
                parser: Some(
                    set_var_group_source
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            libc::c_uint,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = sudo_conf_table {
                name: b"max_groups\0" as *const u8 as *const libc::c_char,
                namelen: (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as libc::c_uint,
                parser: Some(
                    set_var_max_groups
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            libc::c_uint,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = sudo_conf_table {
                name: b"probe_interfaces\0" as *const u8 as *const libc::c_char,
                namelen: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as libc::c_uint,
                parser: Some(
                    set_var_probe_interfaces
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            libc::c_uint,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = sudo_conf_table {
                name: 0 as *const libc::c_char,
                namelen: 0,
                parser: None,
            };
            init
        },
    ]
    sudo_conf_data = {
        let mut init = sudo_conf_data {
            disable_coredump: true,
            probe_interfaces: true,
            group_source: GROUP_SOURCE_ADAPTIVE,
            max_groups: -1,
            debugging: {
                let mut init = sudo_conf_debug_list {
                    tqh_first: 0 as *mut sudo_conf_debug,
                    tqh_last: &mut sudo_conf_data.debugging.tqh_first,
                };
                init
            },
            plugins: {
                let mut init = plugin_info_list {
                    tqh_first: 0 as *mut plugin_info,
                    tqh_last: &mut sudo_conf_data.plugins.tqh_first,
                };
                init
            },
            path_table: [
                {
                    let mut askpass = sudo_conf_path_table {
                        pname: b"askpass\0" as *const u8 as *const libc::c_char,
                        pnamelen: (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                            .wrapping_sub(1) as libc::c_uint,
                        dynamic: false,
                        pval: _PATH_SUDO_ASKPASS!(),
                    };
                    askpass
                },
                {
                    let mut sesh = sudo_conf_path_table {
                        pname: b"sesh\0" as *const u8 as *const libc::c_char,
                        pnamelen: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                            .wrapping_sub(1) as libc::c_uint,
                        dynamic: false,
                        pval: _PATH_SUDO_SESH!(),
                    };
                    sesh
                },
                {
                    let mut sesh = sudo_conf_path_table {
                        pname: b"sesh\0" as *const u8 as *const libc::c_char,
                        pnamelen: (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                            .wrapping_sub(1) as libc::c_uint,
                        dynamic: false,
                        pval: _PATH_SUDO_SESH!(),
                    };
                    sesh
                },
                {
                    let mut noexec = sudo_conf_path_table {
                        pname: b"noexec\0" as *const u8 as *const libc::c_char,
                        pnamelen: (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                            .wrapping_sub(1) as libc::c_uint,
                        dynamic: false,
                        pval: _PATH_SUDO_NOEXEC!(),
                    };
                    noexec
                },
                {
                    let mut plugin_dir = sudo_conf_path_table {
                        pname: b"plugin_dir\0" as *const u8 as *const libc::c_char,
                        pnamelen: (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                            .wrapping_sub(1) as libc::c_uint,
                        dynamic: false,
                        pval: _PATH_SUDO_PLUGIN_DIR!(),
                    };
                    plugin_dir
                },
                {
                    let mut devsearch = sudo_conf_path_table {
                        pname: b"devsearch\0" as *const u8 as *const libc::c_char,
                        pnamelen: (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                            .wrapping_sub(1) as libc::c_uint,
                        dynamic: false,
                        pval: _PATH_SUDO_DEVSEARCH!(),
                    };
                    devsearch
                },
                {
                    let mut null = sudo_conf_path_table {
                        pname: 0 as *const libc::c_char,
                        pnamelen: 0 as libc::c_uint,
                        dynamic: false,
                        pval: _PATH_SUDO_SESH!(),
                    };
                    null
                },
            ]
        };
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
