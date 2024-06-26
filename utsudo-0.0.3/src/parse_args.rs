/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(unused_imports)]
#![allow(clashing_extern_declarations)]
#![allow(improper_ctypes)]

use crate::struct_macro::*;

use libc::FILE;
//c中全局此处写法固定static mut,
#[no_mangle]
pub static mut tgetpass_flags: libc::c_int = 0;

static mut sudo_settings: [sudo_settings; 24] = [
    {
        let mut init = sudo_settings {
            name: b"bsdauth_type\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"login_class\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"preserve_environment\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"runas_group\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"set_home\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"run_shell\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"login_shell\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"ignore_ticket\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"prompt\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"selinux_role\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"selinux_type\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"runas_user\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"progname\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"implied_shell\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"preserve_groups\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"noninteractive\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"sudoedit\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"closefrom\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"network_addrs\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"max_groups\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"plugin_dir\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"remote_host\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: b"timeout\0" as *const u8 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = sudo_settings {
            name: 0 as *const libc::c_char,
            value: 0 as *const libc::c_char,
        };
        init
    },
];

#[derive(Copy, Clone)]
#[repr(C)]
pub struct environment {
    pub envp: *mut *mut libc::c_char,
    pub env_size: size_t,
    pub env_len: size_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}

//'a' as i32 ?
static mut long_opts: [option; 29] = [
    {
        let mut init = option {
            name: b"askpass\0" as *const u8 as *const libc::c_char,
            has_arg: 0,
            flag: 0 as *mut libc::c_int,
            val: 'A' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"auth-type\0" as *const u8 as *const libc::c_char,
            has_arg: 1,
            flag: 0 as *mut libc::c_int,
            val: 'a' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"background\0" as *const u8 as *const libc::c_char,
            has_arg: 0,
            flag: 0 as *mut libc::c_int,
            val: 'b' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"bell\0" as *const u8 as *const libc::c_char,
            has_arg: 0,
            flag: 0 as *mut libc::c_int,
            val: 'B' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"close-from\0" as *const u8 as *const libc::c_char,
            has_arg: 1,
            flag: 0 as *mut libc::c_int,
            val: 'C' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"login-class\0" as *const u8 as *const libc::c_char,
            has_arg: 1,
            flag: 0 as *mut libc::c_int,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"preserve-env\0" as *const u8 as *const libc::c_char,
            has_arg: 2,
            flag: 0 as *mut libc::c_int,
            val: 'E' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"edit\0" as *const u8 as *const libc::c_char,
            has_arg: 0,
            flag: 0 as *mut libc::c_int,
            val: 'e' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"group\0" as *const u8 as *const libc::c_char,
            has_arg: 1,
            flag: 0 as *mut libc::c_int,
            val: 'g' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"set-home\0" as *const u8 as *const libc::c_char,
            has_arg: 0,
            flag: 0 as *mut libc::c_int,
            val: 'H' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"help\0" as *const u8 as *const libc::c_char,
            has_arg: 0,
            flag: 0 as *mut libc::c_int,
            val: 'h' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"host\0" as *const u8 as *const libc::c_char,
            has_arg: 1,
            flag: 0 as *mut libc::c_int,
            val: 256 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"login\0" as *const u8 as *const libc::c_char,
            has_arg: 0,
            flag: 0 as *mut libc::c_int,
            val: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"remove-timestamp\0" as *const u8 as *const libc::c_char,
            has_arg: 0,
            flag: 0 as *mut libc::c_int,
            val: 'K' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"reset-timestamp\0" as *const u8 as *const libc::c_char,
            has_arg: 0,
            flag: 0 as *mut libc::c_int,
            val: 'k' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"list\0" as *const u8 as *const libc::c_char,
            has_arg: 0,
            flag: 0 as *mut libc::c_int,
            val: 'l' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"non-interactive\0" as *const u8 as *const libc::c_char,
            has_arg: 0,
            flag: 0 as *mut libc::c_int,
            val: 'n' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"preserve-groups\0" as *const u8 as *const libc::c_char,
            has_arg: 0,
            flag: 0 as *mut libc::c_int,
            val: 'P' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"prompt\0" as *const u8 as *const libc::c_char,
            has_arg: 1,
            flag: 0 as *mut libc::c_int,
            val: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"role\0" as *const u8 as *const libc::c_char,
            has_arg: 1,
            flag: 0 as *mut libc::c_int,
            val: 'r' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"stdin\0" as *const u8 as *const libc::c_char,
            has_arg: 0,
            flag: 0 as *mut libc::c_int,
            val: 'S' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"shell\0" as *const u8 as *const libc::c_char,
            has_arg: 0,
            flag: 0 as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"type\0" as *const u8 as *const libc::c_char,
            has_arg: 1,
            flag: 0 as *mut libc::c_int,
            val: 't' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"command-timeout\0" as *const u8 as *const libc::c_char,
            has_arg: 1,
            flag: 0 as *mut libc::c_int,
            val: 'T' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"other-user\0" as *const u8 as *const libc::c_char,
            has_arg: 1,
            flag: 0 as *mut libc::c_int,
            val: 'U' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"user\0" as *const u8 as *const libc::c_char,
            has_arg: 1,
            flag: 0 as *mut libc::c_int,
            val: 'u' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const libc::c_char,
            has_arg: 0,
            flag: 0 as *mut libc::c_int,
            val: 'V' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"validate\0" as *const u8 as *const libc::c_char,
            has_arg: 0,
            flag: 0 as *mut libc::c_int,
            val: 'v' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: 0 as *const libc::c_char,
            has_arg: 0,
            flag: 0 as *mut libc::c_int,
            val: '\u{0}' as i32,
        };
        init
    },
];

use crate::sudo_debug_printf2_v1;
pub const SUDO_DEBUG_ARGS: libc::c_int = 1 << 6;
use stdext::function_name;
use utsudo_util::debug_decl;
use utsudo_util::debug_decl_vars;
use utsudo_util::debug_return;
use utsudo_util::debug_return_int;
use utsudo_util::sudo_debug_macro::sudo_debug_subsys;
use utsudo_util::sudo_debug_macro::SUDO_DEBUG_ERRNO;
use utsudo_util::sudo_debug_macro::SUDO_DEBUG_ERROR;
use utsudo_util::sudo_debug_macro::SUDO_DEBUG_INFO;
use utsudo_util::sudo_debug_macro::SUDO_DEBUG_LINENO;
use utsudo_util::sudo_debug_macro::SUDO_DEBUG_WARN;
use utsudo_util::sudo_debug_printf;
//use utsudo_util::debug_return_str;
use utsudo_util::sudo_debug::sudo_debug_enter_v1;
use utsudo_util::sudo_debug::sudo_debug_exit_int_v1;
use utsudo_util::sudo_debug::sudo_debug_exit_str_v1;
use utsudo_util::sudo_debug::sudo_debug_exit_v1;

//externc
extern "C" {
    fn gc_add(type_0: sudo_gc_types, v: *mut libc::c_void) -> bool;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn sudo_conf_plugin_dir_path_v1() -> *const libc::c_char;
    static mut list_user: *const libc::c_char;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __functin: *const libc::c_char,
    ) -> !;
    static mut optind: libc::c_int;
    fn reallocarray(__ptr: *mut libc::c_void, __nmemb: size_t, __size: size_t)
        -> *mut libc::c_void;
    fn sudo_fatalx_nodebug_v1(fmt: *const libc::c_char, _: ...) -> !;
    fn sudo_warn_gettext_v1(
        domainname: *const libc::c_char,
        msgid: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn sudo_new_key_val_v1(
        key: *const libc::c_char,
        value: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
    fn sudo_warn_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_getprogname() -> *const libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(__s: *const libc::c_char) -> size_t;
    fn exit(_: libc::c_int) -> !;
    fn sudo_lbuf_init_v1(
        lbuf: *mut sudo_lbuf,
        output: sudo_lbuf_output_t,
        indent: libc::c_int,
        continuation: *const libc::c_char,
        cols: libc::c_int,
    );
    fn sudo_lbuf_destroy_v1(lbuf: *mut sudo_lbuf);
    fn sudo_lbuf_append_v1(lbuf: *mut sudo_lbuf, fmt: *const libc::c_char, _: ...) -> bool;
    fn sudo_lbuf_print_v1(lbuf: *mut sudo_lbuf);
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    static mut user_details: user_details;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn get_net_ifs(addrinfo: *mut *mut libc::c_char) -> libc::c_int;
    fn sudo_conf_max_groups_v1() -> libc::c_int;
    //    fn sudo_fatal_nodebug_v1(fmt:*const libc::c_char,_:...) -> !;
    //    fn sudo_fatalx_nodebug_v1(fmt:*const libc::c_char,_:...) -> !;
    fn asprintf(__ptr: *mut *mut libc::c_char, __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn getopt_long(
        __argc: libc::c_int,
        __argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn sudo_strtonum(
        _: *const libc::c_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *const libc::c_char,
    ) -> libc::c_longlong;
    static mut optarg: *mut libc::c_char;
}

pub type sudo_lbuf_output_t = Option<unsafe extern "C" fn(*const libc::c_char) -> libc::c_int>;

static mut short_opts: [libc::c_char; 43] = unsafe {
    *::std::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
        b"+Aa:BbC:c:D:Eeg:Hh::iKklnPp:r:SsT:t:U:u:Vv\0",
    )
};

#[no_mangle]
pub unsafe extern "C" fn parse_args(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut nargc: *mut libc::c_int,
    mut nargv: *mut *mut *mut libc::c_char,
    mut settingsp: *mut *mut sudo_settings,
    mut env_addp: *mut *mut *mut libc::c_char,
) -> libc::c_int {
    let mut extra_env: environment = environment {
        envp: 0 as *mut *mut libc::c_char,
        env_size: 0,
        env_len: 0,
    };
    let mut mode: libc::c_int = 0 as libc::c_int;
    let mut flags: libc::c_int = 0 as libc::c_int;
    let mut valid_flags: libc::c_int = 0x10000 as libc::c_int
        | 0x400000 as libc::c_int
        | 0x100000 as libc::c_int
        | 0x40000 as libc::c_int
        | 0x800000 as libc::c_int
        | 0x200000 as libc::c_int
        | 0x20000 as libc::c_int;
    let mut ch: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut runas_user: *const libc::c_char = 0 as *const libc::c_char;
    let mut runas_group: *const libc::c_char = 0 as *const libc::c_char;
    let mut progname: *const libc::c_char = 0 as *const libc::c_char;
    let mut proglen: libc::c_int = 0;
    //define debug_decl(env_insert,SUDO_DEBUG_ARGS) 1<<6
    debug_decl!(parse_args, SUDO_DEBUG_ARGS);
    //end of define
    if argc <= 0 as libc::c_int {
        usage(1 as libc::c_int);
    }
    progname = sudo_getprogname();
    sudo_settings[12 as libc::c_int as usize].value = progname;

    proglen = strlen(progname) as libc::c_int;
    if proglen > 4 as libc::c_int
        && strcmp(
            progname
                .offset(proglen as isize)
                .offset(-(4 as libc::c_int as isize)),
            b"edit\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        progname = b"sudoedit\0" as *const u8 as *const libc::c_char;
        mode = 0x2 as libc::c_int;
        sudo_settings[16 as libc::c_int as usize].value =
            b"true\0" as *const u8 as *const libc::c_char;
        valid_flags = 0x800000 as libc::c_int;
    }

    if get_net_ifs(&mut cp) > 0 as libc::c_int {
        sudo_settings[18 as libc::c_int as usize].value = cp;
    }

    i = sudo_conf_max_groups_v1();

    if i != -1 {
        if asprintf(
            &mut cp as *mut *mut libc::c_char,
            b"%d\0" as *const u8 as *const libc::c_char,
            i,
        ) == -1 as libc::c_int
        {
            //define sudo_fatalx(U_("%s: %s"),__func__,U_("unable to allocate memory"));
            sudo_debug_printf!(
                SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"%s:%s\0" as *const u8 as *const libc::c_char
                ),
                function_name!(),
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"unable to allocate memory\0" as *const u8 as *const libc::c_char
                )
            );
            sudo_fatalx_nodebug_v1(
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
            //end of define
        }
        sudo_settings[19 as libc::c_int as usize].value = cp;
    }

    memset(
        &mut extra_env as *mut environment as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<environment>() as libc::c_ulong,
    );

    loop {
        ch = getopt_long(
            argc,
            argv,
            short_opts.as_ptr(),
            long_opts.as_mut_ptr(),
            0 as *mut libc::c_int,
        );
        if ch != -(1 as libc::c_int) {
            match ch {
                65 => {
                    tgetpass_flags |= 0x04 as libc::c_int;
                    continue;
                }
                98 => {
                    flags |= 0x10000 as libc::c_int;
                    continue;
                }
                66 => {
                    tgetpass_flags |= 0x20 as libc::c_int;
                    continue;
                }
                67 => {
                    if !optarg.is_null() {
                    } else {
                        __assert_fail(
                        b"optarg != NULL\0" as *const u8 as *const libc::c_char,
                        b"parse_args.c\0" as *const u8 as *const libc::c_char,
                        331 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8;81],
                            &[libc::c_char;81],
                        >(
                            b"int parse_args(int, char **, int *, char ***, struct sudo_settings **, char ***)\0",
                        ))
                            .as_ptr(),
                    );
                    }

                    if sudo_strtonum(
                        optarg,
                        3 as libc::c_int as libc::c_longlong,
                        2147483647 as libc::c_int as libc::c_longlong,
                        0 as *mut *const libc::c_char,
                    ) == 0 as libc::c_int as libc::c_longlong
                    {
                        //define sudo_warnx(U_("the argument to -C must be a number greater than or equal to 3"));
                        sudo_debug_printf!(
                            SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
                            sudo_warn_gettext_v1(
                                0 as *const libc::c_char,
                                b"the argument to -C must be a number greater than or equal to 3\0"
                                    as *const u8
                                    as *const libc::c_char
                            )
                        );
                        sudo_warn_nodebug_v1(sudo_warn_gettext_v1(
                            0 as *const libc::c_char,
                            b"the argument to -C must be a number greater than or equal to 3\0"
                                as *const u8 as *const libc::c_char,
                        ));
                        //end of define
                        usage(1);
                    }
                    sudo_settings[17 as libc::c_int as usize].value = optarg;
                    continue;
                } //end of match 67
                68 => {
                    continue;
                }
                69 => {
                    if optarg.is_null() {
                        sudo_settings[2 as libc::c_int as usize].value =
                            b"true\0" as *const u8 as *const libc::c_char;
                        flags |= 0x400000 as libc::c_int;
                    } else {
                        parse_env_list(&mut extra_env, optarg);
                    }
                    continue;
                }
                101 => {
                    if mode != 0 && mode != 0x2 as libc::c_int {
                        usage_excl(1 as libc::c_int);
                    }
                    mode = 0x2 as libc::c_int;
                    sudo_settings[16 as libc::c_int as usize].value =
                        b"true\0" as *const u8 as *const libc::c_char;
                    valid_flags = 0x800000 as libc::c_int;
                    continue;
                }
                103 => {
                    if !optarg.is_null() {
                    } else {
                        __assert_fail(
                        b"optarg != NULL\0" as *const u8 as *const libc::c_char,
                        b"parse_args.c\0" as *const u8 as *const libc::c_char,
                        370 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8;81],
                            &[libc::c_char;81],
                        >(
                            b"int parse_args(int, char **, int *, char ***, struct sudo_settings **, char ***)\0",
                        ))
                            .as_ptr(),
                    );
                    }
                    if *optarg as libc::c_int == '\u{0}' as i32 {
                        usage(1 as libc::c_int);
                    }
                    runas_group = optarg;
                    sudo_settings[3 as libc::c_int as usize].value = optarg;
                    continue;
                }
                72 => {
                    sudo_settings[4 as libc::c_int as usize].value =
                        b"true\0" as *const u8 as *const libc::c_char;
                    flags |= 0x100000 as libc::c_int;
                    continue;
                }
                104 => {
                    if optarg.is_null() {
                        if optind > 1 as libc::c_int
                            && *(*argv.offset((optind - 1 as libc::c_int) as isize))
                                .offset(0 as libc::c_int as isize)
                                as libc::c_int
                                == '-' as i32
                            && *(*argv.offset((optind - 1 as libc::c_int) as isize))
                                .offset(1 as libc::c_int as isize)
                                as libc::c_int
                                == 'h' as i32
                            && *(*argv.offset((optind - 1 as libc::c_int) as isize))
                                .offset(2 as libc::c_int as isize)
                                as libc::c_int
                                == '\u{0}' as i32
                            && !(optind < argc
                                && *(*argv.offset(optind as isize))
                                    .offset(0 as libc::c_int as isize)
                                    as libc::c_int
                                    != '/' as i32
                                && !(strchr(*argv.offset(optind as isize), '=' as i32)).is_null())
                            && !(*argv.offset(optind as isize)).is_null()
                            && *(*argv.offset(optind as isize)).offset(0 as libc::c_int as isize)
                                as libc::c_int
                                != '-' as i32
                        {
                            let fresh6 = optind;
                            optind = optind + 1;
                            sudo_settings[21 as libc::c_int as usize].value =
                                *argv.offset(fresh6 as isize);
                            continue;
                        } else {
                            if mode != 0 && mode != 0x40 as libc::c_int {
                                if strcmp(
                                    progname,
                                    b"sudoedit\0" as *const u8 as *const libc::c_char,
                                ) != 0 as libc::c_int
                                {
                                    usage_excl(1 as libc::c_int);
                                }
                            }
                            mode = 0x40 as libc::c_int;
                            valid_flags = 0 as libc::c_int;
                            continue;
                        }
                    }
                }
                256 => {}

                105 => {
                    sudo_settings[6 as libc::c_int as usize].value =
                        b"true\0" as *const u8 as *const libc::c_char;
                    flags |= 0x40000 as libc::c_int;
                    continue;
                }
                107 => {
                    sudo_settings[7 as libc::c_int as usize].value =
                        b"true\0" as *const u8 as *const libc::c_char;
                    continue;
                }
                75 => {
                    sudo_settings[7 as libc::c_int as usize].value =
                        b"true\0" as *const u8 as *const libc::c_char;
                    if mode != 0 && mode != 0x10 as libc::c_int {
                        usage_excl(1 as libc::c_int);
                    }
                    mode = 0x10 as libc::c_int;
                    valid_flags = 0 as libc::c_int;
                    continue;
                }
                108 => {
                    if mode != 0 {
                        if mode == 0x80 as libc::c_int {
                            flags |= 0x1000000 as libc::c_int;
                        } else {
                            usage_excl(1 as libc::c_int);
                        }
                    }
                    mode = 0x80 as libc::c_int;
                    valid_flags = 0x800000 as libc::c_int | 0x1000000 as libc::c_int;
                    continue;
                }
                110 => {
                    flags |= 0x800000 as libc::c_int;
                    sudo_settings[15 as libc::c_int as usize].value =
                        b"true\0" as *const u8 as *const libc::c_char;
                    continue;
                }
                80 => {
                    sudo_settings[14 as libc::c_int as usize].value =
                        b"true\0" as *const u8 as *const libc::c_char;
                    flags |= 0x200000 as libc::c_int;
                    continue;
                }
                112 => {
                    if !optarg.is_null() {
                    } else {
                        __assert_fail(
                        b"optarg != NULL\0" as *const u8 as *const libc::c_char,
                        b"parse_args.c\0" as *const u8 as *const libc::c_char,
                        441 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8;81],
                            &[libc::c_char;81],
                        >(
                            b"int parse_args(int, char **, int *, char ***, struct sudo_settings **, char ***)\0",
                        ))
                            .as_ptr(),
                    );
                    }
                    sudo_settings[8 as libc::c_int as usize].value = optarg;
                    continue;
                }

                114 => {
                    if !optarg.is_null() {
                    } else {
                        __assert_fail(
                        b"optarg != NULL\0" as *const u8 as *const libc::c_char,
                        b"parse_args.c\0" as *const u8 as *const libc::c_char,
                        446 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8;81],
                            &[libc::c_char;81],
                        >(
                            b"int parse_args(int, char **, int *, char ***, struct sudo_settings **, char ***)\0",
                        ))
                            .as_ptr(),
                    );
                    }
                    if *optarg as libc::c_int == '\u{0}' as i32 {
                        usage(1 as libc::c_int);
                    }
                    sudo_settings[9 as libc::c_int as usize].value = optarg;
                    continue;
                }

                116 => {
                    if !optarg.is_null() {
                    } else {
                        __assert_fail(
                        b"optarg != NULL\0" as *const u8 as *const libc::c_char,
                        b"parse_args.c\0" as *const u8 as *const libc::c_char,
                        452 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8;81],
                            &[libc::c_char;81],
                        >(
                            b"int parse_args(int, char **, int *, char ***, struct sudo_settings **, char ***)\0",
                        ))
                            .as_ptr(),
                    );
                    }
                    if *optarg as libc::c_int == '\u{0}' as i32 {
                        usage(1 as libc::c_int);
                    }
                    sudo_settings[10 as libc::c_int as usize].value = optarg;
                    continue;
                }

                84 => {
                    if !optarg.is_null() {
                    } else {
                        __assert_fail(
                        b"optarg != NULL\0" as *const u8 as *const libc::c_char,
                        b"parse_args.c\0" as *const u8 as *const libc::c_char,
                        460 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8;81],
                            &[libc::c_char;81],
                        >(
                            b"int parse_args(int, char **, int *, char ***, struct sudo_settings **, char ***)\0",
                        ))
                            .as_ptr(),
                    );
                    }
                    sudo_settings[22 as libc::c_int as usize].value = optarg;
                    continue;
                }

                83 => {
                    tgetpass_flags |= 0x2 as libc::c_int;
                    continue;
                }

                115 => {
                    sudo_settings[5 as libc::c_int as usize].value =
                        b"true\0" as *const u8 as *const libc::c_char;
                    flags |= 0x20000 as libc::c_int;
                    continue;
                }

                85 => {
                    if !optarg.is_null() {
                    } else {
                        __assert_fail(
                        b"optarg != NULL\0" as *const u8 as *const libc::c_char,
                        b"parse_args.c\0" as *const u8 as *const libc::c_char,
                        471 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8;81],
                            &[libc::c_char;81],
                        >(
                            b"int parse_args(int, char **, int *, char ***, struct sudo_settings **, char ***)\0",
                        ))
                            .as_ptr(),
                    );
                    }
                    if *optarg as libc::c_int == '\u{0}' as i32 {
                        usage(1 as libc::c_int);
                    }
                    list_user = optarg;
                    continue;
                }

                117 => {
                    if !optarg.is_null() {
                    } else {
                        __assert_fail(
                        b"optarg != NULL\0" as *const u8 as *const libc::c_char,
                        b"parse_args.c\0" as *const u8 as *const libc::c_char,
                        477 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8;81],
                            &[libc::c_char;81],
                        >(
                            b"int parse_args(int, char **, int *, char ***, struct sudo_settings **, char ***)\0",
                        ))
                            .as_ptr(),
                    );
                    }
                    if *optarg as libc::c_int == '\u{0}' as i32 {
                        usage(1 as libc::c_int);
                    }
                    runas_user = optarg;
                    sudo_settings[11 as libc::c_int as usize].value = optarg;
                    continue;
                }

                118 => {
                    if mode != 0 && mode != 0x4 as libc::c_int {
                        usage_excl(1 as libc::c_int);
                    }
                    mode = 0x4 as libc::c_int;
                    valid_flags = 0x800000 as libc::c_int;
                    continue;
                }

                86 => {
                    if mode != 0 && mode != 0x20 as libc::c_int {
                        usage_excl(1 as libc::c_int);
                    }
                    mode = 0x20 as libc::c_int;
                    valid_flags = 0 as libc::c_int;
                    continue;
                }

                _ => {
                    usage(1 as libc::c_int);
                    continue;
                }
            }

            if !optarg.is_null() {
            } else {
                __assert_fail(
                        b"optarg != NULL\0" as *const u8 as *const libc::c_char,
                        b"parse_args.c\0" as *const u8 as *const libc::c_char,
                        402 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8;81],
                            &[libc::c_char;81],
                        >(
                            b"int parse_args(int, char **, int *, char ***, struct sudo_settings **, char ***)\0",
                        ))
                            .as_ptr(),
                    );
            }
            if *optarg as libc::c_int == '\u{0}' as i32 {
                usage(1 as libc::c_int);
            }
            runas_user = optarg;
            sudo_settings[21 as libc::c_int as usize].value = optarg;
        } else {
            if !(!(optind > 1 as libc::c_int
                && *(*argv.offset((optind - 1 as libc::c_int) as isize))
                    .offset(0 as libc::c_int as isize) as libc::c_int
                    == '-' as i32
                && *(*argv.offset((optind - 1 as libc::c_int) as isize))
                    .offset(1 as libc::c_int as isize) as libc::c_int
                    == '-' as i32
                && *(*argv.offset((optind - 1 as libc::c_int) as isize))
                    .offset(2 as libc::c_int as isize) as libc::c_int
                    == '\u{0}' as i32)
                && (optind < argc
                    && *(*argv.offset(optind as isize)).offset(0 as libc::c_int as isize)
                        as libc::c_int
                        != '/' as i32
                    && !(strchr(*argv.offset(optind as isize), '=' as i32)).is_null()))
            {
                break;
            }
            env_insert(&mut extra_env, *argv.offset(optind as isize));
            optind += 1;
        }
    } //end of loop

    argc -= optind;
    argv = argv.offset(optind as isize);

    if mode == 0 {
        if !(sudo_settings[7 as libc::c_int as usize].value).is_null() {
            if argc == 0 as libc::c_int
                && flags & (0x20000 as libc::c_int | 0x40000 as libc::c_int) == 0
            {
                mode = 0x8 as libc::c_int;
                sudo_settings[7 as libc::c_int as usize].value = 0 as *const libc::c_char;
                valid_flags = 0 as libc::c_int;
            }
        }
        if mode == 0 {
            mode = 0x1 as libc::c_int;
        }
    }

    if argc > 0 as libc::c_int && mode == 0x80 as libc::c_int {
        mode = 0x100 as libc::c_int;
    }

    if flags & 0x40000 as libc::c_int != 0 {
        if flags & 0x20000 as libc::c_int != 0 {
            //define sudo_warnx(U_("you may not specify both the `-i' asn `-s' options"));
            sudo_debug_printf!(
                SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"you may not specify both the `-i' and `-s' options\0" as *const u8
                        as *const libc::c_char
                )
            );
            sudo_warn_nodebug_v1(sudo_warn_gettext_v1(
                0 as *const libc::c_char,
                b"you may not specify both the `-i' and `-s' options\0" as *const u8
                    as *const libc::c_char,
            ));
            //end of define;
            usage(1 as libc::c_int);
        }
        if flags & 0x400000 as libc::c_int != 0 {
            //define sudo_warnx(U_("you may not specify both the `-i' asn `-E' options"));
            sudo_debug_printf!(
                SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"you may not specify both the `-i' and `-E' options\0" as *const u8
                        as *const libc::c_char
                )
            );
            sudo_warn_nodebug_v1(sudo_warn_gettext_v1(
                0 as *const libc::c_char,
                b"you may not specify both the `-i' and `-E' options\0" as *const u8
                    as *const libc::c_char,
            ));
            //end of define;
            usage(1 as libc::c_int);
        }
        flags |= 0x20000 as libc::c_int;
    }

    if flags & valid_flags != flags {
        usage(1 as libc::c_int);
    }

    if mode == 0x2 as libc::c_int
        && (flags & 0x400000 as libc::c_int != 0
            || extra_env.env_len != 0 as libc::c_int as libc::c_ulong)
    {
        if mode & 0x400000 as libc::c_int != 0 {
            //define sudo_warnx(U_("the `-E' option is not valid in edit mode"));
            sudo_debug_printf!(
                SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"the `-E' option is not valid in edit mode\0" as *const u8
                        as *const libc::c_char
                )
            );
            sudo_warn_nodebug_v1(sudo_warn_gettext_v1(
                0 as *const libc::c_char,
                b"the `-E' option is not valid in edit mode\0" as *const u8 as *const libc::c_char,
            ));
            //end of define;
        }
        if extra_env.env_len != 0 as libc::c_int as libc::c_ulong {
            //define sudo_warnx(U_("you may not specify environment variables in edit mode"));
            sudo_debug_printf!(
                SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"you may not specify environment variables in edit mode\0" as *const u8
                        as *const libc::c_char
                )
            );
            sudo_warn_nodebug_v1(sudo_warn_gettext_v1(
                0 as *const libc::c_char,
                b"you may not specify environment variables in edit mode\0" as *const u8
                    as *const libc::c_char,
            ));
            //end of define;
        }
        usage(1 as libc::c_int);
    }

    if (!runas_user.is_null() || !runas_group.is_null())
        && mode
            & (0x2 as libc::c_int | 0x1 as libc::c_int | 0x100 as libc::c_int | 0x4 as libc::c_int)
            == 0
    {
        usage(1 as libc::c_int);
    }

    if !list_user.is_null() && mode != 0x80 as libc::c_int && mode != 0x100 as libc::c_int {
        //define sudo_warnx(U_("the `-U' option may only be used with the `-l' option"));
        sudo_debug_printf!(
            SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
            sudo_warn_gettext_v1(
                0 as *const libc::c_char,
                b"the `-U' option may only be used with the `-l' option\0" as *const u8
                    as *const libc::c_char
            )
        );
        sudo_warn_nodebug_v1(sudo_warn_gettext_v1(
            0 as *const libc::c_char,
            b"the `-U' option may only be used with the `-l' option\0" as *const u8
                as *const libc::c_char,
        ));
        //end of define;
        usage(1 as libc::c_int);
    }

    if tgetpass_flags & 0x2 as libc::c_int != 0 && tgetpass_flags & 0x4 as libc::c_int != 0 {
        //define sudo_warnx(U_("the `-A' asd `-S' options may not be used together"));
        sudo_debug_printf!(
            SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
            sudo_warn_gettext_v1(
                0 as *const libc::c_char,
                b"the `-A' and `-S' options may not be used together\0" as *const u8
                    as *const libc::c_char
            )
        );
        sudo_warn_nodebug_v1(sudo_warn_gettext_v1(
            0 as *const libc::c_char,
            b"the `-A' and `-S' options may not be used together\0" as *const u8
                as *const libc::c_char,
        ));
        //end of define;
        usage(1 as libc::c_int);
    }

    if argc == 0 as libc::c_int && mode == 0x2 as libc::c_int
        || argc > 0 as libc::c_int
            && mode & (0x1 as libc::c_int | 0x2 as libc::c_int | 0x100 as libc::c_int) == 0
    {
        usage(1 as libc::c_int);
    }

    if argc == 0 as libc::c_int && mode == 0x1 as libc::c_int && flags & 0x20000 as libc::c_int == 0
    {
        flags |= 0x80000 as libc::c_int | 0x20000 as libc::c_int;
        sudo_settings[13 as libc::c_int as usize].value =
            b"true\0" as *const u8 as *const libc::c_char;
    }

    sudo_settings[20 as libc::c_int as usize].value = sudo_conf_plugin_dir_path_v1();

    if mode == 0x40 as libc::c_int {
        help();
    }

    if flags & (0x20000 as libc::c_int | 0x40000 as libc::c_int) != 0
        && mode & 0x1 as libc::c_int != 0
    {
        let mut av: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        let mut cmnd: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut ac: libc::c_int = 1 as libc::c_int;
        if argc != 0 as libc::c_int {
            let mut src: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut dst: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut size: size_t = 0 as libc::c_int as size_t;

            av = argv;
            while !(*av).is_null() {
                size = (size as libc::c_ulong)
                    .wrapping_add((strlen(*av)).wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as size_t;
                av = av.offset(1);
            }

            if size == 0 as libc::c_int as libc::c_ulong || {
                cmnd = reallocarray(
                    0 as *mut libc::c_void,
                    size as size_t,
                    2 as libc::c_int as size_t,
                ) as *mut libc::c_char;
                cmnd.is_null()
            } {
                //define sudo_fatalx(U_(%s:%s),__func__,U_("unable to allocate memory"));
                sudo_debug_printf!(
                    SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"%s:%s\0" as *const u8 as *const libc::c_char
                    ),
                    function_name!(),
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"unable to allocate memory\0" as *const u8 as *const libc::c_char
                    )
                );
                sudo_fatalx_nodebug_v1(
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
                //end of define;
            }

            if !gc_add(2, av as *mut libc::c_void) {
                exit(1 as libc::c_int);
            };

            dst = cmnd;
            av = argv;
            while !(*av).is_null() {
                src = *av;
                while *src as libc::c_int != '\u{0}' as i32 {
                    if *(*__ctype_b_loc()).offset(*src as libc::c_uchar as libc::c_int as isize)
                        as libc::c_int
                        & 8 as libc::c_int as libc::c_ushort as libc::c_int
                        == 0
                        && *src as libc::c_int != '_' as i32
                        && *src as libc::c_int != '-' as i32
                        && *src as libc::c_int != '$' as i32
                    {
                        let fresh7 = dst;
                        dst = dst.offset(1);
                        *fresh7 = '\\' as i32 as libc::c_char;
                    }
                    let fresh8 = dst;
                    dst = dst.offset(1);
                    *fresh8 = *src;
                    src = src.offset(1);
                }
                let fresh9 = dst;
                dst = dst.offset(1);
                *fresh9 = ' ' as i32 as libc::c_char;
                av = av.offset(1);
            }
            if cmnd != dst {
                dst = dst.offset(-1);
            }

            *dst = '\u{0}' as i32 as libc::c_char;
            ac += 2 as libc::c_int;
        } //line 1716

        av = reallocarray(
            0 as *mut libc::c_void,
            (ac + 1 as libc::c_int) as size_t,
            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
        ) as *mut *mut libc::c_char;

        if av.is_null() {
            //define sudo_fatalx(U_(%s:%s),__func__,U_("unable to allocate memory"));
            sudo_debug_printf!(
                SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"%s:%s\0" as *const u8 as *const libc::c_char
                ),
                function_name!(),
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"unable to allocate memory\0" as *const u8 as *const libc::c_char
                )
            );
            sudo_fatalx_nodebug_v1(
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
            //end of define;
        }

        if !gc_add(2, av as *mut libc::c_void) {
            exit(1 as libc::c_int);
        }

        let ref mut fresh10 = *av.offset(0 as libc::c_int as isize);
        *fresh10 = user_details.shell as *mut libc::c_char;

        if !cmnd.is_null() {
            *av.offset(1 as libc::c_int as isize) =
                b"-c\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            *av.offset(2 as libc::c_int as isize) = cmnd;
        }

        *av.offset(ac as isize) = 0 as *mut libc::c_char;
        argv = av;
        argc = ac;
    } //1779

    if mode == 0x2 as libc::c_int {
        argc += 1;
        argv = argv.offset(-1);
        *argv.offset(0 as libc::c_int as isize) =
            b"sudoedit\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }

    *settingsp = sudo_settings.as_mut_ptr();
    *env_addp = extra_env.envp;
    *nargc = argc;
    *nargv = argv;

    //define debug_return_int(mode | flags);
    debug_return_int!(mode | flags);
    //end of define;
} //end of func

