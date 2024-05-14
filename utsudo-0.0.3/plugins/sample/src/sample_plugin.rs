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
    unused_variables,
    unused_unsafe
)]

use utsudo_util::common::*;

pub const SUDO_API_VERSION_MAJOR: libc::c_int = 1;
pub const SUDO_CONV_ERROR_MSG: libc::c_int = 3;
pub const SUDO_CONV_INFO_MSG: libc::c_int = 4;
pub const PATH_MAX: libc::c_int = 4096;
pub const SUDO_CONV_PROMPT_ECHO_OFF: libc::c_int = 1;
pub const _ISblank: libc::c_int = 1;
pub const O_WRONLY: libc::c_int = 1;
pub const O_CREAT: libc::c_int = 64;
pub const O_EXCL: libc::c_int = 128;

pub type uid_t = libc::c_uint;
pub type gid_t = libc::c_uint;

pub type sudo_conv_t = Option<
    unsafe extern "C" fn(
        libc::c_int,
        *const sudo_conv_message,
        *mut sudo_conv_reply,
        *mut sudo_conv_callback,
    ) -> libc::c_int,
>;
pub type sudo_conv_callback_fn_t =
    Option<unsafe extern "C" fn(libc::c_int, *mut libc::c_void) -> libc::c_int>;
pub type sudo_hook_fn_t = Option<unsafe extern "C" fn() -> libc::c_int>;

static mut input: *mut FILE = 0 as *const FILE as *mut FILE;
static mut output: *mut FILE = 0 as *const FILE as *mut FILE;
static mut sudo_conv: sudo_conv_t = None;
static mut sudo_log: sudo_printf_t = None;
static mut use_sudoedit: libc::c_int = 0 as libc::c_int;
static mut runas_uid: uid_t = 0 as libc::c_int as uid_t;
static mut runas_gid: gid_t = -(1 as libc::c_int) as gid_t;
static mut plugin_state: plugin_state = plugin_state {
    envp: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    settings: 0 as *const *mut libc::c_char,
    user_info: 0 as *const *mut libc::c_char,
};

