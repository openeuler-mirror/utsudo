/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(
    non_camel_case_types,
    unreachable_code,
    unused_assignments,
    unused_mut,
    non_upper_case_globals,
    non_snake_case,
    clashing_extern_declarations,
    dead_code
)]

mod conversation;
mod copy_file;
mod env_hooks;
mod exec;
mod exec_common;
mod exec_monitor;
mod exec_nopty;
mod exec_pty;
mod get_pty;
mod hooks;
mod limits;
mod load_plugins;
mod net_ifs;
mod parse_args;
mod preserve_fds;
mod selinux;
mod signal;
mod struct_macro;
mod sudo_edit;
mod tcsetpgrp_nobg;
mod tgetpass;
mod ttyname;
mod utmp;

pub use hooks::*;
pub use struct_macro::*;
use utsudo_util::sudo_debug::*;
use utsudo_util::sudo_debug_macro::*;
use utsudo_util::*;

/* Initializer for instance index to indicate that debugging is not setup. */
// #define SUDO_DEBUG_INSTANCE_INITIALIZER	-1
pub const SUDO_DEBUG_INSTANCE_INITIALIZER: libc::c_int = -1;

/* API version major/minor */
// #define SUDO_API_VERSION_MAJOR 1
// #define SUDO_API_VERSION_MINOR 13
// #define SUDO_API_MKVERSION(x, y) (((x) << 16) | (y))
// #define SUDO_API_VERSION SUDO_API_MKVERSION(SUDO_API_VERSION_MAJOR, SUDO_API_VERSION_MINOR)
pub const SUDO_API_VERSION_MAJOR: libc::c_int = 1;
pub const SUDO_API_VERSION_MINOR: libc::c_int = 13;

/* Hook API version major/minor */
// #define SUDO_HOOK_VERSION_MAJOR	1
// #define SUDO_HOOK_VERSION_MINOR	0
// #define SUDO_HOOK_VERSION SUDO_API_MKVERSION(SUDO_HOOK_VERSION_MAJOR, SUDO_HOOK_VERSION_MINOR)
pub const SUDO_HOOK_VERSION_MAJOR: libc::c_int = 1;
pub const SUDO_HOOK_VERSION_MINOR: libc::c_int = 0;

// #define	X_OK	1		/* Test for execute permission.  */
pub const X_OK: libc::c_int = 1;

/* Values of sudo_conf_group_source() */
// #define GROUP_SOURCE_STATIC	1
// #define GROUP_SOURCE_DYNAMIC	2
pub const GROUP_SOURCE_STATIC: libc::c_int = 1;
pub const GROUP_SOURCE_DYNAMIC: libc::c_int = 2;

/* Define to the max length of a uid_t in string context (excluding the NUL).
 */
//    #define MAX_UID_T_LEN 10
pub const MAX_UID_T_LEN: libc::c_int = 10;

pub const _SC_NGROUPS_MAX: libc::c_int = 3;

// #define NGROUPS_MAX    65536	/* supplemental group IDs are available */
pub const NGROUPS_MAX: libc::c_int = 65536;

/* Flags for sudo_conf_read() */
// #define SUDO_CONF_DEBUG		0x01
// #define SUDO_CONF_PATHS		0x02
// #define SUDO_CONF_PLUGINS	0x04
// #define SUDO_CONF_SETTINGS	0x08
// #define SUDO_CONF_ALL		0x0f
pub const SUDO_CONF_DEBUG: libc::c_int = 0x01;
pub const SUDO_CONF_PATHS: libc::c_int = 0x02;
pub const SUDO_CONF_PLUGINS: libc::c_int = 0x04;
pub const SUDO_CONF_SETTINGS: libc::c_int = 0x08;
pub const SUDO_CONF_ALL: libc::c_int = 0x0f;

/* We define these the same for all machines.
Changes from this to the outside world should be done in `_exit'.  */
//    #define	EXIT_FAILURE	1	/* Failing exit status.  */
//    #define	EXIT_SUCCESS	0	/* Successful exit status.  */
pub const EXIT_FAILURE: libc::c_int = 1;
pub const EXIT_SUCCESS: libc::c_int = 0;

/*
 * Various modes sudo can be in (based on arguments) in hex
 */
