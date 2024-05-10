/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(
    unused_variables,
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    clashing_extern_declarations,
    unreachable_code
)]

use crate::struct_macro::*;
use utsudo_util::sudo_debug::*;
use utsudo_util::sudo_debug_macro::*;
use utsudo_util::*;

#[link(name = "selinux")]
#[link(name = "audit")]

extern "C" {
    fn audit_open() -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn asprintf(__ptr: *mut *mut libc::c_char, __fmt: *const libc::c_char, ...) -> libc::c_int;
    fn audit_log_user_message(
        audit_fd: libc::c_int,
        type_0: libc::c_int,
        message: *const libc::c_char,
        hostname: *const libc::c_char,
        addr: *const libc::c_char,
        tty: *const libc::c_char,
        result: libc::c_int,
    ) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn fgetfilecon(fd: libc::c_int, con: *mut *mut libc::c_char) -> libc::c_int;
    fn freecon(con: *mut libc::c_char);
    fn strcmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn fsetfilecon(fd: libc::c_int, con: *const libc::c_char) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn string_to_security_class(name: *const libc::c_char) -> security_class_t;
    fn security_compute_relabel(
        scon: *const libc::c_char,
        tcon: *const libc::c_char,
        tclass: security_class_t,
        newcon: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn context_free(_: context_t) -> libc::c_void;
    fn get_default_type(role: *const libc::c_char, types: *mut *mut libc::c_char) -> libc::c_int;
    fn context_new(_: *const libc::c_char) -> context_t;
    fn context_role_set(_: context_t, _: *const libc::c_char) -> libc::c_int;
    fn context_type_set(_: context_t, _: *const libc::c_char) -> libc::c_int;
    fn security_check_context(con: *const libc::c_char) -> libc::c_int;
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    fn context_str(_: context_t) -> *mut libc::c_char;
    fn getprevcon(con: *mut *mut libc::c_char) -> libc::c_int;
    fn security_getenforce() -> libc::c_int;
    fn setexeccon(con: *const libc::c_char) -> libc::c_int;
    fn setkeycreatecon(context: *const libc::c_char) -> libc::c_int;
    fn sudo_conf_sesh_path_v1() -> *const libc::c_char;
    fn reallocarray(__ptr: *mut libc::c_void, __nmemb: size_t, __size: size_t)
        -> *mut libc::c_void;
    fn sudo_execve(
        fd: libc::c_int,
        path: *const libc::c_char,
        argv: *const *mut libc::c_char,
        envp: *mut *mut libc::c_char,
        noexec: bool,
    ) -> libc::c_int;
    fn memcpy(
        __restrict__: *mut libc::c_void,
        __restrict__: *const libc::c_void,
        __n: size_t,
    ) -> *mut libc::c_void;
    fn sudo_fatal_nodebug_v1(fmt: *const libc::c_char, _: ...) -> !;
    fn sudo_fatalx_nodebug_v1(fmt: *const libc::c_char, _: ...) -> !;
    fn sudo_warn_gettext_v1(
        domainname: *const libc::c_char,
        msgid: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn sudo_warn_nodebug_v1(fmt: *const libc::c_char, _: ...);
}

//#define	EPROTONOSUPPORT	93	/* Protocol not supported */
//#define	EAFNOSUPPORT	97	/* Address family not supported by protocol */
pub const EPROTONOSUPPORT: libc::c_int = 93;
pub const EAFNOSUPPORT: libc::c_int = 97;

pub type security_class_t = libc::c_ushort;
pub type context_t = *mut context_s_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct context_s_t {
    pub ptr: *mut libc::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct selinux_state {
    pub old_context: security_context_t,
    pub new_context: security_context_t,
    pub tty_context: security_context_t,
    pub new_tty_context: security_context_t,
    pub ttyn: *const libc::c_char,
    pub ttyfd: libc::c_int,
    pub enforcing: libc::c_int,
}

pub type security_context_t = *mut libc::c_char;

static mut se_state: selinux_state = selinux_state {
    old_context: 0 as security_context_t,
    new_context: 0 as security_context_t,
    tty_context: 0 as security_context_t,
    new_tty_context: 0 as security_context_t,
    ttyn: 0 as *const libc::c_char,
    ttyfd: 0 as libc::c_int,
    enforcing: 0 as libc::c_int,
};

use crate::sudo_debug_printf2_v1;
use stdext::function_name;

// 因为原文件有static,所以不需要加#[no_mangle]
unsafe extern "C" fn audit_role_change(
    old_context: security_context_t,
    new_context: security_context_t,
    mut ttyn: *const libc::c_char,
    mut result: libc::c_int,
) -> libc::c_int {
    let mut au_fd: libc::c_int = 0;
    let mut rc: libc::c_int = -1;
    let mut message: *mut libc::c_char = 0 as *mut libc::c_char;
    //define debug_decl(audit_role_change,SUDO_DEBUG_SELINUX);
    debug_decl!(audit_role_change, SUDO_DEBUG_SELINUX);
    //end of define
    au_fd = audit_open();

    if au_fd == -1 {
        if *__errno_location() != EINVAL
            && *__errno_location() != EPROTONOSUPPORT
            && *__errno_location() != EAFNOSUPPORT
        {
            //define sudo_fatal(U_("unable to open audit system"));
            sudo_debug_printf!(
                SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"unable to open audit system\0" as *const u8 as *const libc::c_char
                )
            );
            sudo_fatal_nodebug_v1(sudo_warn_gettext_v1(
                0 as *const libc::c_char,
                b"unable to open audit system\0" as *const u8 as *const libc::c_char,
            ));
            //end of define
        }
    } else {
        rc = asprintf(
            &mut message as *mut *mut libc::c_char,
            b"newrole:old-context=%s new-context=%s\0" as *const u8 as *const libc::c_char,
            old_context,
            new_context,
        );
        if rc == -1 {
            //define sudo_fatalx(U_("%s:%s"),__func__,U_("unable to allocate memory"));
            sudo_debug_printf!(
                SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
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
        rc = audit_log_user_message(
            au_fd,
            2300,
            message,
            0 as *const libc::c_char,
            0 as *const libc::c_char,
            ttyn,
            result,
        );
        if rc <= 0 {
            //define sudo_warn(U_("unable to send audit message"));
            sudo_debug_printf!(
                SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"unable to send audit message\0" as *const u8 as *const libc::c_char
                )
            );
            sudo_warn_nodebug_v1(sudo_warn_gettext_v1(
                0 as *const libc::c_char,
                b"unable to send audit message\0" as *const u8 as *const libc::c_char,
            ));
            //end of define
        }
        free(message as *mut libc::c_void);
        close(au_fd);
    };
    //define debug_return_int(rc);
    debug_return_int!(rc);
    //end of define;
}

#[no_mangle]
unsafe extern "C" fn selinux_restore_tty() -> libc::c_int {
    let mut ret: libc::c_int = -(1 as libc::c_int);
    let mut chk_tty_context: security_context_t = 0 as security_context_t;
    //define debug_decl(selinux_restore_tty,SUDO_DEBUG_SELINUX)
    debug_decl!(selinux_restore_tty, SUDO_DEBUG_SELINUX);
    //end of define

    if se_state.ttyfd == -1 || (se_state.new_tty_context).is_null() {
        //define sudo_debug_printf(SUDO_DEBUG_INFO,"%s:no tty,skip relabel",__func__);
        sudo_debug_printf!(
            SUDO_DEBUG_INFO,
            b"%s: no tty,skip relabel\0" as *const u8 as *const libc::c_char,
            function_name!()
        );
        //end of define

        //define debug_return_int(0);
        debug_return_int!(0);
        //end of define
    }

    //define sudo_debug_printf(SUDO_DEBUG_INFO,"%s:%s->%s",__func__,se_state.new_tty_context,se_state.tty_context);
    sudo_debug_printf!(
        SUDO_DEBUG_INFO,
        b"%s:%s->%s\0" as *const u8 as *const libc::c_char,
        function_name!(),
        se_state.new_tty_context,
        se_state.tty_context
    );
    //end of define

    'skip_relabel: loop {
        //loop
        if fgetfilecon(se_state.ttyfd, &mut chk_tty_context) == -1 {
            //define sudo_warn(U_("unable to fgetfilecon %s"),se_state.ttyn);
            sudo_debug_printf!(
                SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"unable to fgetfilecon %s\0" as *const u8 as *const libc::c_char
                ),
                se_state.ttyn
            );
            sudo_warn_nodebug_v1(
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"unable to fgetfilecon\0" as *const u8 as *const libc::c_char,
                ),
                se_state.ttyn,
            );
            //end of define

            //跳转
            break 'skip_relabel;
        }

        if strcmp(
            chk_tty_context as *const libc::c_char,
            se_state.new_tty_context as *const libc::c_char,
        ) != 0
        {
            //define sudo_warnx(U_("%s changed labels"),se_state.ttyn);
            sudo_debug_printf!(
                SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"%s changed labels\0" as *const u8 as *const libc::c_char
                ),
                se_state.ttyn
            );
            sudo_warn_nodebug_v1(
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"%s changed labels\0" as *const u8 as *const libc::c_char,
                ),
                se_state.ttyn,
            );
            //end of define

            //define sudo_debug_printf(SUDO_DEBUG_INFO,"%s:not restoring tty label,expected %s,have %s",__func__,se_state.new_tty_context,chk_tty_context);
            sudo_debug_printf!(
                SUDO_DEBUG_INFO,
                b"%s:not restoring tty label,expected %s,have %s\0" as *const u8
                    as *const libc::c_char,
                function_name!(),
                se_state.new_tty_context,
                chk_tty_context
            );
            //end of define

            break 'skip_relabel;
        }

        if fsetfilecon(
            se_state.ttyfd as libc::c_int,
            se_state.tty_context as *const libc::c_char,
        ) == -1
        {
            //define sudo_warn(U_("unable to restore context for %s"),se_state.ttyn);
            sudo_debug_printf!(
                SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"unable to restore context for %s\0" as *const u8 as *const libc::c_char
                ),
                se_state.ttyn
            );
            sudo_warn_nodebug_v1(
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"unable to restore context for %s\0" as *const u8 as *const libc::c_char,
                ),
                se_state.ttyn,
            );
            //end of define
            break 'skip_relabel;
        }

        //define sudo_debug_printf(SUDO_DEBUG_INFO,"%s: successfully set tty label to
        //%s",__func__,se_state.tty_context);
        sudo_debug_printf!(
            SUDO_DEBUG_INFO,
            b"%s: successfully set tty label to %s\0" as *const u8 as *const libc::c_char,
            function_name!(),
            se_state.tty_context
        );
        //end of define
        ret = 0;

        break 'skip_relabel;
    } //loop skip_relabel

    if se_state.ttyfd != -1 {
        close(se_state.ttyfd);
        se_state.ttyfd = -1;
    }
    freecon(chk_tty_context);
    //define debug_return_int(ret);
    debug_return_int!(ret);
    //end of define
} //end of func

