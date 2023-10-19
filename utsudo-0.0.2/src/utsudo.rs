
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