//  #define MODE_RUN		0x00000001
//  #define MODE_EDIT		0x00000002
//  #define MODE_VALIDATE		0x00000004
//  #define MODE_INVALIDATE		0x00000008
//  #define MODE_KILL		0x00000010
//  #define MODE_VERSION		0x00000020
//  #define MODE_HELP		0x00000040
//  #define MODE_LIST		0x00000080
//  #define MODE_CHECK		0x00000100
//  #define MODE_MASK		0x0000ffff
pub const MODE_RUN: libc::c_int = 0x00000001; // 1
pub const MODE_EDIT: libc::c_int = 0x00000002; // 2
pub const MODE_VALIDATE: libc::c_int = 0x00000004; // 4
pub const MODE_INVALIDATE: libc::c_int = 0x00000008; // 8
pub const MODE_KILL: libc::c_int = 0x00000010; // 16
pub const MODE_VERSION: libc::c_int = 0x00000020; // 32
pub const MODE_HELP: libc::c_int = 0x00000040; // 64
pub const MODE_LIST: libc::c_int = 0x00000080; // 128
pub const MODE_CHECK: libc::c_int = 0x00000100; // 256
pub const MODE_MASK: libc::c_int = 0x0000ffff; //  65535

pub const MODE_VALIDATE_MODE_INVALIDATE: libc::c_int = MODE_VALIDATE | MODE_INVALIDATE;
pub const MODE_CHECK_MODE_INVALIDATE: libc::c_int = MODE_CHECK | MODE_INVALIDATE;
pub const MODE_LIST_MODE_INVALIDATE: libc::c_int = MODE_LIST | MODE_INVALIDATE;

/* Mode flags */
/* XXX - prune this */
// #define MODE_BACKGROUND		0x00010000
// #define MODE_SHELL		0x00020000
// #define MODE_LOGIN_SHELL	0x00040000
// #define MODE_IMPLIED_SHELL	0x00080000
// #define MODE_RESET_HOME		0x00100000
// #define MODE_PRESERVE_GROUPS	0x00200000
// #define MODE_PRESERVE_ENV	0x00400000
// #define MODE_NONINTERACTIVE	0x00800000
// #define MODE_LONG_LIST		0x01000000

pub const MODE_BACKGROUND: libc::c_int = 0x00010000;
pub const MODE_SHELL: libc::c_int = 0x00020000;
pub const MODE_LOGIN_SHELL: libc::c_int = 0x00040000;
pub const MODE_IMPLIED_SHELL: libc::c_int = 0x00080000;
pub const MODE_RESET_HOME: libc::c_int = 0x00100000;
pub const MODE_PRESERVE_GROUPS: libc::c_int = 0x00200000;
pub const MODE_PRESERVE_ENV: libc::c_int = 0x00400000;
pub const MODE_NONINTERACTIVE: libc::c_int = 0x00800000;
pub const MODE_LONG_LIST: libc::c_int = 0x01000000;

// let sudo_api_mkversion_1_0: libc::c_uint = SUDO_API_MKVERSION!(1, 0);
// let sudo_api_mkversion_1_1: libc::c_uint = SUDO_API_MKVERSION!(1, 1);
pub const sudo_api_mkversion_1_0: libc::c_uint = SUDO_API_MKVERSION!(1, 0);
pub const sudo_api_mkversion_1_1: libc::c_uint = SUDO_API_MKVERSION!(1, 1);

#[macro_export]
macro_rules! SUDO_API_VERSION {
    () => {
        SUDO_API_MKVERSION!(SUDO_API_VERSION_MAJOR, SUDO_API_VERSION_MINOR)
    };
}

// #define SUDO_HOOK_VERSION SUDO_API_MKVERSION(SUDO_HOOK_VERSION_MAJOR, SUDO_HOOK_VERSION_MINOR)
#[macro_export]
macro_rules! SUDO_HOOK_VERSION {
    () => {
        SUDO_API_MKVERSION!(SUDO_HOOK_VERSION_MAJOR, SUDO_HOOK_VERSION_MINOR)
    };
}