unsafe extern "C" fn env_insert(mut e: *mut environment, mut pair: *mut libc::c_char) {
    //->libc::c_void
    //define debug_decl(env_insert,SUDO_DEBUG_ARGS) 1<<6
    debug_decl!(env_insert, SUDO_DEBUG_ARGS);
    //end of define

    if (*e).env_len + 1 >= (*e).env_size {
        let mut tmp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;

        if (*e).env_size == 0 {
            (*e).env_size = 16;
        }
        tmp = reallocarray(
            (*e).envp as *mut libc::c_void,
            (*e).env_size,
            (2 as size_t).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as size_t),
        ) as *mut *mut libc::c_char;
        if tmp.is_null() {
            //define sudo_fatalx(U_(%s:%s),__func__,U_("unable to allocate memory"));
            sudo_debug_printf!(
                SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"%s:%s\0" as *const u8 as *const libc::c_char
                ),
                function_name!(),
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"unable to allocate memory\0" as *const u8 as *const libc::c_char
                )
            );
            sudo_fatalx_nodebug_v1(
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
            //end of define;
        }
        (*e).envp = tmp;
        (*e).env_size *= 2;
    }

    //let ref mut fresh 与　*fresh
    *((*e).envp).offset((*e).env_len.wrapping_add(1) as isize) = pair;
    *((*e).envp).offset((*e).env_len as isize) = 0 as *mut libc::c_char;

    //define debug_return
    debug_return!();
    //end of define
}