unsafe extern "C" fn relabel_tty(ttyn: *const libc::c_char, ptyfd: libc::c_int) -> libc::c_int {
    let mut tty_con: security_context_t = 0 as security_context_t;
    let mut new_tty_con: security_context_t = 0 as security_context_t;
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
    //'bad: loop{//goto bad;
    let mut fd: libc::c_int = 0;

    //define debug_decl(relabel_tty,SUDO_DEBUG_SELINUX)
    debug_decl!(relabel_tty, SUDO_DEBUG_SELINUX);
    //end of define

    se_state.ttyfd = ptyfd;

    if ptyfd == -1 && ttyn.is_null() {
        //define sudo_debug_printf(SUDO_DDEBUG_INFO,"%s: no tty,skip relabel",__func__);
        sudo_debug_printf!(
            SUDO_DEBUG_INFO,
            b"%s: no tty,skip relabel\0" as *const u8 as *const libc::c_char,
            function_name!()
        );
        //end of define

        //define debug_return_int(0);
        debug_return_int!(0);
        //end of define
    }

    //define sudo_debug_printf(SUDO_DEBUG_INFO,"%s:relabeling tty %s",__func__,ttyn);
    sudo_debug_printf!(
        SUDO_DEBUG_INFO,
        b"%s:relabeling tty %s\0" as *const u8 as *const libc::c_char,
        function_name!(),
        ttyn
    );
    //end of define

    'bad: loop {
        //goto bad;

        if ptyfd == -1 {
            se_state.ttyfd = open(ttyn, 2 | 256 | 2048);

            if se_state.ttyfd == -1 || fstat(se_state.ttyfd, &mut sb) == -1 {
                //define sudo_warn(U_("unable to open %s,not relabeling tty"),ttyn);
                sudo_debug_printf!(
                    SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"unable to open %s,not relabeling tty\0" as *const u8
                            as *const libc::c_char
                    ),
                    ttyn
                );
                sudo_warn_nodebug_v1(
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"unable to open %s,not relabeling tty\0" as *const u8
                            as *const libc::c_char,
                    ),
                    ttyn,
                );
                //end of define
                break 'bad;
            }

            //            if !(sb.st_mode & 61440 == 16384) {
            if !(sb.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o20000 as libc::c_int as libc::c_uint)
            {
                //define sudo_warn(U_("%s is not a character device,not relabeling tty"),ttyn);
                sudo_debug_printf!(
                    SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"%s is not a character device,not relabeling tty\0" as *const u8
                            as *const libc::c_char
                    ),
                    ttyn
                );
                sudo_warn_nodebug_v1(
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"%s is not a character device,not relabeling tty\0" as *const u8
                            as *const libc::c_char,
                    ),
                    ttyn,
                );
                //end of define
                break 'bad;
            }

            fcntl(
                se_state.ttyfd,
                F_SETFL,
                fcntl(se_state.ttyfd, F_GETFL, 0) & !(2048),
            );
        }

        if fgetfilecon(se_state.ttyfd, &mut tty_con) == -1 {
            //define sudo_warn(U_("unable to get current ttycontext,not relabeling tty"));
            sudo_debug_printf!(
                SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"unable to get current ttycontext,not relabeling tty\0" as *const u8
                        as *const libc::c_char
                )
            );
            sudo_warn_nodebug_v1(sudo_warn_gettext_v1(
                0 as *const libc::c_char,
                b"unable to get current ttycontext,not relabeling tty\0" as *const u8
                    as *const libc::c_char,
            ));
            //end of define
            break 'bad;
        }

        if !tty_con.is_null() {
            let mut tclass: security_class_t =
                string_to_security_class(b"chr_file\0" as *const u8 as *const libc::c_char);

            if tclass == 0 {
                //define sudo_warn(U_(""));
                sudo_debug_printf!(
                    SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"unknown security class \"char_file\",not relabeling tty\0" as *const u8
                            as *const libc::c_char
                    )
                );
                sudo_warn_nodebug_v1(sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"unknown security class \"char_file\",not relabeling tty\0" as *const u8
                        as *const libc::c_char,
                ));
                //end of define
                break 'bad;
            }

            if security_compute_relabel(se_state.new_context, tty_con, tclass, &mut new_tty_con)
                == -1
            {
                //define sudo_warn(U_(""));
                sudo_debug_printf!(
                    SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"unable to get new tty context,not relabeling tty\0" as *const u8
                            as *const libc::c_char
                    )
                );
                sudo_warn_nodebug_v1(sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"unable to get new tty context,not relabeling tty\0" as *const u8
                        as *const libc::c_char,
                ));
                //end of define
                break 'bad;
            }
        }

        if !new_tty_con.is_null() {
            //define sudo_debug_printf(SUDO_DEBUG_INFO,"&s:tty context %s -> %s",__func__,tty_con,new_tty_con);
            sudo_debug_printf!(
                SUDO_DEBUG_INFO,
                b"%s:ttycontext %s -> %s\0" as *const u8 as *const libc::c_char,
                function_name!(),
                tty_con,
                new_tty_con
            );
            //end of define
            if fsetfilecon(se_state.ttyfd, new_tty_con) == -1 {
                //define sudo_warn(U_(""));
                sudo_debug_printf!(
                    SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"unable to set new tty context\0" as *const u8 as *const libc::c_char
                    )
                );
                sudo_warn_nodebug_v1(sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"unable to set new tty context\0" as *const u8 as *const libc::c_char,
                ));
                //end of define
                break 'bad;
            }
        }

        if ptyfd != -1 {
            se_state.ttyfd = open(ttyn, 2 | 256, 0);
            if se_state.ttyfd == -1 || fstat(se_state.ttyfd, &mut sb) == -1 {
                //sudo_warn(u_("unable to open %s",ttyn));
                sudo_debug_printf!(
                    SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"unable to open %s\0" as *const u8 as *const libc::c_char
                    ),
                    ttyn
                );
                sudo_warn_nodebug_v1(
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"unable to open %s\0" as *const u8 as *const libc::c_char,
                    ),
                    ttyn,
                );
                //end of define;
                break 'bad;
            }
            //            if !(sb.st_mode & 61440 == 8192) {
            if !(sb.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o20000 as libc::c_int as libc::c_uint)
            {
                //sudo_warn(u_("",ttyn));
                sudo_debug_printf!(
                    SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"%s is not a character device,not relabeling tty\0" as *const u8
                            as *const libc::c_char
                    ),
                    ttyn
                );
                sudo_warn_nodebug_v1(
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"%s is not a character device,not relabeling tty\0" as *const u8
                            as *const libc::c_char,
                    ),
                    ttyn,
                );
                //end of define;
                break 'bad;
            }
            if dup2(se_state.ttyfd, ptyfd) == -1 {
                //define sudo_warn("dup2");
                sudo_debug_printf!(
                    SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                    b"dup2\0" as *const u8 as *const libc::c_char
                );
                sudo_warn_nodebug_v1(b"dup2\0" as *const u8 as *const libc::c_char);
                //end of define;
                break 'bad;
            }
        } else {
            close(se_state.ttyfd);
            se_state.ttyfd = open(ttyn, 2 | 256 | 2048);
            if se_state.ttyfd == -1 || fstat(se_state.ttyfd, &mut sb) == -1 {
                //sudo_warn(u_("unable to open %s",ttyn));
                sudo_debug_printf!(
                    SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"unable to open %s\0" as *const u8 as *const libc::c_char
                    ),
                    ttyn
                );
                sudo_warn_nodebug_v1(
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"unable to open %s\0" as *const u8 as *const libc::c_char,
                    ),
                    ttyn,
                );
                //end of define
                break 'bad;
            }
            if !(sb.st_mode & 61440 == 8192) {
                //sudo_warn(u_("%s is not a character device,not relabeling tty",ttyn));
                sudo_debug_printf!(
                    SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"%s is not a character device,not relabeling tty\0" as *const u8
                            as *const libc::c_char
                    ),
                    ttyn
                );
                sudo_warn_nodebug_v1(
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"%s is not a character device,not relabeling tty\0" as *const u8
                            as *const libc::c_char,
                    ),
                    ttyn,
                );
                //end of define
                break 'bad;
            }

            fcntl(se_state.ttyfd, 4, fcntl(se_state.ttyfd, 3, 0) & !2048);
            loop {
                if fd <= 2 {
                    break;
                }
                if isatty(fd) != 0 && dup2(se_state.ttyfd, fd) == -1 {
                    //define sudo_warn("dup2");
                    sudo_debug_printf!(
                        SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                        b"dup2\0" as *const u8 as *const libc::c_char
                    );
                    sudo_warn_nodebug_v1(b"dup2\0" as *const u8 as *const libc::c_char);
                    //end of define

                    break 'bad;
                }
            }
        } //else

        fcntl(se_state.ttyfd, 2, 1);

        se_state.ttyn = ttyn;
        se_state.tty_context = tty_con;
        se_state.new_tty_context = new_tty_con;

        //define debug_return_int(0);
        debug_return_int!(0);
        //end of define
        break 'bad;
    } //goto bad;

    //bad;
    if se_state.ttyfd != -1 && se_state.ttyfd != ptyfd {
        close(se_state.ttyfd);
        se_state.ttyfd = -1;
    }
    freecon(tty_con);
    //define debug_return_int(se_state.enforcing ? -1 : 0); 替换为以下ｉｆ函数
    if se_state.enforcing != 0 {
        //define debug_return_int(-1);
        debug_return_int!(-1);
        //end of define
    } else {
        //define debug_return_int(0);
        debug_return_int!(0);
        //end of define
    }
} //fn relabel_tty

