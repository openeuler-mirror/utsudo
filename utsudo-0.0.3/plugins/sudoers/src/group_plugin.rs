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
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn reallocarray(__ptr: *mut libc::c_void, __nmemb: size_t, __size: size_t)
        -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
    fn sudo_warn_gettext_v1(
        domainname: *const libc::c_char,
        msgid: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn sudo_warn_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_warnx_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_debug_enter_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        subsys: libc::c_int,
    );
    fn sudo_debug_exit_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        subsys: libc::c_int,
    );
    fn sudo_debug_exit_bool_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        subsys: libc::c_int,
        ret: bool,
    );
    fn sudo_debug_exit_int_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        subsys: libc::c_int,
        ret: libc::c_int,
    );
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    static mut sudoers_subsystem_ids: [libc::c_uint; 0];
    static mut sudo_printf:
        Option<unsafe extern "C" fn(libc::c_int, *const libc::c_char, ...) -> libc::c_int>;
    fn sudo_dso_strerror_v1() -> *mut libc::c_char;
    fn sudo_dso_unload_v1(handle: *mut libc::c_void) -> libc::c_int;
    fn sudo_dso_findsym_v1(
        handle: *mut libc::c_void,
        symbol: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn sudo_dso_load_v1(path: *const libc::c_char, mode: libc::c_int) -> *mut libc::c_void;
}

// 多处定义，后期统一处理
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;

#[macro_export]
macro_rules! _PATH_SUDO_PLUGIN_DIR {
    () => {
        b"/usr/libexec/utsudo/\0" as *const u8 as *const libc::c_char
    };
}

pub type sudo_printf_t =
    Option<unsafe extern "C" fn(libc::c_int, *const libc::c_char, ...) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudoers_group_plugin {
    pub version: libc::c_uint,
    pub init: Option<
        unsafe extern "C" fn(libc::c_int, sudo_printf_t, *const *mut libc::c_char) -> libc::c_int,
    >,
    pub cleanup: Option<unsafe extern "C" fn() -> ()>,
    pub query: Option<
        unsafe extern "C" fn(
            *const libc::c_char,
            *const libc::c_char,
            *const passwd,
        ) -> libc::c_int,
    >,
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(xstat_flag as libc::c_int, __path, __statbuf);
}
static mut group_handle: *mut libc::c_void = 0 as *mut libc::c_void;
static mut group_plugin: *mut sudoers_group_plugin = 0 as *mut sudoers_group_plugin;

#[no_mangle]
pub static mut path_plugin_dir: *const libc::c_char = _PATH_SUDO_PLUGIN_DIR!();

/*
 * Load the specified plugin and run its init function.
 * Returns -1 if unable to open the plugin, else it returns
 * the value from the plugin's init function.
 */