unsafe extern "C" fn env_set(
    mut e: *mut environment,
    mut var: *mut libc::c_char,
    mut val: *mut libc::c_char,
) {
    let mut pair: *mut libc::c_char = 0 as *mut libc::c_char;
    //define debug_decl(env_insert,SUDO_DEBUG_ARGS) 1<<6
    debug_decl!(env_set, SUDO_DEBUG_ARGS);
    //end of define

    pair = sudo_new_key_val_v1(var, val);
    if pair.is_null() {
        //define sudo_fatalx(U_(%s: %s),__func__,u_("unable toallocate memory"));
        sudo_debug_printf!(
            SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
            sudo_warn_gettext_v1(
                0 as *const libc::c_char,
                b"%s:%s\0" as *const u8 as *const libc::c_char
            ),
            function_name!(),
            sudo_warn_gettext_v1(
                0 as *const libc::c_char,
                b"unable to allocate memory\0" as *const u8 as *const libc::c_char
            )
        );
        sudo_fatalx_nodebug_v1(
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
        //end of define
    }
    env_insert(e, pair);

    //define debug_return
    debug_return!();
    //end of define
}

unsafe extern "C" fn parse_env_list(mut e: *mut environment, mut list: *mut libc::c_char) {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut last: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    //define debug_decl(env_insert,SUDO_DEBUG_ARGS) 1<<6
    debug_decl!(parse_env_list, SUDO_DEBUG_ARGS);
    //end of define

    cp = strtok_r(list, b",\0" as *const u8 as *const libc::c_char, &mut last);
    while !cp.is_null() {
        if !strchr(cp, '=' as i32).is_null() {
            //define sudo_warnx(U_("invalid environment variable name: %s"),cp);
            sudo_debug_printf!(
                SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"invalid environment variable name: %s\0" as *const u8 as *const libc::c_char
                ),
                cp
            );
            sudo_warn_nodebug_v1(
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"invalid environment variable name: %s\0" as *const u8 as *const libc::c_char,
                ),
                cp,
            );
            //end of define;

            //had not write ,line 661
            usage(1);
        }

        val = getenv(cp);
        if !val.is_null() {
            env_set(e, cp, val);
        }
        cp = strtok_r(
            0 as *mut libc::c_char,
            b",\0" as *const u8 as *const libc::c_char,
            &mut last,
        );
    }

    //define debug_return
    debug_return!();
    //end of define
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_lbuf {
    pub output: Option<unsafe extern "C" fn(*const libc::c_char) -> libc::c_int>,
    pub buf: *mut libc::c_char,
    pub continuation: *const libc::c_char,
    pub indent: libc::c_int,
    pub len: libc::c_int,
    pub size: libc::c_int,
    pub cols: libc::c_short,
    pub error: libc::c_short,
}

