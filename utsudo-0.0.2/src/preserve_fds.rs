/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */
use crate::struct_macro::*;
use utsudo_util::sudo_debug::*;
use utsudo_util::sudo_debug_macro::*;
use utsudo_util::*;
use libc::__errno_location;
use libc::strerror;
extern "C" {
    fn malloc(__size: size_t) -> *mut libc::c_void;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn strtol(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> libc::c_long;
    fn sudo_closefrom(_: libc::c_int);
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut libc::c_void;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn dup(__fd: libc::c_int) -> libc::c_int;
    fn sudo_debug_update_fd_v1(ofd: libc::c_int, nfd: libc::c_int);
    fn sudo_warn_gettext_v1(
        domainname: *const libc::c_char,
        msgid: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn sudo_warn_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_fatalx_nodebug_v1(fmt: *const libc::c_char, _: ...) -> !;
}
use crate::sudo_debug_printf2_v1;
use stdext::function_name;

#[no_mangle]
pub unsafe extern "C" fn add_preserved_fd(
    mut pfds: *mut preserved_fd_list,
    mut fd: libc::c_int,
) -> libc::c_int {
    let mut pfd: *mut preserved_fd = 0 as *mut preserved_fd;
    let mut pfd_new: *mut preserved_fd = 0 as *mut preserved_fd;
    //define  debug_decl(add_preserved_fd,SUDO_DEBUG_UTIL)
    debug_decl!(add_preserved_fd, SUDO_DEBUG_UTIL);
    //end of define
    pfd_new = malloc(::std::mem::size_of::<preserved_fd>() as size_t) as *mut preserved_fd;
    if pfd_new.is_null() {
        //define sudo_fatalx(U_("%s:%s"),__func__,U_("unable to allocate memory"));
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
    (*pfd_new).lowfd = fd;
    (*pfd_new).highfd = fd;
    (*pfd_new).flags = fcntl(fd, 1);
    if (*pfd_new).flags == -1 {
        free(pfd_new as *mut libc::c_void);
        //define debug_return_int(-1);
        debug_return_int!(-1);
        //end of define
    }
    pfd = (*pfds).tqh_first;
    while !pfd.is_null() {
        if fd == (*pfd).highfd {
            //define sudo_debug_printf(SUDO_DEBUG_DEBUG|SUDO_DEBUG_LINENO,"fd %d already preserved",fd);
            sudo_debug_printf!(
                SUDO_DEBUG_DEBUG | SUDO_DEBUG_LINENO,
                b"fd %d already preserved\0" as *const u8 as *const libc::c_char,
                fd
            );
            //end of define
            free(pfd_new as *mut libc::c_void);
            pfd_new = 0 as *mut preserved_fd;
            break;
        }
        if fd < (*pfd).highfd {
            (*pfd_new).entries.tqe_prev = (*pfd).entries.tqe_prev;
            (*pfd_new).entries.tqe_next = pfd;
            *(*pfd).entries.tqe_prev = pfd_new;
            (*pfd).entries.tqe_prev = &mut (*pfd_new).entries.tqe_next;
            //define sudo_debug_printf(SUDO_DEBUG_DEBUG|SUDOD_DEBUG_LINENO,"preserving fd %d",fd);
            sudo_debug_printf!(
                SUDO_DEBUG_DEBUG | SUDO_DEBUG_LINENO,
                b"preserving fd %d\0" as *const u8 as *const libc::c_char,
                fd
            );
            //end of define
            pfd_new = 0 as *mut preserved_fd;
            break;
        }
        pfd = (*pfd).entries.tqe_next;
    } //while
    if !pfd_new.is_null() {
        (*pfd_new).entries.tqe_next = 0 as *mut preserved_fd;
        (*pfd_new).entries.tqe_prev = (*pfds).tqh_last;
        *(*pfds).tqh_last = pfd_new;
        (*pfds).tqh_last = &mut (*pfd_new).entries.tqe_next;
        //define sudo_debug_printf(SUDO_DEBUG_DEBUG|SUDO_DEBUG_LINENO,"preserving fd %d",fd);
        sudo_debug_printf!(
            SUDO_DEBUG_DEBUG | SUDO_DEBUG_LINENO,
            b"preserving fd %d\0" as *const u8 as *const libc::c_char,
            fd
        );
        //end of dedine
    }
    //debug_return_int(0);
    debug_return_int!(0);
    //end of define
}