#[no_mangle]
pub unsafe extern "C" fn group_plugin_load(mut plugin_info: *mut libc::c_char) -> libc::c_int {
    let mut sb: stat = sb_all_arch;
    let mut args: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path: [libc::c_char; 4096] = [0; 4096];
    let mut argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut rc: libc::c_int = -(1 as libc::c_int);
    debug_decl!(SUDOERS_DEBUG_UTIL!());

    /*
     * Fill in .so path and split out args (if any).
     */
    args = strpbrk(plugin_info, b" \t\0" as *const u8 as *const libc::c_char);
    if !args.is_null() {
        len = snprintf(
            path.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
            b"%s%.*s\0" as *const u8 as *const libc::c_char,
            if *plugin_info as libc::c_int != '/' as i32 {
                path_plugin_dir
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            args.offset_from(plugin_info) as libc::c_long as libc::c_int,
            plugin_info,
        );
        args = args.offset(1);
    } else {
        len = snprintf(
            path.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
            b"%s%s\0" as *const u8 as *const libc::c_char,
            if *plugin_info as libc::c_int != '/' as i32 {
                path_plugin_dir
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            plugin_info,
        );
    }

    'done: loop {
        if len < 0 as libc::c_int
            || len as libc::c_long
                >= ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as ssize_t
        {
            *__errno_location() = ENAMETOOLONG;
            sudo_warn!(
                b"%s%s\0" as *const u8 as *const libc::c_char,
                if *plugin_info as libc::c_int != '/' as i32 {
                    path_plugin_dir
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                plugin_info,
            );
            break 'done;
        }

        /* Sanity check plugin path. */
        if stat(path.as_mut_ptr(), &mut sb) != 0 {
            sudo_warn!(b"%s" as *const u8 as *const libc::c_char, path.as_mut_ptr());
            break 'done;
        }

        if sb.st_uid != ROOT_UID as libc::c_uint {
            sudo_warnx!(
                b"%s must be owned by uid %d" as *const u8 as *const libc::c_char,
                path.as_mut_ptr(),
                ROOT_UID
            );
            break 'done;
        }

        if sb.st_mode & (S_IWGRP!() | S_IWOTH!()) != 0 {
            sudo_warnx!(
                b"%s must only be writable by owner" as *const u8 as *const libc::c_char,
                path.as_mut_ptr()
            );
            break 'done;
        }

        /* Open plugin and map in symbol. */
        group_handle = sudo_dso_load_v1(path.as_mut_ptr(), SUDO_DSO_LAZY!() | SUDO_DSO_GLOBAL!());
        if group_handle.is_null() {
            let mut errstr: *const libc::c_char = sudo_dso_strerror_v1();
            sudo_warnx!(
                b"unable to load %s: %s" as *const u8 as *const libc::c_char,
                path.as_mut_ptr(),
                if !errstr.is_null() {
                    errstr
                } else {
                    b"unknown error\0" as *const u8 as *const libc::c_char
                }
            );

            break 'done;
        }

        group_plugin = sudo_dso_findsym_v1(
            group_handle,
            b"group_plugin" as *const u8 as *const libc::c_char,
        ) as *mut sudoers_group_plugin;
        if group_plugin.is_null() {
            sudo_warnx!(
                b"unable to find symbol \"group_plugin\" in %s\0" as *const u8
                    as *const libc::c_char,
                path.as_mut_ptr()
            );
            break 'done;
        }

        if SUDO_API_VERSION_GET_MAJOR!((*group_plugin).version)
            != GROUP_API_VERSION_MAJOR as libc::c_uint
        {
            sudo_warnx!(
                b"%s: incompatible group plugin major version %d, expected %d\0" as *const u8
                    as *const libc::c_char,
                path.as_mut_ptr(),
                SUDO_API_VERSION_GET_MAJOR!((*group_plugin).version),
                GROUP_API_VERSION_MAJOR
            );

            break 'done;
        }

        /*
         * Split args into a vector if specified.
         */
        if !args.is_null() {
            let mut ac: libc::c_int = 0 as libc::c_int;
            let mut wasblank: bool = 1 as libc::c_int != 0;
            let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut last: *mut libc::c_char = 0 as *mut libc::c_char;
            cp = args;

            while *cp as libc::c_int != '\0' as i32 {
                if isblank!(*cp as libc::c_uchar as libc::c_int as isize) != 0 {
                    wasblank = true;
                } else if wasblank {
                    wasblank = false;
                    ac += 1;
                }
                cp = cp.offset(1);
            }
            if ac != 0 {
                argv = reallocarray(
                    0 as *mut libc::c_void,
                    ac as size_t,
                    ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ) as *mut *mut libc::c_char;
                if argv.is_null() {
                    sudo_warnx!(
                        b"%s: %s\0" as *const u8 as *const libc::c_char,
                        get_function_name!(),
                        b"unable to allocate memory\0" as *const u8 as *const libc::c_char
                    );
                    break 'done;
                }

                ac = 0;
                cp = strtok_r(
                    args,
                    b" \t\0" as *const u8 as *const libc::c_char,
                    &mut last,
                );
                while !cp.is_null() {
                    let fresh0 = ac;
                    ac = ac + 1;
                    let ref mut fresh1 = *argv.offset(fresh0 as isize);
                    *fresh1 = cp;
                    cp = strtok_r(
                        0 as *mut libc::c_char,
                        b" \t\0" as *const u8 as *const libc::c_char,
                        &mut last,
                    );
                }
            }
        }

        rc = ((*group_plugin).init).expect("non-null function pointer")(
            GROUP_API_VERSION!(),
            sudo_printf,
            argv as *const *mut libc::c_char,
        );

        break 'done;
    }

    free(argv as *mut libc::c_void);
    if rc != true as libc::c_int {
        if !group_handle.is_null() {
            sudo_dso_unload_v1(group_handle);
            group_handle = 0 as *mut libc::c_void;
            group_plugin = 0 as *mut sudoers_group_plugin;
        }
    }

    debug_return_int!(rc);
}

#[no_mangle]
pub unsafe extern "C" fn group_plugin_unload() {
    debug_decl!(SUDOERS_DEBUG_UTIL!());

    if !group_plugin.is_null() {
        ((*group_plugin).cleanup).expect("non-null function pointer")();
        group_plugin = 0 as *mut sudoers_group_plugin;
    }
    if !group_handle.is_null() {
        sudo_dso_unload_v1(group_handle);
        group_handle = 0 as *mut libc::c_void;
    }

    debug_return!();
}