unsafe extern "C" fn usage_err(mut buf: *const libc::c_char) -> libc::c_int {
    return fputs(buf, stderr);
}

unsafe extern "C" fn usage_out(mut buf: *const libc::c_char) -> libc::c_int {
    return fputs(buf, stdout);
}

pub type sudo_gc_types = libc::c_uint;

#[no_mangle]
pub unsafe extern "C" fn usage(mut fatal: libc::c_int) {
    let mut lbuf: sudo_lbuf = sudo_lbuf {
        output: None,
        buf: 0 as *mut libc::c_char,
        continuation: 0 as *const libc::c_char,
        indent: 0,
        len: 0,
        size: 0,
        cols: 0,
        error: 0,
    };

    let mut uvec: [*mut libc::c_char; 6] = [0 as *mut libc::c_char; 6];
    let mut i: libc::c_int = 0;
    let mut ulen: libc::c_int = 0;

    if strcmp(
        sudo_getprogname(),
        b"sudoedit\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        uvec[0 as usize] = &*(b" -e [-AknS] [-r role] [-t type] [-C num] [-g group] [-h host] [-p prompt] [-T timeout] [-u user] file ...\0" as *const u8 as *const libc::c_char).offset(3 as isize) as *const libc::c_char as *mut libc::c_char;
        uvec[1 as usize] = 0 as *mut libc::c_char;
    } else {
        uvec[0 as usize] =
            b" -h | -K | -k | -V\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        uvec[1 as usize] = b" -v [-AknS] [-g group] [-h host] [-p prompt] [-u user]\0" as *const u8
            as *const libc::c_char as *mut libc::c_char;
        uvec[2 as usize] = b" -I [-AknS] [-g group] [-h host] [-p prompt] [-u user] [command]\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char;
        uvec[3 as usize] = b" [-AbEHknPS] [-r role] [-t type] [-C num] [-g group] [-h host] [-p prompt] [-T timeout] [-u user] [VAR=value] [-i|-s] [<command>]\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        uvec[4 as usize] = b" -e [-AknS] [-r role] [-t type] [-C num] [-g group] [-h host] [-p prompt] [-T timeout] [-u user] file ...\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        uvec[5 as usize] = 0 as *mut libc::c_char;
    }

    ulen = strlen(sudo_getprogname()) as libc::c_int + 8;
    sudo_lbuf_init_v1(
        &mut lbuf,
        if fatal != 0 {
            Some(usage_err as unsafe extern "C" fn(*const libc::c_char) -> libc::c_int)
        } else {
            Some(usage_out as unsafe extern "C" fn(*const libc::c_char) -> libc::c_int)
        },
        ulen,
        0 as *const libc::c_char,
        user_details.ts_cols,
    );

    i = 0 as libc::c_int;
    while !uvec[i as usize].is_null() {
        sudo_lbuf_append_v1(
            &mut lbuf as *mut sudo_lbuf,
            b"usage: %s%s\0" as *const u8 as *const libc::c_char,
            sudo_getprogname(),
            uvec[i as usize],
        );
        sudo_lbuf_print_v1(&mut lbuf);
        i += 1;
    }

    sudo_lbuf_destroy_v1(&mut lbuf);
    if fatal != 0 {
        exit(1);
    }
}