#[no_mangle]
pub unsafe extern "C" fn get_exec_context(
    mut old_context: security_context_t,
    mut role: *const libc::c_char,
    mut types: *const libc::c_char,
) -> security_context_t {
    let mut new_context: security_context_t = 0 as security_context_t;
    let mut context: context_t = 0 as context_t;
    let mut typebuf: *mut libc::c_char = 0 as *mut libc::c_char;
    //define debug_decl(get_exec_context,SUDO_DEBUG_SELINUX)
    debug_decl!(get_exec_context, SUDO_DEBUG_SELINUX);
    //end of define
    'bad: loop {
        if role.is_null() {
            //define sudo_warnx(U_("you mast specify a role for type %s"),type);
            sudo_debug_printf!(
                SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"you mast specity a role for type %s\0" as *const u8 as *const libc::c_char
                ),
                types
            );
            sudo_warn_nodebug_v1(
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"you mast specity a role for type %s\0" as *const u8 as *const libc::c_char,
                ),
                types,
            );
            //end of define
            *__errno_location() = EINVAL;
            break 'bad;
        }

        if types.is_null() {
            if get_default_type(role, &mut typebuf) != 0 {
                //define sudo_warnx(U_("unable to get default type for role %s"),role);
                sudo_debug_printf!(
                    SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"unable to get default type for role %s\0" as *const u8
                            as *const libc::c_char
                    ),
                    role
                );
                sudo_warn_nodebug_v1(
                    sudo_warn_gettext_v1(
                        0 as *const libc::c_char,
                        b"unable to get default type for role %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    role,
                );
                //end of define
                *__errno_location() = EINVAL;
                break 'bad;
            }
            types = typebuf;
        }

        context = context_new(old_context);
        if context.is_null() {
            //define sudo_warn(U_("failed to get new context"));
            sudo_debug_printf!(
                SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"failed to get new context\0" as *const u8 as *const libc::c_char
                )
            );
            sudo_warn_nodebug_v1(sudo_warn_gettext_v1(
                0 as *const libc::c_char,
                b"failed to get new context\0" as *const u8 as *const libc::c_char,
            ));
            //end of define
            break 'bad;
        }

        if context_role_set(context, role) != 0 {
            //define sudo_warn(U_("fail to set new role %s"),role);
            sudo_debug_printf!(
                SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"failed to set new role %s\0" as *const u8 as *const libc::c_char
                ),
                role
            );
            sudo_warn_nodebug_v1(
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"failed to set new role %s\0" as *const u8 as *const libc::c_char,
                ),
                role,
            );
            //end of define
            break 'bad;
        }

        if context_type_set(context, types) != 0 {
            //define sudo_warn(U_("fail to set new type %s"),types);
            sudo_debug_printf!(
                SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"fail to set new type %s\0" as *const u8 as *const libc::c_char
                ),
                types
            );
            sudo_warn_nodebug_v1(
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"failed to set new type %s\0" as *const u8 as *const libc::c_char,
                ),
                types,
            );
            //end of define
            break 'bad;
        }

        new_context = strdup(context_str(context));
        if new_context.is_null() {
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
            sudo_warn_nodebug_v1(
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

        if security_check_context(new_context) == -1 {
            //define sudo_warnx(U_("%s is not a valid context"),new_context);
            sudo_debug_printf!(
                SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"%s is not a valid context\0" as *const u8 as *const libc::c_char
                ),
                new_context
            );
            sudo_warn_nodebug_v1(
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"%s is not a valid context\0" as *const u8 as *const libc::c_char,
                ),
                new_context,
            );
            //end of define
            *__errno_location() = EINVAL;
            break 'bad;
        }

        context_free(context);
        //define debug_return_str(new_context);
        debug_return_str!(new_context);
        //end of define
        break 'bad;
    } //loop bad

    free(typebuf as *mut libc::c_void);
    context_free(context);
    freecon(new_context);

    //define  debug_return_str(NULL);
    debug_return_str!(0 as *mut libc::c_char);
    //end of define
}