// #define SET_STRING(s, n)                                                   \
// 	if (strncmp(s, info[i], sizeof(s) - 1) == 0 && info[i][sizeof(s) - 1]) \
// 	{                                                                      \
// 		details->n = info[i] + sizeof(s) - 1;                              \
// 		break;                                                             \
// 	}
#[macro_export]
macro_rules! SET_STRING {
    ($s:expr, $details_n:expr, $len:expr, $info_i:expr) => {
        if strncmp(
            $s,
            $info_i,
            (::std::mem::size_of::<[libc::c_char; $len]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0
            && *($info_i).offset(
                (::std::mem::size_of::<[libc::c_char; $len]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) as libc::c_int
                != 0
        {
            $details_n = ($info_i)
                .offset(::std::mem::size_of::<[libc::c_char; $len]>() as libc::c_ulong as isize)
                .offset(-(1 as libc::c_int as isize));
            break;
        }
    };
}

// #define SET_FLAG(s, n)                                                  \
// 	if (strncmp(s, info[i], sizeof(s) - 1) == 0)                        \
// 	{                                                                   \
// 		switch (sudo_strtobool(info[i] + sizeof(s) - 1))                \
// 		{                                                               \
// 		case true:                                                      \
// 			SET(details->flags, n);                                     \
// 			break;                                                      \
// 		case false:                                                     \
// 			CLR(details->flags, n);                                     \
// 			break;                                                      \
// 		default:                                                        \
// 			sudo_debug_printf(SUDO_DEBUG_ERROR,                         \
// 							  "invalid boolean value for %s", info[i]); \
// 			break;                                                      \
// 		}                                                               \
// 		break;                                                          \
// 	}
#[macro_export]
macro_rules! SET_FLAG {
    ($s:expr, $n:expr, $len:expr, $info_i:expr, $details_flags:expr) => {
        if strncmp(
            $s,
            $info_i,
            (::std::mem::size_of::<[libc::c_char; $len]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0 as libc::c_int
        {
            // 为了可以中途跳出，加一个循环
            loop {
                match sudo_strtobool_v1(
                    ($info_i)
                        .offset(
                            ::std::mem::size_of::<[libc::c_char; $len]>() as libc::c_ulong as isize,
                        )
                        .offset(-(1 as libc::c_int as isize)),
                ) {
                    1 => {
                        SET!($details_flags, $n);
                        break;
                    }
                    0 => {
                        CLR!($details_flags, $n);
                        break;
                    }
                    _ => {
                        sudo_debug_printf!(
                            SUDO_DEBUG_ERROR,
                            b"invalid boolean value for %s\0" as *const u8 as *const libc::c_char,
                            $info_i
                        );
                        break;
                    }
                } // !match
                break;
            } //  ! if
        } // ! loop
    };
}

// #define	_PATH_DEVNULL	"/dev/null"
#[macro_export]
macro_rules! _PATH_DEVNULL {
    () => {
        b"/dev/null\0" as *const u8 as *const libc::c_char
    };
}

/* Define to the full name of this package. */
// #define PACKAGE_NAME "utsudo"
#[macro_export]
macro_rules! PACKAGE_NAME {
    () => {
        (b"utsudo\0" as *const u8 as *const libc::c_char)
    };
}

#[macro_export]
macro_rules! LOCALEDIR {
    () => {
        (b"/usr/share/locale\0" as *const u8 as *const libc::c_char)
    };
}

/* Define to the version of this package. */
// #define PACKAGE_VERSION "0.0.4"
#[macro_export]
macro_rules! PACKAGE_VERSION {
    () => {
        (b"0.0.4\0" as *const u8 as *const libc::c_char)
    };
}

#[macro_export]
macro_rules! CONFIGURE_ARGS {
    () => {
        (b"--build=***64-uos-linux-gnu
        --host=***64-uos-linux-gnu
        --program-prefix=
        --disable-dependency-tracking
        --prefix=/usr
        --exec-prefix=/usr
        --bindir=/usr/bin
        --sbindir=/usr/sbin
        --sysconfdir=/etc
        --datadir=/usr/share
        --includedir=/usr/include
        --libdir=/usr/lib64
        --libexecdir=/usr/libexec
        --localstatedir=/var
        --sharedstatedir=/var/lib
        --mandir=/usr/share/man
        --infodir=/usr/share/info
        --prefix=/usr
        --sbindir=/usr/sbin
        --libdir=/usr/lib64
        --docdir=/usr/share/doc/sudo
        --disable-root-mailer
        --with-logging=syslog
        --with-logfac=authpriv
        --with-pam
        --with-pam-login
        --with-editor=/bin/vi
        --with-env-editor
        --with-ignore-dot
        --with-tty-tickets
        --with-ldap
        --with-ldap-conf-file=/etc/sudo-ldap.conf
        --with-selinux
        --with-passprompt=[utsudo] password for %p:
        --with-linux-audit
        --with-sssd\0" as *const u8 as *const libc::c_char)
    };
}

// # define WCOREDUMP(status)	__WCOREDUMP (status)
/* Nonzero if STATUS indicates the child dumped core.  */
// #define	__WCOREDUMP(status)	((status) & __WCOREFLAG)
// #define	__WCOREFLAG		0x80
#[macro_export]
macro_rules! WCOREDUMP {
    ($status:expr) => {
        (($status) & 0x80)
    };
}

#[link(name = "utsudo_variadic")]
extern "C" {
    fn umask(__mask: __mode_t) -> __mode_t;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn asprintf(__ptr: *mut *mut libc::c_char, __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn reallocarray(__ptr: *mut libc::c_void, __nmemb: size_t, __size: size_t)
        -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn getpid() -> __pid_t;
    fn getppid() -> __pid_t;
    fn getpgid(__pid: __pid_t) -> __pid_t;
    fn getsid(__pid: __pid_t) -> __pid_t;
    fn getuid() -> __uid_t;
    fn geteuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn getegid() -> __gid_t;
    fn getgroups(__size: libc::c_int, __list: *mut __gid_t) -> libc::c_int;
    fn setuid(__uid: __uid_t) -> libc::c_int;
    fn setgid(__gid: __gid_t) -> libc::c_int;
    fn setegid(__gid: __gid_t) -> libc::c_int;
    fn tcgetpgrp(__fd: libc::c_int) -> __pid_t;
    fn __errno_location() -> *mut libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn tzset();
    fn is_selinux_enabled() -> libc::c_int;
    fn sudo_debug_exit_bool_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        subsys: libc::c_int,
        ret: bool,
    );
    fn sudo_warn_set_conversation_v1(
        conv: Option<
            unsafe extern "C" fn(
                libc::c_int,
                *const sudo_conv_message,
                *mut sudo_conv_reply,
                *mut sudo_conv_callback,
            ) -> libc::c_int,
        >,
    );
    fn sudo_conf_read_v1(conf_file: *const libc::c_char, conf_types: libc::c_int) -> libc::c_int;
    fn sudo_conf_debug_files_v1(progname: *const libc::c_char) -> *mut sudo_conf_debug_file_list;
    fn sudo_conf_disable_coredump_v1() -> bool;
    fn sudo_conf_group_source_v1() -> libc::c_int;
    fn sudo_conf_max_groups_v1() -> libc::c_int;
    fn sudo_debug_get_active_instance_v1() -> libc::c_int;
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn sudo_debug_register_v1(
        program: *const libc::c_char,
        subsystems: *const *const libc::c_char,
        ids: *mut libc::c_uint,
        debug_files: *mut sudo_conf_debug_file_list,
    ) -> libc::c_int;
    fn sudo_debug_set_active_instance_v1(inst: libc::c_int) -> libc::c_int;
    fn sudo_gethostname_v1() -> *mut libc::c_char;
    fn sudo_parse_gids_v1(
        gidstr: *const libc::c_char,
        basegid: *const gid_t,
        gidsp: *mut *mut gid_t,
    ) -> libc::c_int;
    fn sudo_getgrouplist2_v1(
        name: *const libc::c_char,
        basegid: gid_t,
        groupsp: *mut *mut gid_t,
        ngroupsp: *mut libc::c_int,
    ) -> libc::c_int;
    fn sudo_new_key_val_v1(
        key: *const libc::c_char,
        value: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn initprogname(_: *const libc::c_char);
    fn sudo_setgroups_v1(ngids: libc::c_int, gids: *const gid_t) -> libc::c_int;
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
    fn sudo_strtoid_v2(str: *const libc::c_char, errstr: *mut *const libc::c_char) -> id_t;
    fn sudo_strtomode_v1(cp: *const libc::c_char, errstr: *mut *const libc::c_char) -> libc::c_int;
    fn sudo_get_ttysize_v1(rowp: *mut libc::c_int, colp: *mut libc::c_int);
    fn sudo_execute(details: *mut command_details, cstat: *mut command_status) -> libc::c_int;
    fn parse_args(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        nargc: *mut libc::c_int,
        nargv: *mut *mut *mut libc::c_char,
        settingsp: *mut *mut sudo_settings,
        env_addp: *mut *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn sudo_fatalx_nodebug_v1(fmt: *const libc::c_char, _: ...) -> !;
    fn setlocale(__category: libc::c_int, __locale: *const libc::c_char) -> *mut libc::c_char;
    fn sudo_warnx_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_warn_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_edit(details: *mut command_details) -> libc::c_int;
    fn usage(_: libc::c_int);
    fn deregister_hook(hook: *mut sudo_hook) -> libc::c_int;
    fn getenv_unhooked(name: *const libc::c_char) -> *mut libc::c_char;
    fn get_process_ttyname(name: *mut libc::c_char, namelen: size_t) -> *mut libc::c_char;
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn sudo_fatal_nodebug_v1(fmt: *const libc::c_char, _: ...) -> !;
    fn sudo_pw_dup(pw: *const passwd) -> *mut passwd;
    fn sudo_getprogname() -> *const libc::c_char;
    fn init_signals();
    fn save_signals();
    fn add_preserved_fd(pfds: *mut preserved_fd_list, fd: libc::c_int) -> libc::c_int;
    fn parse_preserved_fds(pfds: *mut preserved_fd_list, fdstr: *const libc::c_char);
    fn disable_coredump();
    fn unlimit_sudo();
    fn sudo_conversation(
        num_msgs: libc::c_int,
        msgs: *const sudo_conv_message,
        replies: *mut sudo_conv_reply,
        callback: *mut sudo_conv_callback,
    ) -> libc::c_int;
    fn sudo_conversation_1_7(
        num_msgs: libc::c_int,
        msgs: *const sudo_conv_message,
        replies: *mut sudo_conv_reply,
    ) -> libc::c_int;
    fn sudo_conversation_printf(
        msg_type: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sudo_load_plugins(
        policy_plugin_0: *mut plugin_container,
        io_plugins_0: *mut plugin_container_list,
    ) -> bool;
}

pub type sudo_gc_types = libc::c_uint;
pub const GC_PTR: sudo_gc_types = 2;
pub const GC_VECTOR: sudo_gc_types = 1;
pub const GC_UNKNOWN: sudo_gc_types = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
    pub generic: *mut generic_plugin,
    pub policy: *mut policy_plugin,
    pub policy_1_0: *mut policy_plugin_1_0,
    pub io: *mut io_plugin,
    pub io_1_0: *mut io_plugin_1_0,
    pub io_1_1: *mut io_plugin_1_1,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct plugin_container {
    pub entries: C2RustUnnamed_14,
    pub debug_files: *mut sudo_conf_debug_file_list,
    pub name: *mut libc::c_char,
    pub path: *mut libc::c_char,
    pub options: *mut *mut libc::c_char,
    pub handle: *mut libc::c_void,
    pub debug_instance: libc::c_int,
    pub u: C2RustUnnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub tqe_next: *mut plugin_container,
    pub tqe_prev: *mut *mut plugin_container,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plugin_container_list {
    pub tqh_first: *mut plugin_container,
    pub tqh_last: *mut *mut plugin_container,
}
#[no_mangle]
pub static mut policy_plugin: plugin_container = plugin_container {
    entries: C2RustUnnamed_14 {
        tqe_next: 0 as *const plugin_container as *mut plugin_container,
        tqe_prev: 0 as *const *mut plugin_container as *mut *mut plugin_container,
    },
    debug_files: 0 as *const sudo_conf_debug_file_list as *mut sudo_conf_debug_file_list,
    name: 0 as *const libc::c_char as *mut libc::c_char,
    path: 0 as *const libc::c_char as *mut libc::c_char,
    options: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    handle: 0 as *const libc::c_void as *mut libc::c_void,
    debug_instance: 0,
    u: C2RustUnnamed_13 {
        generic: 0 as *const generic_plugin as *mut generic_plugin,
    },
};
#[no_mangle]
pub static mut io_plugins: plugin_container_list = plugin_container_list {
    tqh_first: 0 as *const plugin_container as *mut plugin_container,
    tqh_last: 0 as *const *mut plugin_container as *mut *mut plugin_container,
};
#[no_mangle]
pub static mut user_details: user_details = user_details {
    pid: 0,
    ppid: 0,
    pgid: 0,
    tcpgid: 0,
    sid: 0,
    uid: 0,
    euid: 0,
    gid: 0,
    egid: 0,
    username: 0 as *const libc::c_char,
    cwd: 0 as *const libc::c_char,
    tty: 0 as *const libc::c_char,
    host: 0 as *const libc::c_char,
    shell: 0 as *const libc::c_char,
    groups: 0 as *const gid_t as *mut gid_t,
    ngroups: 0,
    ts_rows: 0,
    ts_cols: 0,
};
#[no_mangle]
pub static mut list_user: *const libc::c_char = 0 as *const libc::c_char;

#[no_mangle]
pub static mut sudo_debug_instance: libc::c_int = -(1 as libc::c_int);
static mut command_details: command_details = command_details {
    uid: 0,
    euid: 0,
    gid: 0,
    egid: 0,
    umask: 0,
    priority: 0,
    timeout: 0,
    ngroups: 0,
    closefrom: 0,
    flags: 0,
    execfd: 0,
    preserved_fds: preserved_fd_list {
        tqh_first: 0 as *const preserved_fd as *mut preserved_fd,
        tqh_last: 0 as *const *mut preserved_fd as *mut *mut preserved_fd,
    },
    pw: 0 as *const passwd as *mut passwd,
    groups: 0 as *const gid_t as *mut gid_t,
    command: 0 as *const libc::c_char,
    cwd: 0 as *const libc::c_char,
    login_class: 0 as *const libc::c_char,
    chroot: 0 as *const libc::c_char,
    selinux_role: 0 as *const libc::c_char,
    selinux_type: 0 as *const libc::c_char,
    utmp_user: 0 as *const libc::c_char,
    tty: 0 as *const libc::c_char,
    argv: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    envp: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
};
static mut sudo_mode: libc::c_int = 0;

#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
        #[cfg(target_arch = "x86_64")]
        return __xstat(1 as libc::c_int, __path, __statbuf);
        #[cfg(not(target_arch = "x86_64"))]
        return __xstat(0 as libc::c_int, __path, __statbuf); 
}

unsafe extern "C" fn iolog_open(
    mut plugin: *mut plugin_container,
    mut settings: *mut sudo_settings,
    mut user_info: *const *mut libc::c_char,
    mut command_info: *const *mut libc::c_char,
    mut argc: libc::c_int,
    mut argv: *const *mut libc::c_char,
    mut user_env: *const *mut libc::c_char,
) -> libc::c_int {
    let mut plugin_settings: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut ret: libc::c_int = 0;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_PCOMM);

    /* Convert struct sudo_settings to plugin_settings[] */
    plugin_settings = format_plugin_settings(plugin, settings);
    if plugin_settings.is_null() {
        sudo_warnx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr(),
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
        debug_return_int!(-(1 as libc::c_int));
    }

    /*
     * Backwards compatibility for older API versions
     */
    sudo_debug_set_active_instance_v1((*plugin).debug_instance);
    match (*(*plugin).u.generic).version {
        sudo_api_mkversion_1_0 => {
            ret = ((*(*plugin).u.io_1_0).open).expect("non-null function pointer")(
                (*(*plugin).u.io_1_0).version,
                Some(
                    sudo_conversation_1_7
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const sudo_conv_message,
                            *mut sudo_conv_reply,
                        ) -> libc::c_int,
                ),
                Some(
                    sudo_conversation_printf
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const libc::c_char,
                            ...
                        ) -> libc::c_int,
                ),
                plugin_settings as *const *mut libc::c_char,
                user_info,
                argc,
                argv,
                user_env,
            );
        }
        sudo_api_mkversion_1_1 => {
            ret = ((*(*plugin).u.io_1_1).open).expect("non-null function pointer")(
                (*(*plugin).u.io_1_1).version,
                Some(
                    sudo_conversation_1_7
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const sudo_conv_message,
                            *mut sudo_conv_reply,
                        ) -> libc::c_int,
                ),
                Some(
                    sudo_conversation_printf
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const libc::c_char,
                            ...
                        ) -> libc::c_int,
                ),
                plugin_settings as *const *mut libc::c_char,
                user_info,
                command_info,
                argc,
                argv,
                user_env,
            );
        }
        _ => {
            ret = ((*(*plugin).u.io).open).expect("non-null function pointer")(
                SUDO_API_VERSION!() as libc::c_uint,
                Some(
                    sudo_conversation
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const sudo_conv_message,
                            *mut sudo_conv_reply,
                            *mut sudo_conv_callback,
                        ) -> libc::c_int,
                ),
                Some(
                    sudo_conversation_printf
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const libc::c_char,
                            ...
                        ) -> libc::c_int,
                ),
                plugin_settings as *const *mut libc::c_char,
                user_info,
                command_info,
                argc,
                argv,
                user_env,
                (*plugin).options as *const *mut libc::c_char,
            );
        }
    } // !  match (*(*plugin).u.generic).version

    /* Stash plugin debug instance ID if set in open() function. */
    (*plugin).debug_instance = sudo_debug_get_active_instance_v1();
    sudo_debug_set_active_instance_v1(sudo_debug_instance);

    debug_return_int!(ret);
}
unsafe extern "C" fn iolog_close(
    mut plugin: *mut plugin_container,
    mut exit_status: libc::c_int,
    mut error_code: libc::c_int,
) {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_PCOMM);

    if ((*(*plugin).u.io).close).is_some() {
        sudo_debug_set_active_instance_v1((*plugin).debug_instance);
        ((*(*plugin).u.io).close).expect("non-null function pointer")(exit_status, error_code);
        sudo_debug_set_active_instance_v1(sudo_debug_instance);
    }
    debug_return!();
}

unsafe extern "C" fn iolog_show_version(
    mut plugin: *mut plugin_container,
    mut verbose: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_PCOMM);

    if ((*(*plugin).u.io).show_version).is_none() {
        debug_return_int!(true as libc::c_int);
    }
    sudo_debug_set_active_instance_v1((*plugin).debug_instance);
    ret = ((*(*plugin).u.io).show_version).expect("non-null function pointer")(verbose);
    sudo_debug_set_active_instance_v1(sudo_debug_instance);

    debug_return_int!(ret);
}

