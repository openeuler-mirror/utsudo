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

unsafe extern "C" fn sudo_load_plugin(
    mut policy_plugin: *mut plugin_container,
    mut io_plugins: *mut plugin_container_list,
    mut info: *mut plugin_info,
) -> bool {
    let mut current_block: u64;
    let mut container: *mut plugin_container = 0 as *mut plugin_container;
    let mut plugin: *mut generic_plugin = 0 as *mut generic_plugin;
    let mut path: [libc::c_char; 4096] = [0; 4096];
    let mut handle: *mut libc::c_void = 0 as *mut libc::c_void;
    debug_decl!(sudo_load_plugin, SUDO_DEBUG_PLUGIN);
    'bad: loop {
        if !sudo_check_plugin(
            info,
            path.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        ) {
            break 'bad;
        }
        handle = sudo_dso_load_v1(path.as_mut_ptr(), 0x1 as libc::c_int | 0x4 as libc::c_int);
        if handle.is_null() {
            let mut errstr: *const libc::c_char = sudo_dso_strerror_v1();
            sudo_debug_printf!(
                SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"error in %s, line %d while loading plugin \"%s\"\0" as *const u8
                        as *const libc::c_char
                ),
                b"/etc/utsudo.conf\0" as *const u8 as *const libc::c_char,
                (*info).lineno,
                (*info).symbol_name
            );
            sudo_warnx_nodebug_v1(
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"error in %s, line %d while loading plugin \"%s\"\0" as *const u8
                        as *const libc::c_char,
                ),
                b"/etc/utsudo.conf\0" as *const u8 as *const libc::c_char,
                (*info).lineno,
                (*info).symbol_name,
            );
            sudo_debug_printf!(
                SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"unable to load %s: %s\0" as *const u8 as *const libc::c_char
                ),
                path,
                if !errstr.is_null() {
                    errstr
                } else {
                    b"unknown error\0" as *const u8 as *const libc::c_char
                }
            );
            sudo_warnx_nodebug_v1(
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"unable to load %s: %s\0" as *const u8 as *const libc::c_char,
                ),
                path,
                if !errstr.is_null() {
                    errstr
                } else {
                    b"unknown error\0" as *const u8 as *const libc::c_char
                },
            );
            break 'bad;
        }
        plugin = sudo_dso_findsym_v1(handle, (*info).symbol_name) as *mut generic_plugin;
        if plugin.is_null() {
            //sudo_warnx(U_("error in %s, line %d while loading plugin \"%s\""),_PATH_SUDO_CONF, info->lineno, info->symbol_name);
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
            //sudo_warnx(U_("unable to find symbol \"%s\" in %s"), info->symbol_name, path);
            sudo_debug_printf!(
                SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"unable to find symbol \"%s\" in %s\0" as *const u8 as *const libc::c_char
                ),
                (*info).symbol_name,
                path
            );
            sudo_warnx_nodebug_v1(
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"unable to find symbol \"%s\" in %s\0" as *const u8 as *const libc::c_char,
                ),
                (*info).symbol_name,
                path,
            );
            break 'bad;
        }
        if (*plugin).type_0 != 1 as libc::c_int as libc::c_uint
            && (*plugin).type_0 != 2 as libc::c_int as libc::c_uint
        {
            //sudo_warnx(U_("error in %s, line %d while loading plugin \"%s\""),_PATH_SUDO_CONF, info->lineno, info->symbol_name);
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
            //sudo_warnx(U_("unknown policy type %d found in %s"), plugin->type, path);
            sudo_debug_printf!(
                SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"unknown policy type %d found in %s\0" as *const u8 as *const libc::c_char
                ),
                (*plugin).type_0,
                path
            );
            sudo_warnx_nodebug_v1(
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"unable to find symbol \"%s\" in %s\0" as *const u8 as *const libc::c_char,
                ),
                (*plugin).type_0,
                path,
            );
            break 'bad;
        }
        if (*plugin).version >> 16 as libc::c_int != 1 as libc::c_int as libc::c_uint {
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
            //sudo_warnx(U_("incompatible plugin major version %d (expected %d) found in %s"),SUDO_API_VERSION_GET_MAJOR(plugin->version),SUDO_API_VERSION_MAJOR, path);
            sudo_debug_printf!(
                SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"incompatible plugin major version %d (expected %d) found in %s\0" as *const u8
                        as *const libc::c_char
                ),
                (*plugin).version >> 16 as libc::c_int,
                1,
                path
            );
            sudo_warnx_nodebug_v1(
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"incompatible plugin major version %d (expected %d) found in %s\0" as *const u8
                        as *const libc::c_char,
                ),
                (*plugin).version >> 16 as libc::c_int,
                1,
                path,
            );
            break 'bad;
        }
        if (*plugin).type_0 == 1 as libc::c_int as libc::c_uint {
            if !((*policy_plugin).handle).is_null() {
                if strcmp((*policy_plugin).name, (*info).symbol_name) != 0 as libc::c_int {
                    sudo_debug_printf!(
                        SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
                        sudo_warn_gettext_v1(
                            0 as *const libc::c_char,
                            b"ignoring policy plugin \"%d\0" as *const u8 as *const libc::c_char
                        ),
                        (*info).symbol_name,
                        b"/etc/utsudo.conf\0" as *const u8 as *const libc::c_char,
                        (*info).lineno
                    );
                    sudo_warnx_nodebug_v1(
                        sudo_warn_gettext_v1(
                            0 as *const libc::c_char,
                            b"error in %s,line %d while loading plugin \"%s\"\0" as *const u8
                                as *const libc::c_char,
                        ),
                        (*info).symbol_name,
                        b"/etc/utsudo.conf\0" as *const u8 as *const libc::c_char,
                        (*info).lineno,
                    );
                    sudo_debug_printf!(
                        SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
                        sudo_warn_gettext_v1(
                            0 as *const libc::c_char,
                            b"only a single policy plugin may be specified\0" as *const u8
                                as *const libc::c_char
                        )
                    );
                    sudo_warnx_nodebug_v1(sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"only a single policy plugin may be specified\0" as *const u8
                            as *const libc::c_char,
                    ));
                    break 'bad;
                }
                //sudo_warnx(U_("ignoring duplicate policy plugin \"%s\" in %s, line %d"),info->symbol_name, _PATH_SUDO_CONF, info->lineno);
                sudo_debug_printf!(
                    SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"ignoring duplicate policy plugin \"%s\" in %s, line %d\0" as *const u8
                            as *const libc::c_char
                    ),
                    (*info).symbol_name,
                    b"/etc/utsudo.conf\0" as *const u8 as *const libc::c_char,
                    (*info).lineno
                );
                sudo_warnx_nodebug_v1(
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"ignoring duplicate policy plugin \"%s\" in %s, line %d\0" as *const u8
                            as *const libc::c_char,
                    ),
                    (*info).symbol_name,
                    b"/etc/utsudo.conf\0" as *const u8 as *const libc::c_char,
                    (*info).lineno,
                );
                break 'bad;
            }
            (*policy_plugin).handle = handle;
            (*policy_plugin).path = strdup(path.as_mut_ptr());
            if ((*policy_plugin).path).is_null() {
                //define sudo_warnx(U_("%s:%s"),__func__,U_("unable to allocate memory"));
                sudo_debug_printf!(
                    SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"%s : %s\0" as *const u8 as *const libc::c_char
                    ),
                    function_name!(),
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"unable to allocate memory\0" as *const u8 as *const libc::c_char
                    )
                );
                sudo_warnx_nodebug_v1(
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"%s :%s\0" as *const u8 as *const libc::c_char,
                    ),
                    function_name!(),
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"unable to allocate memory\0" as *const u8 as *const libc::c_char,
                    ),
                );
                //end of define
                break 'bad;
            }
            (*policy_plugin).name = (*info).symbol_name;
            (*policy_plugin).options = (*info).options;
            (*policy_plugin).debug_instance = -(1 as libc::c_int);
            (*policy_plugin).u.generic = plugin;
            (*policy_plugin).debug_files = sudo_conf_debug_files_v1(path.as_mut_ptr());
        } else if (*plugin).type_0 == 2 as libc::c_int as libc::c_uint {
            container = (*io_plugins).tqh_first;
            while !container.is_null() {
                if strcmp((*container).name, (*info).symbol_name) == 0 as libc::c_int {
                    //sudo_warnx(U_("ignoring duplicate I/O plugin \"%s\" in %s, line %d"),info->symbol_name, _PATH_SUDO_CONF, info->lineno);
                    sudo_debug_printf!(
                        SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
                        sudo_warn_gettext_v1(
                            0 as *const libc::c_char,
                            b"ignoring duplicate policy plugin \"%s\" in %s, line %d\0" as *const u8
                                as *const libc::c_char
                        ),
                        (*info).symbol_name,
                        b"/etc/utsudo.conf\0" as *const u8 as *const libc::c_char,
                        (*info).lineno
                    );
                    sudo_warnx_nodebug_v1(
                        sudo_warn_gettext_v1(
                            0 as *const libc::c_char,
                            b"ignoring duplicate I/O plugin \"%s\" in %s, line %d\0" as *const u8
                                as *const libc::c_char,
                        ),
                        (*info).symbol_name,
                        b"/etc/utsudo.conf\0" as *const u8 as *const libc::c_char,
                        (*info).lineno,
                    );
                    sudo_dso_unload_v1(handle);
                    handle = 0 as *mut libc::c_void;
                    break;
                } else {
                    container = (*container).entries.tqe_next;
                }
            }
            container = calloc(
                1 as libc::c_int as libc::c_ulong,
                ::std::mem::size_of::<plugin_container>() as libc::c_ulong,
            ) as *mut plugin_container;
            (*container).path = strdup(path.as_mut_ptr());
            if container.is_null() || ((*container).path).is_null() {
                //define sudo_warnx(U_("%s: %s"),__func__,U_("unable to allocate memory"));
                sudo_debug_printf!(
                    SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"%s: %s\0" as *const u8 as *const libc::c_char
                    ),
                    function_name!(),
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"unable to allocate memory\0" as *const u8 as *const libc::c_char
                    )
                );
                sudo_warnx_nodebug_v1(
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"%s: %s\0" as *const u8 as *const libc::c_char,
                    ),
                    function_name!(),
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"unable to allocate memory\0" as *const u8 as *const libc::c_char,
                    ),
                );
                break 'bad;
            }
            (*container).handle = handle;
            (*container).name = (*info).symbol_name;
            (*container).options = (*info).options;
            (*container).debug_instance = -(1 as libc::c_int);
            (*container).u.generic = plugin;
            (*container).debug_files = sudo_conf_debug_files_v1(path.as_mut_ptr());
            (*container).entries.tqe_next = 0 as *mut plugin_container;
            (*container).entries.tqe_prev = (*io_plugins).tqh_last;
            *(*io_plugins).tqh_last = container;
            (*io_plugins).tqh_last = &mut (*container).entries.tqe_next;
        } //end of else if
        (*info).symbol_name = 0 as *mut libc::c_char;
        (*info).options = 0 as *mut *mut libc::c_char;
        debug_return_bool!(true);
        break 'bad;
    } //end of goto bad
    free(container as *mut libc::c_void);
    if !handle.is_null() {
        sudo_dso_unload_v1(handle);
    }
    debug_return_bool!(false)
} //end of func