#[no_mangle]
pub unsafe extern "C" fn selinux_setup(
    mut role: *const libc::c_char,
    mut types: *const libc::c_char,
    mut ttyn: *const libc::c_char,
    mut ptyfd: libc::c_int,
    mut label_tty: bool,
) -> libc::c_int {
    let mut ret: libc::c_int = -1 as libc::c_int;
    //define debug_decl(selinux_setup,SUDO_DEBUG_SELINUX)
    debug_decl!(selinux_setup, SUDO_DEBUG_SELINUX);
    //end of define

    'done: loop {
        if getprevcon(&mut se_state.old_context) != 0 {
            //define sudo_warn(U_("failed to get old context"));
            sudo_debug_printf!(
                SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"failed to get old context\0" as *const u8 as *const libc::c_char
                )
            );
            sudo_warn_nodebug_v1(sudo_warn_gettext_v1(
                0 as *const libc::c_char,
                b"failed to get old context\0" as *const u8 as *const libc::c_char,
            ));
            //end of define
            break 'done;
        }

        se_state.enforcing = security_getenforce();
        if se_state.enforcing == -1 {
            //define  sudo_warn(U_("unable to determine enforcing mode."));
            sudo_debug_printf!(
                SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"unable to determine enforcing mode.\0" as *const u8 as *const libc::c_char
                )
            );
            sudo_warn_nodebug_v1(sudo_warn_gettext_v1(
                0 as *const libc::c_char,
                b"unable to determine enforcing mode.\0" as *const u8 as *const libc::c_char,
            ));
            //end of define;
            break 'done;
        }

        //define sudo_debug_printf(SUDO_DEBUG_INFO,%s: old context %s,__func__,se_state.old_context);
        sudo_debug_printf!(
            SUDO_DEBUG_INFO,
            b"%s: old context %s\0" as *const u8 as *const libc::c_char,
            function_name!(),
            se_state.old_context
        );
        //end of define
        se_state.new_context = get_exec_context(se_state.old_context, role, types);
        if se_state.new_context.is_null() {
            //why translate 3 times
            audit_role_change(
                se_state.old_context,
                b"?\0" as *const u8 as *const libc::c_char as security_context_t,
                se_state.ttyn,
                0,
            );
            break 'done;
        }

        //define sudo_debug_printf(SUDO_DEBUG_INFO,%s: new context %s,__func__,se_state.new_context);
        sudo_debug_printf!(
            SUDO_DEBUG_INFO,
            b"%s: new context %s\0" as *const u8 as *const libc::c_char,
            function_name!(),
            se_state.new_context
        );
        //end of define

        if label_tty == true && relabel_tty(ttyn, ptyfd) == -1 {
            //dedine sudo_warn(U_("unable to set tty context to %s"),se_state.new_context);
            sudo_debug_printf!(
                SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"unable to set tty context to %s\0" as *const u8 as *const libc::c_char
                ),
                se_state.new_context
            );
            sudo_warn_nodebug_v1(
                sudo_warn_gettext_v1(
                    0 as *const libc::c_char,
                    b"unable to set tty context to %s\0" as *const u8 as *const libc::c_char,
                ),
                se_state.new_context,
            );
            //end of define

            break 'done;
        }

        audit_role_change(se_state.old_context, se_state.new_context, se_state.ttyn, 1);

        ret = 0;

        break 'done;
    } //loop done

    //define: debug_return_int(ret);
    debug_return_int!(ret);
    //end of define
}

