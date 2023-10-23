/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */
#![allow(unused_imports)]
#![allow(unused_variables)]
use crate::struct_macro::*;
use utsudo_util::sudo_debug::*;
use utsudo_util::sudo_debug_macro::*;
use utsudo_util::*;
extern "C" {
    fn sudo_warnx_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn register_hook(hook: *mut sudo_hook) -> libc::c_int;
    static mut sudo_debug_instance: libc::c_int;
    fn sudo_strlcpy(dst: *mut libc::c_char, src: *const libc::c_char, siz: size_t) -> size_t;
    fn sudo_warn_gettext_v1(
        domainname: *const libc::c_char,
        msgid: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn sudo_warn_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn sudo_conf_plugin_dir_path_v1() -> *const libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn sudo_dso_strerror_v1() -> *mut libc::c_char;
    fn sudo_dso_load_v1(path: *const libc::c_char, mode: libc::c_int) -> *mut libc::c_void;
    fn sudo_dso_findsym_v1(
        handle: *mut libc::c_void,
        symbol: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn sudo_conf_debug_files_v1(progname: *const libc::c_char) -> *mut sudo_conf_debug_file_list;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn sudo_dso_unload_v1(handle: *mut libc::c_void) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn sudo_conf_plugins_v1() -> *mut plugin_info_list;
    fn sudo_debug_set_active_instance_v1(inst: libc::c_int) -> libc::c_int;
}
use crate::sudo_debug_printf2_v1;
use stdext::function_name;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct plugin_info {
    pub entries: tmp_1,
    pub path: *mut libc::c_char,
    pub symbol_name: *mut libc::c_char,
    pub options: *mut *mut libc::c_char,
    pub lineno: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plugin_info_list {
    pub tqh_first: *mut plugin_info,
    pub tqh_last: *mut *mut plugin_info,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tmp_1 {
    pub tqe_next: *mut plugin_info,
    pub tqe_prev: *mut *mut plugin_info,
}

unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
        #[cfg(target_arch = "x86_64")]
        return __xstat(1 as libc::c_int, __path, __statbuf);
        #[cfg(not(target_arch = "x86_64"))]
        return __xstat(0 as libc::c_int, __path, __statbuf);
}

unsafe extern "C" fn sudo_check_plugin(
    mut info: *mut plugin_info,
    mut fullpath: *mut libc::c_char,
    mut pathsize: size_t,
) -> bool {
    let mut sb: stat = stat {
        st_dev: 0,
        st_ino: 0,
        #[cfg(target_arch = "x86_64")]
        st_nlink: 0,
        st_mode: 0,
        #[cfg(not(target_arch = "x86_64"))]
        st_nlink: 0,
        st_uid: 0,
        st_gid: 0,
        #[cfg(target_arch = "x86_64")]
        __pad0: 0,
        st_rdev: 0,
        #[cfg(not(target_arch = "x86_64"))]
        __pad1: 0,
        st_size: 0,
        st_blksize: 0,
        #[cfg(not(target_arch = "x86_64"))]
        __pad2: 0,
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
        #[cfg(target_arch = "x86_64")]
        __glibc_reserved: [0; 3],
        #[cfg(not(target_arch = "x86_64"))]
        __glibc_reserved: [0; 2],
    };
    let mut ret: bool = 0 as libc::c_int != 0;
    debug_decl!(sudo_check_plugin, SUDO_DEBUG_PLUGIN);
    'done: loop {
        if sudo_stat_plugin(info, fullpath, pathsize, &mut sb) != 0 as libc::c_int {
            //define sudo_warnx(U_("error in %s,line %d while loading plugin
            //\"%s\""),"/etc/utsudo.conf",info->lineno,info->symbol_name);
            sudo_debug_printf!(
                SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"error in %s,line %d while loading plugin \"%s\"\0" as *const u8
                        as *const libc::c_char
                ),
                b"/etc/utsudo.conf\0" as *const u8 as *const libc::c_char,
                (*info).lineno,
                (*info).symbol_name
            );
            sudo_warnx_nodebug_v1(
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"error in %s,line %d while loading plugin \"%s\"\0" as *const u8
                        as *const libc::c_char,
                ),
                b"/etc/utsudo.conf\0" as *const u8 as *const libc::c_char,
                (*info).lineno,
                (*info).symbol_name,
            );
            //end of define
            if *((*info).path).offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
                //define sudo_warn("%s",info->path);
                sudo_debug_printf!(
                    SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    (*info).path
                );
                sudo_warn_nodebug_v1(b"%s\0" as *const u8 as *const libc::c_char, (*info).path);
                //end of define
            } else {
                sudo_debug_printf!(
                    SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                    b"%s%s\0" as *const u8 as *const libc::c_char,
                    if !(sudo_conf_plugin_dir_path_v1()).is_null() {
                        sudo_conf_plugin_dir_path_v1()
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                    (*info).path
                );
                sudo_warn_nodebug_v1(
                    b"%s%s\0" as *const u8 as *const libc::c_char,
                    if !(sudo_conf_plugin_dir_path_v1()).is_null() {
                        sudo_conf_plugin_dir_path_v1()
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                    (*info).path,
                );
            }
            break 'done;
        }
        if sb.st_uid != 0 as libc::c_int as libc::c_uint {
            //define sudo_warnx(U_("error in %s,line %d while loading plugin
            //\"%s\""),"/etc/utsudo.conf",info->lineno,info->symbol_name);
            sudo_debug_printf!(
                SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"error in %s,line %d while loading plugin \"%s\"\0" as *const u8
                        as *const libc::c_char
                ),
                b"/etc/utsudo.conf\0" as *const u8 as *const libc::c_char,
                (*info).lineno,
                (*info).symbol_name
            );
            sudo_warnx_nodebug_v1(
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"error in %s,line %d while loading plugin \"%s\"\0" as *const u8
                        as *const libc::c_char,
                ),
                b"/etc/utsudo.conf\0" as *const u8 as *const libc::c_char,
                (*info).lineno,
                (*info).symbol_name,
            );
            //end of define
            //define sudo_warnx(U_("%s must be owned by uid %d"),fullpath,0);
            sudo_debug_printf!(
                SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"%s must be owned by uid %d\0" as *const u8 as *const libc::c_char
                ),
                fullpath,
                0
            );
            sudo_warnx_nodebug_v1(
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"%s must be owned by uid %d\0" as *const u8 as *const libc::c_char,
                ),
                fullpath,
                0,
            );
            //end of define
            break 'done;
        }
        if sb.st_mode
            & (0o200 as libc::c_int >> 3 as libc::c_int
                | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
        {
            //define sudo_warnx(U_("error in %s,line %d while loading plugin
            //\"%s\""),"/etc/utsudo.conf",info->lineno,info->symbol_name);
            sudo_debug_printf!(
                SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"error in %s,line %d while loading plugin \"%s\"\0" as *const u8
                        as *const libc::c_char
                ),
                b"/etc/utsudo.conf\0" as *const u8 as *const libc::c_char,
                (*info).lineno,
                (*info).symbol_name
            );
            sudo_warnx_nodebug_v1(
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"error in %s,line %d while loading plugin \"%s\"\0" as *const u8
                        as *const libc::c_char,
                ),
                b"/etc/utsudo.conf\0" as *const u8 as *const libc::c_char,
                (*info).lineno,
                (*info).symbol_name,
            );
            //end of define
            //define sudo_warnx(U_("%s must be writable by owner"),fullpath);
            sudo_debug_printf!(
                SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"%s must be writable by owner\0" as *const u8 as *const libc::c_char
                ),
                fullpath
            );
            sudo_warnx_nodebug_v1(
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"%s must be owned by uid %d\0" as *const u8 as *const libc::c_char,
                ),
                fullpath,
            );
            //end of define
            break 'done;
        }
        ret = 1 as libc::c_int != 0;
        break 'done;
    } //end of done;
      //debug_return_bool!(ret);
      //return ret;
    let mut sudo_debug_ret: bool = ret;
    sudo_debug_exit_bool_v1(
        (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"sudo_check_plugin\0")).as_ptr(),
        b"load_plugins.c\0" as *const u8 as *const libc::c_char,
        148 as libc::c_int,
        sudo_debug_subsys,
        sudo_debug_ret,
    );
    return sudo_debug_ret;
} //end of func

unsafe extern "C" fn free_plugin_info(mut info: *mut plugin_info) {
    free((*info).path as *mut libc::c_void);
    free((*info).symbol_name as *mut libc::c_void);
    if !((*info).options).is_null() {
        let mut i: libc::c_int = 0 as libc::c_int;
        while !(*((*info).options).offset(i as isize)).is_null() {
            free(*((*info).options).offset(i as isize) as *mut libc::c_void);
            i = i + 1;
        }
        free((*info).options as *mut libc::c_void);
    }
    free(info as *mut libc::c_void);
} //end of func