/*
 * Remove the specified I/O logging plugin from the io_plugins list.
 * Deregisters any hooks before unlinking, then frees the container.
 */
unsafe extern "C" fn iolog_unlink(mut plugin: *mut plugin_container) {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_PCOMM);

    /* Deregister hooks, if any. */
    if (*(*plugin).u.io).version >= SUDO_API_MKVERSION!(1, 2) as libc::c_uint {
        if ((*(*plugin).u.io).deregister_hooks).is_some() {
            sudo_debug_set_active_instance_v1((*plugin).debug_instance);
            ((*(*plugin).u.io).deregister_hooks).expect("non-null function pointer")(
                SUDO_HOOK_VERSION!(),
                Some(deregister_hook as unsafe extern "C" fn(*mut sudo_hook) -> libc::c_int),
            );
            sudo_debug_set_active_instance_v1(sudo_debug_instance);
        }
    }

    /* Remove from io_plugins list and free. */
    if !((*plugin).entries.tqe_next).is_null() {
        (*(*plugin).entries.tqe_next).entries.tqe_prev = (*plugin).entries.tqe_prev;
    } else {
        io_plugins.tqh_last = (*plugin).entries.tqe_prev;
    }
    *(*plugin).entries.tqe_prev = (*plugin).entries.tqe_next;
    free_plugin_container(plugin, 1 as libc::c_int != 0);

    debug_return!();
}