#[no_mangle]
pub unsafe extern "C" fn selinux_setcon() -> libc::c_int {
    //define debug_decl(selinux_setcon,SUDO_DEBUG_SELINUX);
    debug_decl!(selinux_setcon, SUDO_DEBUG_SELINUX);
    //end of define

    if setexeccon(se_state.new_context) != 0 {
        //define sudo_warn(U_("unable to set exec context to %s"),se_state.new_context,);
        sudo_debug_printf!(
            SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
            sudo_warn_gettext_v1(
                0 as *const libc::c_char,
                b"unable to set exec context to %s\0" as *const u8 as *const libc::c_char
            ),
            se_state.new_context
        );
        sudo_warn_nodebug_v1(
            sudo_warn_gettext_v1(
                0 as *const libc::c_char,
                b"unable to set exec context to %s\0" as *const u8 as *const libc::c_char,
            ),
            se_state.new_context,
        );
        //end of define
        if se_state.enforcing != 0 {
            //define  debug_return_int(-1);
            debug_return_int!(-1);
            //end of define;
        }
    }

    if setkeycreatecon(se_state.new_context) != 0 {
        //sudo_warn(U_("unable to set key creation context to %s"),se_state.new_context);
        sudo_debug_printf!(
            SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
            sudo_warn_gettext_v1(
                0 as *const libc::c_char,
                b"unable to set key creation context to %s\0" as *const u8 as *const libc::c_char
            ),
            se_state.new_context
        );
        sudo_warn_nodebug_v1(
            sudo_warn_gettext_v1(
                0 as *const libc::c_char,
                b"unable to set key creation context to %s\0" as *const u8 as *const libc::c_char,
            ),
            se_state.new_context,
        );
        //end of define
        if se_state.enforcing != 0 {
            //define debug_return_int(-1);
            debug_return_int!(-1);
            //end of define
        }
    }

    //define debug_return_int(0);
    debug_return_int!(0);
    //end of define
}

