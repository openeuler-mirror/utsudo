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

use crate::sudo_debug_macro::SUDO_DEBUG_ERROR;
use crate::sudo_debug_macro::SUDO_DEBUG_INFO;

extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    fn strncasecmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> libc::c_int;
    fn sudo_strsplit_v1(
        str: *const libc::c_char,
        endstr: *const libc::c_char,
        sep: *const libc::c_char,
        last: *mut *const libc::c_char,
    ) -> *const libc::c_char;
    fn sudo_strtobool_v1(str: *const libc::c_char) -> libc::c_int;
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

                        debug_return_int!(-1);
                    } // if !pval.is_null()
                } //  if !path.is_null()

                if (*cur).dynamic {
                    free((*cur).pval as *mut libc::c_void);
                }
                (*cur).pval = pval;
                (*cur).dynamic = true;

                debug_return_int!(true as libc::c_int);
            } // if  namelen == (*cur).pnamelen &&

            cur = cur.offset(1 as isize);
        } // !(*cur).pname.is_null()

        debug_return_int!(false as libc::c_int);

        break 'bad;
    } // 'bad loop

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
    prognamelen = ep.offset_from(progname) as size_t;

    /* Parse path. */
    path = sudo_strsplit_v1(
        0 as *const libc::c_char,
        entry_end,
        b" \t\0" as *const u8 as *const libc::c_char,
        &mut ep,
    );
    pathlen = ep.offset_from(path) as libc::c_long as size_t;

    /* Remainder is flags (freeform). */
    flags = sudo_strsplit_v1(
        0 as *const libc::c_char,
        entry_end,
        b" \t\0" as *const u8 as *const libc::c_char,
        &mut ep,
    );

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
    let mut pathlen: size_t = 0 as size_t;
    let mut symlen: size_t = 0 as size_t;

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
