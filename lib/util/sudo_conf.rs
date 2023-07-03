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

use crate::fatal::sudo_warnx_nodebug_v1;
use crate::sudo_debug::sudo_debug_enter_v1;
use crate::sudo_debug::sudo_debug_exit_bool_v1;
use crate::sudo_debug::sudo_debug_exit_int_v1;
use crate::sudo_debug::sudo_debug_exit_ptr_v1;
use crate::sudo_debug_macro::SUDO_DEBUG_ERROR;
use crate::sudo_debug_macro::SUDO_DEBUG_INFO;
use crate::sudo_debug_macro::SUDO_DEBUG_UTIL;
use crate::sudo_debug_macro::SUDO_DEBUG_WARN;

/* Indexes into path_table[] below (order is important). */
#define SUDO_CONF_PATH_ASKPASS		0
#define SUDO_CONF_PATH_SESH		    1
#define SUDO_CONF_PATH_NOEXEC		2

/* Values of sudo_conf_group_source() */
#define GROUP_SOURCE_ADAPTIVE	0
#define GROUP_SOURCE_STATIC 	1
#define GROUP_SOURCE_DYNAMIC	2

extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn reallocarray(__ptr: *mut libc::c_void, __nmemb: size_t, __size: size_t)
        -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncasecmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> libc::c_int;
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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_conf_debug {
    pub entries: C2RustUnnamed_2,
    pub debug_files: sudo_conf_debug_file_list,
    pub progname: *mut libc::c_char,
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

            TAILQ_INIT(&debug_spec->debug_files);
            TAILQ_INSERT_TAIL(&sudo_conf_data.debugging, debug_spec, entries);
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

        TAILQ_INSERT_TAIL(&debug_spec->debug_files, debug_file, entries);

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

        TAILQ_INSERT_TAIL(&sudo_conf_data.plugins, info, entries);

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
        },]
}
