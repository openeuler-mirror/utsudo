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

#[no_mangle]
pub unsafe extern "C" fn closefrom_except(
    mut startfd: libc::c_int,
    mut pfds: *mut preserved_fd_list,
) {
    let mut fd: libc::c_int = 0;
    let mut lastfd: libc::c_int = -1;
    let mut pfd: *mut preserved_fd = 0 as *mut preserved_fd;
    let mut pfd_next: *mut preserved_fd = 0 as *mut preserved_fd;
    let mut fdbits: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    //define debug_decl(closefrom_except,SUDO_DEBUG_UTIL)
    debug_decl!(closefrom_except, SUDO_DEBUG_UTIL);
    //end of define
    pfd = *(*((*pfds).tqh_last as *mut preserved_fd_list)).tqh_last;
    while !pfd.is_null() && {
        pfd_next = *(*((*pfd).entries.tqe_prev as *mut preserved_fd_list)).tqh_last;
        1 != 0
    } {
        //line 102-126
        if (*pfd).highfd < startfd {
            continue;
        }
        fd = dup((*pfd).highfd);
        if fd == -1 {
            //define sudo_debug_printf(SUDO_DEBUG_ERROR|SUDO_DEBUG_LINENO|SUDO_DEBUG_ERRNO,
            //"dup %d",pfd->highfd);
            sudo_debug_printf!(
                SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                b"dup %d\0" as *const u8 as *const libc::c_char,
                (*pfd).highfd
            );
            //end of define
            if *__errno_location() == 9 {
                //wait line 110
                if !((*pfd).entries.tqe_next).is_null() {
                    (*(*pfd).entries.tqe_next).entries.tqe_prev = (*pfd).entries.tqe_prev;
                } else {
                    (*pfds).tqh_last = (*pfd).entries.tqe_prev;
                    *(*pfd).entries.tqe_prev = (*pfd).entries.tqe_next;
                }
                continue;
            }
        } else if fd < (*pfd).highfd {
            //define sudo_debug_printf(SUDO_DEBUG_DEBUG|SUDO_DEBUG_LINENO,
            //"dup %d -> %d",pfd->highfd,pfd->lowfd);
            sudo_debug_printf!(
                SUDO_DEBUG_DEBUG | SUDO_DEBUG_LINENO,
                b"dup %d -> %d\0" as *const u8 as *const libc::c_char,
                (*pfd).highfd,
                (*pfd).lowfd
            );
            //end of define;
            sudo_debug_update_fd_v1((*pfd).highfd, (*pfd).lowfd);
            (*pfd).lowfd = fd;
            fd = (*pfd).highfd;
        }
        if fd != -1 {
            close(fd);
        }
        if (*pfd).lowfd > lastfd {
            lastfd = (*pfd).lowfd;
        }
        pfd = pfd_next;
    } //end while
    if lastfd == -1 {
        //define sudo_debug_printf(SUDO_DEBUG_DEBUG|SUDO_DEBUG_LINENO,"closefrom(%d)",startfd);
        sudo_debug_printf!(
            SUDO_DEBUG_DEBUG | SUDO_DEBUG_LINENO,
            b"closefrom(%d)\0" as *const u8 as *const libc::c_char,
            startfd
        );
        //end of define
        sudo_closefrom(startfd);
        //define debug_return;
        debug_return!();
        //end of define;
    }
    fdbits = calloc(
        ((lastfd + 8 as libc::c_int) / 8 as libc::c_int) as size_t,
        1 as size_t,
    ) as *mut libc::c_uchar;
    if fdbits.is_null() {
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
    pfd = (*pfds).tqh_first;
    while !pfd.is_null() {
        *fdbits.offset(((*pfd).lowfd / 8 as libc::c_int) as isize) |= 1 << (*pfd).lowfd % 8;
        pfd = (*pfd).entries.tqe_next;
    }
    fd = startfd;
    while fd <= lastfd {
        if *fdbits.offset((fd / 8 as libc::c_int) as isize) & 1 << fd % 8 == 0 {
            //define sudo_debug_printf(SUDO_DEBUG_DEBUG|SUDO_DEBUG_LINENO,"closeing fd %d",fd);
            sudo_debug_printf!(
                SUDO_DEBUG_DEBUG | SUDO_DEBUG_LINENO,
                b"closing fd %d\0" as *const u8 as *const libc::c_char,
                fd
            );
            //end of define;
            close(fd);
        }
        fd += 1;
    }
    free(fdbits as *mut libc::c_void);
    if lastfd + 1 > startfd {
        startfd = lastfd + 1;
    }
    //define sudo_debug_printf(SUDO_DEBUG_DEBUG|SUDO_DEBUG_LINENO,"closefrom(%d)",startfd);
    sudo_debug_printf!(
        SUDO_DEBUG_DEBUG | SUDO_DEBUG_LINENO,
        b"closefrom(%d)\0" as *const u8 as *const libc::c_char,
        startfd
    );
    //end of define;
    sudo_closefrom(startfd);
    //170-192
    pfd = *(*((*pfds).tqh_last as *mut preserved_fd_list)).tqh_last;
    while !pfd.is_null() {
        if (*pfd).lowfd != (*pfd).highfd {
            if dup2((*pfd).lowfd, (*pfd).highfd) == -1 {
                //sudo_debug_printf(SUDO_DEBUG_ERROR|SUDO_DEBUG_LINENO,"dup2(%d,%d):%s",pfd->lowfd,pfd->lighfd,strerror(errno));
                sudo_debug_printf!(
                    SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
                    b"dup2(%d, %d)\0" as *const u8 as *const libc::c_char,
                    (*pfd).lowfd,
                    (*pfd).highfd,
                    strerror(*__errno_location())
                );
                //end of define
            } else {
                //sudo_debug_printf(SUDO_DEBUG_ERROR|SUDO_DEBUG_LINENO,"dup2(%d,%d):%s",pfd->lowfd,pfd->lighfd);
                sudo_debug_printf!(
                    SUDO_DEBUG_DEBUG | SUDO_DEBUG_LINENO,
                    b"dup2(%d, %d)\0" as *const u8 as *const libc::c_char,
                    (*pfd).lowfd,
                    (*pfd).highfd
                );
                //end of define
            }
            if fcntl((*pfd).highfd, 2, (*pfd).flags) == -1 {
                //sudo_debug_printf(SUDO_DEBUG_ERROR|SUDO_DEBUG_LINENO,"fcntl(%d,F_SETFD,%d):%s",pfd->highfd,pfd->flags,strerror(errno));
                sudo_debug_printf!(
                    SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
                    b"fcntl(%d,F_SETFD,%d)\0" as *const u8 as *const libc::c_char,
                    (*pfd).lowfd,
                    (*pfd).highfd,
                    strerror(*__errno_location())
                );
                //end of define
            } else {
                //sudo_debug_printf(SUDO_DEBUG_ERROR|SUDO_DEBUG_LINENO,"fcntl(%d,F_SETFD,%d):%s",pfd->highfd,pfd->flags);
                sudo_debug_printf!(
                    SUDO_DEBUG_DEBUG | SUDO_DEBUG_LINENO,
                    b"fcntl(%d,F_SETFD,%d)\0" as *const u8 as *const libc::c_char,
                    (*pfd).lowfd,
                    (*pfd).highfd
                );
                //end of define
            }
            sudo_debug_update_fd_v1((*pfd).lowfd, (*pfd).highfd);
            close((*pfd).lowfd);
            (*pfd).lowfd = (*pfd).highfd;
        }
        pfd = *(*((*pfd).entries.tqe_prev as *mut preserved_fd_list)).tqh_last;
    }
    //define debug_return
    debug_return!();
    //end of define;
}