#[no_mangle]
pub unsafe extern "C" fn selinux_execve(
    mut fd: libc::c_int,
    mut path: *const libc::c_char,
    mut argv: *const *mut libc::c_char,
    mut envp: *mut *mut libc::c_char,
    mut noexec: bool,
) {
    let mut nargv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut sesh: *const libc::c_char = 0 as *const libc::c_char;
    let mut argc: libc::c_int = 0 as libc::c_int;
    let mut nargc: libc::c_int = 0 as libc::c_int;
    let mut serrno: libc::c_int = 0 as libc::c_int;

    //define debug_decl(selinux_execve,SUDO_DEBUG_SELINUX)
    debug_decl!(selinux_execve, SUDO_DEBUG_SELINUX);
    //end of define

    sesh = sudo_conf_sesh_path_v1();
    if sesh.is_null() {
        //define sudo_warnx("internal error:sesh path not set");
        sudo_debug_printf!(
            SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
            b"internal error:sesh path not set\0" as *const u8 as *const libc::c_char
        );
        sudo_warn_nodebug_v1(
            b"internal error:sesh path not set\0" as *const u8 as *const libc::c_char,
        );
        //end of define
        *__errno_location() = EINVAL;
        //define debug_return;
        debug_return!();
        //end of define;
    }

    if selinux_setcon() == -1 {
        //define debug_return;
        debug_return!();
        //end of define;
    }

    argc = 0 as libc::c_int;
    while !(*argv.offset(argc as isize)).is_null() {
        argc += 1;
    }

    nargv = reallocarray(
        0 as *mut libc::c_void,
        (argc + 3 as libc::c_int) as size_t,
        ::std::mem::size_of::<*mut libc::c_char>() as size_t,
    ) as *mut *mut libc::c_char;

    if nargv.is_null() {
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
        sudo_warn_nodebug_v1(
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

        //define debug_return;
        debug_return!();
        //end of define
    }

    if noexec {
        *nargv.offset(0 as isize) = (if **argv.offset(0 as isize) as libc::c_int == '-' as i32 {
            b"-sesh-noexec\0" as *const u8 as *const libc::c_char
        } else {
            b"sesh-noexec\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char
    } else {
        *nargv.offset(0 as isize) = (if **argv.offset(0 as isize) as libc::c_int == '-' as i32 {
            b"-sesh\0" as *const u8 as *const libc::c_char
        } else {
            b"sesh\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char
    }

    nargc = 1;

    if fd != -1 && {
        let tmp = nargc;
        nargc += 1;
        asprintf(
            &mut *nargv.offset(tmp as isize) as *mut *mut libc::c_char,
            b"--execfd=%d\0" as *const u8 as *const libc::c_char,
            fd,
        ) == -1
    } {
        //sudo_warnx(U_("%s:%s"),__func__,U_("unable to allocate memory"));
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
        sudo_warn_nodebug_v1(
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

        //debug_return;
        debug_return!();
        //end of define
    }

    *nargv.offset(nargc as isize) = path as *mut libc::c_char;
    nargc += 1;

    memcpy(
        &mut *nargv.offset(nargc as isize) as *mut *mut libc::c_char as *mut libc::c_void,
        &*argv.offset(1 as isize) as *const *mut libc::c_char as *const libc::c_void,
        argc.wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as i32) as size_t,
    );

    sudo_execve(-1, sesh, nargv as *const *mut libc::c_char, envp, false);
    serrno = *__errno_location();
    free(nargv as *mut libc::c_void);
    *__errno_location() = serrno;
    //define  debug_return;
    debug_return!();
    //end of define
} //end file