unsafe extern "C" fn free_plugin_container(mut plugin: *mut plugin_container, mut ioplugin: bool) {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_PLUGIN);
    free((*plugin).path as *mut libc::c_void);
    free((*plugin).name as *mut libc::c_void);
    if !((*plugin).options).is_null() {
        let mut i: libc::c_int = 0 as libc::c_int;
        while !(*((*plugin).options).offset(i as isize)).is_null() {
            let fresh11 = i;
            i = i + 1;
            free(*((*plugin).options).offset(fresh11 as isize) as *mut libc::c_void);
        }
        free((*plugin).options as *mut libc::c_void);
    }

    if ioplugin {
        free(plugin as *mut libc::c_void);
    }

    debug_return!();
}


unsafe extern "C" fn gc_init() {}

pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    let mut vars: Vec<*mut libc::c_char> = Vec::new();
    for (var_name, var_value) in ::std::env::vars() {
        let var: String = format!("{}={}", var_name, var_value);
        vars.push(
            (::std::ffi::CString::new(var))
                .expect("Failed to convert environment variable into CString.")
                .into_raw(),
        );
    }
    vars.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
            vars.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}

unsafe extern "C" fn run_static_initializers() {
    io_plugins = {
        let mut init = plugin_container_list {
            tqh_first: 0 as *mut plugin_container,
            tqh_last: &mut io_plugins.tqh_first,
        };
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