#[no_mangle]
pub unsafe extern "C" fn sudo_load_plugins(
    mut policy_plugin: *mut plugin_container,
    mut io_plugins: *mut plugin_container_list,
) -> bool {
    let mut container: *mut plugin_container = 0 as *mut plugin_container;
    let mut plugins: *mut plugin_info_list = 0 as *mut plugin_info_list;
    let mut info: *mut plugin_info = 0 as *mut plugin_info;
    let mut next: *mut plugin_info = 0 as *mut plugin_info;
    let mut ret: bool = 0 as libc::c_int != 0;
    //define debug_decl(function_name,SUDO_DEBUG_PLUGIN);
    debug_decl!(sudo_load_plugins, SUDO_DEBUG_PLUGIN);
    //end of define
    plugins = sudo_conf_plugins_v1();

    'bad: loop {
        info = (*plugins).tqh_first;

        loop {
            if !(!info.is_null() && {
                next = (*info).entries.tqe_next;
                1 as libc::c_int != 0
            }) {
                break;
            }
            ret = sudo_load_plugin(policy_plugin, io_plugins, info);
            if !ret {
                break 'bad;
            }
            free_plugin_info(info);
            info = next;
        } //end of loop

        (*plugins).tqh_first = 0 as *mut plugin_info;
        (*plugins).tqh_last = &mut (*plugins).tqh_first;
        if ((*policy_plugin).handle).is_null() {
            info = calloc(
                1 as libc::c_int as libc::c_ulong,
                ::std::mem::size_of::<plugin_info>() as libc::c_ulong,
            ) as *mut plugin_info;
            if info.is_null() {
                //define sudo_warnx(U_("%s: %s"),__func__,U_("unable to allocate memory"));
                sudo_debug_printf!(
                    SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"%s: %s\0" as *const u8 as *const libc::c_char
                    ),
                    function_name!(),
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"unable to allocate memory\0" as *const u8 as *const libc::c_char
                    )
                );
                sudo_warnx_nodebug_v1(
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"%s: %s\0" as *const u8 as *const libc::c_char,
                    ),
                    function_name!(),
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"unable to allocate memory\0" as *const u8 as *const libc::c_char,
                    ),
                );
                break 'bad;
            }
            (*info).symbol_name = strdup(b"sudoers_policy\0" as *const u8 as *const libc::c_char);
            (*info).path = strdup(b"sudoers.so\0" as *const u8 as *const libc::c_char);
            if ((*info).symbol_name).is_null() || ((*info).path).is_null() {
                //define sudo_warnx(U_("%s: %s"),__func__,U_("unable to allocate memory"));
                sudo_debug_printf!(
                    SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"%s: %s\0" as *const u8 as *const libc::c_char
                    ),
                    function_name!(),
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"unable to allocate memory\0" as *const u8 as *const libc::c_char
                    )
                );
                sudo_warnx_nodebug_v1(
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"%s: %s\0" as *const u8 as *const libc::c_char,
                    ),
                    function_name!(),
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"unable to allocate memory\0" as *const u8 as *const libc::c_char,
                    ),
                );
                free_plugin_info(info);
                break 'bad;
            }
            ret = sudo_load_plugin(policy_plugin, io_plugins, info);
            free_plugin_info(info);
            if !ret {
                break 'bad;
            }
            if ((*io_plugins).tqh_first).is_null() {
                info = calloc(
                    1 as libc::c_int as libc::c_ulong,
                    ::std::mem::size_of::<plugin_info>() as libc::c_ulong,
                ) as *mut plugin_info;
                if info.is_null() {
                    //define sudo_warnx(U_("%s: %s"),__func__,U_("unable to allocate memory"));
                    sudo_debug_printf!(
                        SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
                        sudo_warn_gettext_v1(
                            0 as *const libc::c_char,
                            b"%s: %s\0" as *const u8 as *const libc::c_char
                        ),
                        function_name!(),
                        sudo_warn_gettext_v1(
                            0 as *const libc::c_char,
                            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
                        )
                    );
                    sudo_warnx_nodebug_v1(
                        sudo_warn_gettext_v1(
                            0 as *const libc::c_char,
                            b"%s: %s\0" as *const u8 as *const libc::c_char,
                        ),
                        function_name!(),
                        sudo_warn_gettext_v1(
                            0 as *const libc::c_char,
                            b"unable to allocate memory\0" as *const u8 as *const libc::c_char,
                        ),
                    );
                    break 'bad;
                }
                (*info).symbol_name = strdup(b"sudoers_io\0" as *const u8 as *const libc::c_char);
                (*info).path = strdup(b"sudoers.so\0" as *const u8 as *const libc::c_char);
                if ((*info).symbol_name).is_null() || ((*info).path).is_null() {
                    //define sudo_warnx(U_("%s: %s"),__func__,U_("unable to allocate memory"));
                    sudo_debug_printf!(
                        SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
                        sudo_warn_gettext_v1(
                            0 as *const libc::c_char,
                            b"%s: %s\0" as *const u8 as *const libc::c_char
                        ),
                        function_name!(),
                        sudo_warn_gettext_v1(
                            0 as *const libc::c_char,
                            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
                        )
                    );
                    sudo_warnx_nodebug_v1(
                        sudo_warn_gettext_v1(
                            0 as *const libc::c_char,
                            b"%s: %s\0" as *const u8 as *const libc::c_char,
                        ),
                        function_name!(),
                        sudo_warn_gettext_v1(
                            0 as *const libc::c_char,
                            b"unable to allocate memory\0" as *const u8 as *const libc::c_char,
                        ),
                    );
                    free_plugin_info(info);
                    break 'bad;
                }
                ret = sudo_load_plugin(policy_plugin, io_plugins, info);
                free_plugin_info(info);
                if !ret {
                    break 'bad;
                }
            }
        } //end of (*policy_plugin).handle

        if ((*(*policy_plugin).u.policy).check_policy).is_none() {
            //sudo_warnx(U_("policy plugin %s does not include a check_policy method"),policy_plugin->name);
            sudo_debug_printf!(
                SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"policy plugin %s does not include a check_policy method\0" as *const u8
                        as *const libc::c_char
                ),
                (*policy_plugin).name
            );
            sudo_warnx_nodebug_v1(
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"policy plugin %s does not include a check_policy method\0" as *const u8
                        as *const libc::c_char,
                ),
                (*policy_plugin).name,
            );
            ret = 0 as libc::c_int != 0;
            break 'bad;
        }
















        break 'bad;
    } //end of goto bad;
    debug_return_bool!(ret)
} //end of func