extern "C" {
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    fn getpid() -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn initprogname(_: *const libc::c_char);
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;

    fn asprintf(__ptr: *mut *mut libc::c_char, __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn sudo_new_key_val_v1(
        key: *const libc::c_char,
        value: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn getgrnam(__name: *const libc::c_char) -> *mut group;
    fn free(_: *mut libc::c_void);
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;

    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;

}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct io_plugin {
    pub type_0: libc::c_uint,
    pub version: libc::c_uint,
    pub open: Option<
        unsafe extern "C" fn(
            libc::c_uint,
            sudo_conv_t,
            sudo_printf_t,
            *const *mut libc::c_char,
            *const *mut libc::c_char,
            *const *mut libc::c_char,
            libc::c_int,
            *const *mut libc::c_char,
            *const *mut libc::c_char,
            *const *mut libc::c_char,
        ) -> libc::c_int,
    >,
    pub close: Option<unsafe extern "C" fn(libc::c_int, libc::c_int) -> ()>,
    pub show_version: Option<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
    pub log_ttyin: Option<unsafe extern "C" fn(*const libc::c_char, libc::c_uint) -> libc::c_int>,
    pub log_ttyout: Option<unsafe extern "C" fn(*const libc::c_char, libc::c_uint) -> libc::c_int>,
    pub log_stdin: Option<unsafe extern "C" fn(*const libc::c_char, libc::c_uint) -> libc::c_int>,
    pub log_stdout: Option<unsafe extern "C" fn(*const libc::c_char, libc::c_uint) -> libc::c_int>,
    pub log_stderr: Option<unsafe extern "C" fn(*const libc::c_char, libc::c_uint) -> libc::c_int>,
    pub register_hooks: Option<
        unsafe extern "C" fn(
            libc::c_int,
            Option<unsafe extern "C" fn(*mut sudo_hook) -> libc::c_int>,
        ) -> (),
    >,
    pub deregister_hooks: Option<
        unsafe extern "C" fn(
            libc::c_int,
            Option<unsafe extern "C" fn(*mut sudo_hook) -> libc::c_int>,
        ) -> (),
    >,
    pub change_winsize: Option<unsafe extern "C" fn(libc::c_uint, libc::c_uint) -> libc::c_int>,
    pub log_suspend: Option<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_hook {
    pub hook_version: libc::c_uint,
    pub hook_type: libc::c_uint,
    pub hook_fn: sudo_hook_fn_t,
    pub closure: *mut libc::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct policy_plugin {
    pub type_0: libc::c_uint,
    pub version: libc::c_uint,
    pub open: Option<
        unsafe extern "C" fn(
            libc::c_uint,
            sudo_conv_t,
            sudo_printf_t,
            *const *mut libc::c_char,
            *const *mut libc::c_char,
            *const *mut libc::c_char,
            *const *mut libc::c_char,
        ) -> libc::c_int,
    >,
    pub close: Option<unsafe extern "C" fn(libc::c_int, libc::c_int) -> ()>,
    pub show_version: Option<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
    pub check_policy: Option<
        unsafe extern "C" fn(
            libc::c_int,
            *const *mut libc::c_char,
            *mut *mut libc::c_char,
            *mut *mut *mut libc::c_char,
            *mut *mut *mut libc::c_char,
            *mut *mut *mut libc::c_char,
        ) -> libc::c_int,
    >,
    pub list: Option<
        unsafe extern "C" fn(
            libc::c_int,
            *const *mut libc::c_char,
            libc::c_int,
            *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub validate: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub invalidate: Option<unsafe extern "C" fn(libc::c_int) -> ()>,
    pub init_session:
        Option<unsafe extern "C" fn(*mut passwd, *mut *mut *mut libc::c_char) -> libc::c_int>,
    pub register_hooks: Option<
        unsafe extern "C" fn(
            libc::c_int,
            Option<unsafe extern "C" fn(*mut sudo_hook) -> libc::c_int>,
        ) -> (),
    >,
    pub deregister_hooks: Option<
        unsafe extern "C" fn(
            libc::c_int,
            Option<unsafe extern "C" fn(*mut sudo_hook) -> libc::c_int>,
        ) -> (),
    >,
}

#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(xstat_flag as libc::c_int, __path, __statbuf);
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct plugin_state {
    pub envp: *mut *mut libc::c_char,
    pub settings: *const *mut libc::c_char,
    pub user_info: *const *mut libc::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_conv_message {
    pub msg_type: libc::c_int,
    pub timeout: libc::c_int,
    pub msg: *const libc::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_conv_reply {
    pub reply: *mut libc::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_conv_callback {
    pub version: libc::c_uint,
    pub closure: *mut libc::c_void,
    pub on_suspend: sudo_conv_callback_fn_t,
    pub on_resume: sudo_conv_callback_fn_t,
}

#[no_mangle]
pub unsafe extern "C" fn policy_open(
    mut version: libc::c_uint,
    mut conversation: sudo_conv_t,
    mut sudo_printf: sudo_printf_t,
    mut settings: *const *mut libc::c_char,
    mut user_info: *const *mut libc::c_char,
    mut user_env: *const *mut libc::c_char,
    mut args: *const *mut libc::c_char,
) -> libc::c_int {
    let mut ui: *const *mut libc::c_char = 0 as *const *mut libc::c_char;
    let mut pw: *mut passwd = 0 as *mut passwd;
    let mut runas_user: *const libc::c_char = 0 as *const libc::c_char;
    let mut gr: *mut group = 0 as *mut group;
    let mut runas_group: *const libc::c_char = 0 as *const libc::c_char;

    if sudo_conv.is_none() {
        sudo_conv = conversation;
    }
    if sudo_log.is_none() {
        sudo_log = sudo_printf;
    }

    if version >> 16 as libc::c_int != SUDO_API_VERSION_MAJOR as libc::c_int as libc::c_uint {
        sudo_log.expect("non-null function pointer")(
            SUDO_CONV_ERROR_MSG as libc::c_int,
            b"the sample plugin requires API version %d.x\n\0" as *const u8 as *const libc::c_char,
            SUDO_API_VERSION_MAJOR as libc::c_int,
        );
        return -(1 as libc::c_int);
    }

    ui = settings;
    while !(*ui).is_null() {
        if strncmp(
            *ui,
            b"runas_user=\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0 as libc::c_int
        {
            runas_user = (*ui)
                .offset(::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as isize)
                .offset(-(1 as libc::c_int as isize));
        }

        if strncmp(
            *ui,
            b"runas_group=\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0 as libc::c_int
        {
            runas_group = (*ui)
                .offset(::core::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong as isize)
                .offset(-(1 as libc::c_int as isize));
        }

        if strncmp(
            *ui,
            b"progname=\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0 as libc::c_int
        {
            initprogname(
                (*ui)
                    .offset(::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong as isize)
                    .offset(-(1 as libc::c_int as isize)),
            );
        }

        if strncmp(
            *ui,
            b"sudoedit=\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0 as libc::c_int
        {
            if strcasecmp(
                (*ui)
                    .offset(::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong as isize)
                    .offset(-(1 as libc::c_int as isize)),
                b"true\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                use_sudoedit = 1 as libc::c_int;
            }
        }

        if strncmp(
            *ui,
            b"implied_shell=\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0 as libc::c_int
        {
            if strcasecmp(
                (*ui)
                    .offset(::core::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as isize)
                    .offset(-(1 as libc::c_int as isize)),
                b"true\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                return -(2 as libc::c_int);
            }
        }
        ui = ui.offset(1);
    }

    if !runas_user.is_null() {
        pw = getpwnam(runas_user);
        if pw.is_null() {
            sudo_log.expect("non-null function pointer")(
                SUDO_CONV_ERROR_MSG as libc::c_int,
                b"unknown user %s\n\0" as *const u8 as *const libc::c_char,
                runas_user,
            );
            return 0 as libc::c_int;
        }
        runas_uid = (*pw).pw_uid;
    }

    if !runas_group.is_null() {
        gr = getgrnam(runas_group);
        if gr.is_null() {
            sudo_log.expect("non-null function pointer")(
                SUDO_CONV_ERROR_MSG as libc::c_int,
                b"unknown group %s\n\0" as *const u8 as *const libc::c_char,
                runas_group,
            );
            return 0 as libc::c_int;
        }
        runas_gid = (*gr).gr_gid;
    }

    plugin_state.envp = user_env as *mut *mut libc::c_char;
    plugin_state.settings = settings;
    plugin_state.user_info = user_info;
    return 1;
}

#[no_mangle]
unsafe extern "C" fn find_in_path(
    mut command: *mut libc::c_char,
    mut envp: *mut *mut libc::c_char,
) -> *mut libc::c_char {
    let mut sb: stat = sb_all_arch;
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ep: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pathbuf: [libc::c_char; 4096] = [0; 4096];
    let mut qualified: *mut libc::c_char = 0 as *mut libc::c_char;

    if !(strchr(command, '/' as i32)).is_null() {
        return command;
    }

    path = b"/usr/bin:/bin\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    ep = plugin_state.envp;
    while !(*ep).is_null() {
        if strncmp(
            *ep,
            b"PATH=\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            path = (*ep).offset(5 as libc::c_int as isize);
            break;
        }
        ep = ep.offset(1);
    }

    path0 = strdup(path);
    path = path0;

    loop {
        cp = strchr(path, ':' as i32);
        if !cp.is_null() {
            *cp = '\0' as i32 as libc::c_char;
        }
        snprintf(
            pathbuf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
            b"%s/%s\0" as *const u8 as *const libc::c_char,
            if *path as libc::c_int != 0 {
                path as *const libc::c_char
            } else {
                b".\0" as *const u8 as *const libc::c_char
            },
            command,
        );
        if stat(pathbuf.as_mut_ptr(), &mut sb) == 0 as libc::c_int {
            if sb.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o100000 as libc::c_int as libc::c_uint
                && sb.st_mode & 0o111 as libc::c_int as libc::c_uint != 0
            {
                qualified = pathbuf.as_mut_ptr();
                break;
            }
        }
        path = cp.offset(1 as libc::c_int as isize);
        if cp.is_null() {
            break;
        }
    }

    free(path0 as *mut libc::c_void);
    return if !qualified.is_null() {
        strdup(qualified)
    } else {
        0 as *mut libc::c_char
    };
}

unsafe extern "C" fn check_passwd() -> libc::c_int {
    let mut msg: sudo_conv_message = sudo_conv_message {
        msg_type: 0,
        timeout: 0,
        msg: 0 as *const libc::c_char,
    };
    let mut repl: sudo_conv_reply = sudo_conv_reply {
        reply: 0 as *mut libc::c_char,
    };
    memset(
        &mut msg as *mut sudo_conv_message as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<sudo_conv_message>() as libc::c_ulong,
    );
    msg.msg_type = SUDO_CONV_PROMPT_ECHO_OFF as libc::c_int;
    msg.msg = b"Password: \0" as *const u8 as *const libc::c_char;
    memset(
        &mut repl as *mut sudo_conv_reply as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<sudo_conv_reply>() as libc::c_ulong,
    );
    sudo_conv.expect("non-null function pointer")(
        1 as libc::c_int,
        &mut msg as *mut sudo_conv_message as *const sudo_conv_message,
        &mut repl,
        0 as *mut sudo_conv_callback,
    );
    if (repl.reply).is_null() {
        sudo_log.expect("non-null function pointer")(
            SUDO_CONV_ERROR_MSG as libc::c_int,
            b"missing password\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if strcmp(repl.reply, b"test\0" as *const u8 as *const libc::c_char) != 0 as libc::c_int {
        sudo_log.expect("non-null function pointer")(
            SUDO_CONV_ERROR_MSG as libc::c_int,
            b"incorrect password\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}

#[no_mangle]
unsafe extern "C" fn build_command_info(
    mut command: *const libc::c_char,
) -> *mut *mut libc::c_char {
    static mut command_info: *mut *mut libc::c_char =
        0 as *const *mut libc::c_char as *mut *mut libc::c_char;
    let mut i: libc::c_int = 0 as libc::c_int;
    command_info = calloc(
        32 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    if command_info.is_null() {
        return 0 as *mut *mut libc::c_char;
    }

    let fresh0 = i;
    i = i + 1;
    let ref mut fresh1 = *command_info.offset(fresh0 as isize);
    *fresh1 = sudo_new_key_val_v1(b"command\0" as *const u8 as *const libc::c_char, command);
    if (*fresh1).is_null()
        || {
            let fresh2 = i;
            i = i + 1;
            asprintf(
                &mut *command_info.offset(fresh2 as isize) as *mut *mut libc::c_char,
                b"runas_euid=%ld\0" as *const u8 as *const libc::c_char,
                runas_uid as libc::c_long,
            ) == -(1 as libc::c_int)
        }
        || {
            let fresh3 = i;
            i = i + 1;
            asprintf(
                &mut *command_info.offset(fresh3 as isize) as *mut *mut libc::c_char,
                b"runas_uid=%ld\0" as *const u8 as *const libc::c_char,
                runas_uid as libc::c_long,
            ) == -(1 as libc::c_int)
        }
    {
        return 0 as *mut *mut libc::c_char;
    }

    if runas_gid != -(1 as libc::c_int) as gid_t {
        let fresh4 = i;
        i = i + 1;
        if asprintf(
            &mut *command_info.offset(fresh4 as isize) as *mut *mut libc::c_char,
            b"runas_gid=%ld\0" as *const u8 as *const libc::c_char,
            runas_gid as libc::c_long,
        ) == -(1 as libc::c_int)
            || {
                let fresh5 = i;
                i = i + 1;
                asprintf(
                    &mut *command_info.offset(fresh5 as isize) as *mut *mut libc::c_char,
                    b"runas_egid=%ld\0" as *const u8 as *const libc::c_char,
                    runas_gid as libc::c_long,
                ) == -(1 as libc::c_int)
            }
        {
            return 0 as *mut *mut libc::c_char;
        }
    }

    if use_sudoedit != 0 {
        let ref mut fresh6 = *command_info.offset(i as isize);
        *fresh6 = strdup(b"sudoedit=true\0" as *const u8 as *const libc::c_char);
        let fresh7 = i;
        i = i + 1;
        if (*command_info.offset(fresh7 as isize)).is_null() {
            return 0 as *mut *mut libc::c_char;
        }
    }

    return command_info;
}

#[no_mangle]
unsafe extern "C" fn find_editor(
    mut nfiles: libc::c_int,
    mut files: *const *mut libc::c_char,
    mut argv_out: *mut *mut *mut libc::c_char,
) -> *mut libc::c_char {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut last: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ep: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut nargv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut editor: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut editor_path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ac: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut nargc: libc::c_int = 0;
    let mut wasblank: libc::c_int = 0;
    editor = b"/usr/bin/vi\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    ep = plugin_state.envp;
    while !(*ep).is_null() {
        if strncmp(
            *ep,
            b"EDITOR=\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            editor = (*ep).offset(7 as libc::c_int as isize);
            break;
        }
        ep = ep.offset(1);
    }

    editor = strdup(editor);
    if editor.is_null() {
        sudo_log.expect("non-null function pointer")(
            SUDO_CONV_ERROR_MSG as libc::c_int,
            b"unable to allocate memory\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut libc::c_char;
    }

    nargc = 1 as libc::c_int;
    wasblank = 0 as libc::c_int;
    cp = editor;
    while *cp as libc::c_int != '\0' as i32 {
        if *(*__ctype_b_loc()).offset(*cp as libc::c_uchar as libc::c_int as isize) as libc::c_int
            & _ISblank as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            wasblank = 1 as libc::c_int;
        } else if wasblank != 0 {
            wasblank = 0 as libc::c_int;
            nargc += 1;
        }
        cp = cp.offset(1);
    }

    cp = strtok_r(
        editor,
        b" \t\0" as *const u8 as *const libc::c_char,
        &mut last,
    );
    if cp.is_null() || {
        editor_path = find_in_path(editor, plugin_state.envp);
        editor_path.is_null()
    } {
        free(editor as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    if editor_path != editor {
        free(editor as *mut libc::c_void);
    }

    nargv = malloc(
        ((nargc + 1 as libc::c_int + nfiles + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    if nargv.is_null() {
        sudo_log.expect("non-null function pointer")(
            SUDO_CONV_ERROR_MSG as libc::c_int,
            b"unable to allocate memory\n\0" as *const u8 as *const libc::c_char,
        );
        free(editor_path as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }

    ac = 0 as libc::c_int;
    while !cp.is_null() && ac < nargc {
        let ref mut fresh8 = *nargv.offset(ac as isize);
        *fresh8 = cp;
        cp = strtok_r(
            0 as *mut libc::c_char,
            b" \t\0" as *const u8 as *const libc::c_char,
            &mut last,
        );
        ac += 1;
    }

    let fresh9 = ac;
    ac = ac + 1;
    let ref mut fresh10 = *nargv.offset(fresh9 as isize);
    *fresh10 = b"--\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < nfiles {
        let fresh11 = i;
        i = i + 1;
        let fresh12 = ac;
        ac = ac + 1;
        let ref mut fresh13 = *nargv.offset(fresh12 as isize);
        *fresh13 = *files.offset(fresh11 as isize);
    }
    let ref mut fresh14 = *nargv.offset(ac as isize);
    *fresh14 = 0 as *mut libc::c_char;
    *argv_out = nargv;
    return editor_path;
}

#[no_mangle]
unsafe extern "C" fn policy_check(
    mut argc: libc::c_int,
    mut argv: *const *mut libc::c_char,
    mut env_add: *mut *mut libc::c_char,
    mut command_info_out: *mut *mut *mut libc::c_char,
    mut argv_out: *mut *mut *mut libc::c_char,
    mut user_env_out: *mut *mut *mut libc::c_char,
) -> libc::c_int {
    let mut command: *mut libc::c_char = 0 as *mut libc::c_char;

    if argc == 0 || (*argv.offset(0 as libc::c_int as isize)).is_null() {
        sudo_log.expect("non-null function pointer")(
            SUDO_CONV_ERROR_MSG as libc::c_int,
            b"no command specified\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if check_passwd() == 0 {
        return 0 as libc::c_int;
    }

    command = find_in_path(*argv.offset(0 as libc::c_int as isize), plugin_state.envp);
    if command.is_null() {
        sudo_log.expect("non-null function pointer")(
            SUDO_CONV_ERROR_MSG as libc::c_int,
            b"%s: command not found\n\0" as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
        return 0 as libc::c_int;
    }

    if strcmp(
        command,
        b"/usr/bin/vi\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        use_sudoedit = 1 as libc::c_int;
    }

    if use_sudoedit != 0 {
        free(command as *mut libc::c_void);
        command = find_editor(
            argc - 1 as libc::c_int,
            argv.offset(1 as libc::c_int as isize),
            argv_out,
        );
        if command.is_null() {
            sudo_log.expect("non-null function pointer")(
                SUDO_CONV_ERROR_MSG as libc::c_int,
                b"unable to find valid editor\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        use_sudoedit = 1 as libc::c_int;
    } else {
        *argv_out = argv as *mut *mut libc::c_char;
    }

    *user_env_out = plugin_state.envp;
    *command_info_out = build_command_info(command);
    free(command as *mut libc::c_void);
    if (*command_info_out).is_null() {
        sudo_log.expect("non-null function pointer")(
            SUDO_CONV_ERROR_MSG as libc::c_int,
            b"out of memory\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn policy_list(
    mut argc: libc::c_int,
    mut argv: *const *mut libc::c_char,
    mut verbose: libc::c_int,
    mut list_user: *const libc::c_char,
) -> libc::c_int {
    sudo_log.expect("non-null function pointer")(
        SUDO_CONV_INFO_MSG as libc::c_int,
        b"Validated users may run any command\n\0" as *const u8 as *const libc::c_char,
    );
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn policy_version(mut verbose: libc::c_int) -> libc::c_int {
    sudo_log.expect("non-null function pointer")(
        SUDO_CONV_INFO_MSG as libc::c_int,
        b"Sample policy plugin version %s\n\0" as *const u8 as *const libc::c_char,
        b"1.8.29\0" as *const u8 as *const libc::c_char,
    );
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn policy_close(mut exit_status: libc::c_int, mut error: libc::c_int) {
    if error != 0 {
        sudo_log.expect("non-null function pointer")(
            SUDO_CONV_ERROR_MSG as libc::c_int,
            b"Command error: %s\n\0" as *const u8 as *const libc::c_char,
            strerror(error),
        );
    } else if exit_status & 0x7f as libc::c_int == 0 as libc::c_int {
        sudo_log.expect("non-null function pointer")(
            SUDO_CONV_INFO_MSG as libc::c_int,
            b"Command exited with status %d\n\0" as *const u8 as *const libc::c_char,
            (exit_status & 0xff00 as libc::c_int) >> 8 as libc::c_int,
        );
    } else if ((exit_status & 0x7f as libc::c_int) + 1 as libc::c_int) as libc::c_schar
        as libc::c_int
        >> 1 as libc::c_int
        > 0 as libc::c_int
    {
        sudo_log.expect("non-null function pointer")(
            SUDO_CONV_INFO_MSG as libc::c_int,
            b"Command killed by signal %d\n\0" as *const u8 as *const libc::c_char,
            exit_status & 0x7f as libc::c_int,
        );
    }
}

#[no_mangle]
unsafe extern "C" fn io_open(
    mut version: libc::c_uint,
    mut conversation: sudo_conv_t,
    mut sudo_printf: sudo_printf_t,
    mut settings: *const *mut libc::c_char,
    mut user_info: *const *mut libc::c_char,
    mut command_info: *const *mut libc::c_char,
    mut argc: libc::c_int,
    mut argv: *const *mut libc::c_char,
    mut user_env: *const *mut libc::c_char,
    mut args: *const *mut libc::c_char,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut path: [libc::c_char; PATH_MAX as usize] = [0; PATH_MAX as usize];
    if sudo_conv.is_none() {
        sudo_conv = conversation;
    }
    if sudo_log.is_none() {
        sudo_log = sudo_printf;
    }

    snprintf(
        path.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        b"/var/tmp/sample-%u.output\0" as *const u8 as *const libc::c_char,
        getpid() as libc::c_uint,
    );
    fd = open(
        path.as_mut_ptr(),
        O_WRONLY as libc::c_int | O_CREAT as libc::c_int | O_EXCL as libc::c_int,
        0o644 as libc::c_int,
    );
    if fd == -(1 as libc::c_int) {
        return 0 as libc::c_int;
    }
    output = fdopen(fd, b"w\0" as *const u8 as *const libc::c_char);

    snprintf(
        path.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        b"/var/tmp/sample-%u.input\0" as *const u8 as *const libc::c_char,
        getpid() as libc::c_uint,
    );
    fd = open(
        path.as_mut_ptr(),
        O_WRONLY as libc::c_int | O_CREAT as libc::c_int | O_EXCL as libc::c_int,
        0o644 as libc::c_int,
    );
    if fd == -(1 as libc::c_int) {
        return 0 as libc::c_int;
    }
    input = fdopen(fd, b"w\0" as *const u8 as *const libc::c_char);

    return 1 as libc::c_int;
}