unsafe extern "C" fn usage_excl(mut fatal: libc::c_int) {
    //define debug_decl(env_insert,SUDO_DEBUG_ARGS) 1<<6
    debug_decl!(usage_excl, SUDO_DEBUG_ARGS);
    //end of define

    //dedine sudo_warnx(U_("Only one of the -e, -h, -i, -k, -l, -s, -v or -V options may be specified"));
    sudo_debug_printf!(
        SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
        sudo_warn_gettext_v1(
            0 as *const libc::c_char,
            b"Only one of the -e, -h, -i, -k, -l, -s, -v or -V options may be specified\0"
                as *const u8 as *const libc::c_char
        )
    );
    sudo_warn_nodebug_v1(sudo_warn_gettext_v1(
        0 as *const libc::c_char,
        b"Only one of the -e, -h, -i, -k, -l, -s, -v or -V options may be specified\0" as *const u8
            as *const libc::c_char,
    ));
    //end of define

    usage(fatal);
}

unsafe extern "C" fn help() {
    let mut lbuf: sudo_lbuf = sudo_lbuf {
        output: None,
        buf: 0 as *mut libc::c_char,
        continuation: 0 as *const libc::c_char,
        indent: 0,
        len: 0,
        size: 0,
        cols: 0,
        error: 0,
    };
    let mut indent: libc::c_int = 32;
    let mut pname: *const libc::c_char = sudo_getprogname();
    //define debug_decl(help,SUDO_DEBUG_ARGS) 1<<6
    debug_decl!(help, SUDO_DEBUG_ARGS);
    //end of define

    sudo_lbuf_init_v1(
        &mut lbuf,
        Some(usage_out as unsafe extern "C" fn(*const libc::c_char) -> libc::c_int),
        indent,
        0 as *const libc::c_char,
        user_details.ts_cols,
    );
    if strcmp(pname, b"sudoedit\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        sudo_lbuf_append_v1(
            &mut lbuf as *mut sudo_lbuf,
            dcgettext(
                0 as *const libc::c_char,
                b"%s - edit files as another user\n\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            pname,
        );
    } else {
        sudo_lbuf_append_v1(
            &mut lbuf as *mut sudo_lbuf,
            dcgettext(
                0 as *const libc::c_char,
                b"%s - execute a command as another user\n\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            pname,
        );
    }
    sudo_lbuf_print_v1(&mut lbuf);
    usage(0 as libc::c_int);

    sudo_lbuf_append_v1(
        &mut lbuf as *mut sudo_lbuf,
        dcgettext(
            0 as *const libc::c_char,
            b"\nOptions:\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );

    sudo_lbuf_append_v1(
        &mut lbuf as *mut sudo_lbuf,
        b"  -A, --askpass                 %s\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"use a helper program for password prompting\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );

    sudo_lbuf_append_v1(
        &mut lbuf as *mut sudo_lbuf,
        b"  -b, --background              %s\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"run command in the background\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );

    sudo_lbuf_append_v1(
        &mut lbuf as *mut sudo_lbuf,
        b"  -B, --bell                    %s\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"ring bell when prompting\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );

    sudo_lbuf_append_v1(
        &mut lbuf as *mut sudo_lbuf,
        b"  -C, --close-from=num          %s\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"close all file descriptors >= num\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );

    sudo_lbuf_append_v1(
        &mut lbuf as *mut sudo_lbuf,
        b"  -E, --preserve-env            %s\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"preserve user environment when running command\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );

    sudo_lbuf_append_v1(
        &mut lbuf as *mut sudo_lbuf,
        b"      --preserve-env=list       %s\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"preserve specific environment variables\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );

    sudo_lbuf_append_v1(
        &mut lbuf as *mut sudo_lbuf,
        b"  -e, --edit                    %s\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"edit files instead of running a command\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );

    sudo_lbuf_append_v1(
        &mut lbuf as *mut sudo_lbuf,
        b"  -g, --group=group             %s\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"run command as the specified group name or ID\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );

    sudo_lbuf_append_v1(
        &mut lbuf as *mut sudo_lbuf,
        b"  -H, --set-home                %s\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"set HOME variable to target user's home dir\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );

    sudo_lbuf_append_v1(
        &mut lbuf as *mut sudo_lbuf,
        b"  -h, --help                    %s\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"display help message and exit\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );

    sudo_lbuf_append_v1(
        &mut lbuf as *mut sudo_lbuf,
        b"  -h, --host=host               %s\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"run command on host (if supported by plugin)\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );

    sudo_lbuf_append_v1(
        &mut lbuf as *mut sudo_lbuf,
        b"  -i, --login                   %s\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"run login shell as the target user; a command may also be specified\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );

    sudo_lbuf_append_v1(
        &mut lbuf as *mut sudo_lbuf,
        b"  -K, --remove-timestamp        %s\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"remove timestamp file completely\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );

    sudo_lbuf_append_v1(
        &mut lbuf as *mut sudo_lbuf,
        b"  -k, --reset-timestamp         %s\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"invalidate timestamp file\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );

    sudo_lbuf_append_v1(
        &mut lbuf as *mut sudo_lbuf,
        b"  -l, --list                    %s\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"list user's privileges or check a specific command; use twice for longer format\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );

    sudo_lbuf_append_v1(
        &mut lbuf as *mut sudo_lbuf,
        b"  -n, --non-interactive         %s\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"non-interactive mode, no prompts are used\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );

    sudo_lbuf_append_v1(
        &mut lbuf as *mut sudo_lbuf,
        b"  -P, --preserve-groups         %s\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"preserve group vector instead of setting to target's\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );

    sudo_lbuf_append_v1(
        &mut lbuf as *mut sudo_lbuf,
        b"  -p, --prompt=prompt           %s\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"use the specified password prompt\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );

    sudo_lbuf_append_v1(
        &mut lbuf as *mut sudo_lbuf,
        b"  -r, --role=role               %s\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"create SELinux security context with specified role\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );

    sudo_lbuf_append_v1(
        &mut lbuf as *mut sudo_lbuf,
        b"  -S, --stdin                   %s\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"read password from standard input\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );

    sudo_lbuf_append_v1(
        &mut lbuf as *mut sudo_lbuf,
        b"  -s, --shell                   %s\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"run shell as the target user; a command may also be specified\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );

    sudo_lbuf_append_v1(
        &mut lbuf as *mut sudo_lbuf,
        b"  -t, --type=type               %s\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"create SELinux security context with specified type\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );

    sudo_lbuf_append_v1(
        &mut lbuf as *mut sudo_lbuf,
        b"  -T, --command-timeout=timeout %s\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"terminate command after the specified time limit\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );

    sudo_lbuf_append_v1(
        &mut lbuf as *mut sudo_lbuf,
        b"  -U, --other-user=user         %s\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"in list mode, display privileges for user\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );

    sudo_lbuf_append_v1(
        &mut lbuf as *mut sudo_lbuf,
        b"  -u, --user=user               %s\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"run command (or edit file) as specified user name or ID\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );

    sudo_lbuf_append_v1(
        &mut lbuf as *mut sudo_lbuf,
        b"  -V, --version                 %s\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"display version information and exit\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );

    sudo_lbuf_append_v1(
        &mut lbuf as *mut sudo_lbuf,
        b"  -v, --validate                %s\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"update user's timestamp without running a command\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );

    sudo_lbuf_append_v1(
        &mut lbuf as *mut sudo_lbuf,
        b"  --                            %s\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"stop processing command line arguments\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );

    sudo_lbuf_print_v1(&mut lbuf);
    sudo_lbuf_destroy_v1(&mut lbuf);

    sudo_debug_exit_int_v1(
        (*::std::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"help\0")).as_ptr(),
        b"parse_args.rs\0" as *const u8 as *const libc::c_char,
        line!() as libc::c_int,
        sudo_debug_subsys,
        0 as libc::c_int,
    );
    exit(0 as libc::c_int);
} //end of function help
