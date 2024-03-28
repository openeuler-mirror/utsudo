/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

type size_t = libc::c_ulong;
type sa_family_t = libc::c_ushort;
type in_port_t = libc::c_ushort;
type in_addr_t = libc::c_uint;
type socklen_t = libc::c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifaddrs {
    pub ifa_next: *mut ifaddrs,
    pub ifa_name: *mut libc::c_char,
    pub ifa_flags: libc::c_int,
    pub ifa_addr: *mut sockaddr,
    pub ifa_netmask: *mut sockaddr,
    pub ifa_ifu: TMP_1,
    pub ifa_data: *mut libc::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union TMP_1 {
    pub ifu_broadaddr: *mut sockaddr,
    pub ifu_dstaddr: *mut sockaddr,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sa_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: libc::c_uint,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: libc::c_uint,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union TMP_T {
    pub __u6_addr8: [libc::c_uchar; 16],
    pub __u6_addr16: [libc::c_ushort; 8],
    pub __u6_addr32: [libc::c_uint; 4],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: TMP_T,
}

extern "C" {
    fn sudo_conf_probe_interfaces_v1() -> bool;
    fn getifaddrs(__ifap: *mut *mut ifaddrs) -> libc::c_int;
    fn freeifaddrs(__ifa: *mut ifaddrs);
    fn malloc(__size: size_t) -> *mut libc::c_void;
    fn inet_ntop(
        __af: libc::c_int,
        __cp: *const libc::c_void,
        __buf: *mut libc::c_char,
        __len: socklen_t,
    ) -> *const libc::c_char;
    fn snprintf(
        __s: *mut libc::c_char,
        __maxlen: size_t,
        __format: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sudo_warn_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_warn_gettext_v1(
        domainname: *const libc::c_char,
        msgid: *const libc::c_char,
    ) -> *mut libc::c_char;
}

pub const SUDO_DEBUG_NETIF: libc::c_int = 8 << 6;
use crate::sudo_debug_printf2_v1;
use stdext::function_name;
use utsudo_util::debug_decl;
use utsudo_util::debug_decl_vars;
use utsudo_util::debug_return_int;
use utsudo_util::sudo_debug::sudo_debug_enter_v1;
use utsudo_util::sudo_debug::sudo_debug_exit_int_v1;
use utsudo_util::sudo_debug_macro::sudo_debug_subsys;
use utsudo_util::sudo_debug_macro::SUDO_DEBUG_LINENO;
use utsudo_util::sudo_debug_printf;
//use utsudo_util::sudo_debug_macro::SUDO_DEBUG_ERRNO;
use utsudo_util::sudo_debug_macro::SUDO_DEBUG_ERROR;
use utsudo_util::sudo_debug_macro::SUDO_DEBUG_WARN;

#[no_mangle]
pub unsafe extern "C" fn get_net_ifs(mut addrinfo: *mut *mut libc::c_char) -> libc::c_int {
    let mut ifa: *mut ifaddrs = 0 as *mut ifaddrs;
    let mut ifaddrs: *mut ifaddrs = 0 as *mut ifaddrs;
    let mut sin: *mut sockaddr_in = 0 as *mut sockaddr_in;
    let mut sin6: *mut sockaddr_in6 = 0 as *mut sockaddr_in6;
    let mut addrstr: [libc::c_char; 46] = [0; 46];
    let mut maskstr: [libc::c_char; 46] = [0; 46];
    let mut ailen: libc::c_int = 0 as libc::c_int;
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut num_interfaces: libc::c_int = 0 as libc::c_int;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    //define debug_decl(get_net_ifs,SUDO_DEBUG_NETIF);
    debug_decl!(get_net_ifs, SUDO_DEBUG_NETIF);
    //end of define

    if !sudo_conf_probe_interfaces_v1() {
        //define debug_return_int(0);
        debug_return_int!(0);
        //end of define
    }

    if getifaddrs(&mut ifaddrs) == -1 {
        //define debug_return_int(-1);
        debug_return_int!(-1);
        //end of define
    }

    ifa = ifaddrs;
    while !ifa.is_null() {
        if !(((*ifa).ifa_addr).is_null()
            || ((*ifa).ifa_netmask).is_null()
            || (*ifa).ifa_flags & 1 == 0
            || (*ifa).ifa_flags & 8 != 0)
        {
            match (*(*ifa).ifa_addr).sa_family {
                2 | 10 => {
                    num_interfaces += 1;
                }
                _ => {}
            }
        }
        ifa = (*ifa).ifa_next;
    } //end of while

    //wait write line 153
    'done: loop {
        if num_interfaces == 0 {
            break 'done;
        }
        ailen = num_interfaces * 2 * 46;
        cp = malloc(ailen as size_t) as *mut libc::c_char;
        if cp.is_null() {
            //define sudo_debug_printf(SUDO_DEBUG_ERROR|SUDO_DEBUG_LINENO,"unable to allocate memory");
            sudo_debug_printf!(
                SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
                b"unable to allocate memory" as *const u8 as *const libc::c_char
            );
            //end of define
            num_interfaces = -1;
            break 'done;
        }

        *addrinfo = cp;

        ifa = ifaddrs;
        while !ifa.is_null() {
            //same as line 164 for
            if ((*ifa).ifa_addr).is_null()
                || ((*ifa).ifa_netmask).is_null()
                || (*ifa).ifa_flags & 1 == 0
                || (*ifa).ifa_flags & 8 != 0
            {
                ifa = (*ifa).ifa_next;
                continue;
            }

            match (*(*ifa).ifa_addr).sa_family as libc::c_int {
                2 => {
                    sin = (*ifa).ifa_addr as *mut sockaddr_in;
                    if inet_ntop(
                        2,
                        &mut (*sin).sin_addr as *mut in_addr as *const libc::c_void,
                        addrstr.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 46]>() as socklen_t,
                    )
                    .is_null()
                    {
                        ifa = (*ifa).ifa_next;
                        continue;
                    }
                    sin = (*ifa).ifa_netmask as *mut sockaddr_in;
                    if inet_ntop(
                        2,
                        &mut (*sin).sin_addr as *mut in_addr as *const libc::c_void,
                        maskstr.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 46]>() as socklen_t,
                    )
                    .is_null()
                    {
                        ifa = (*ifa).ifa_next;
                        continue;
                    }

                    //wait write line 180
                    len = snprintf(
                        cp,
                        (ailen - (*addrinfo).offset_from(cp) as i32) as size_t,
                        b"%s%s/%s\0" as *const u8 as *const libc::c_char,
                        if cp == *addrinfo {
                            b"\0" as *const u8 as *const libc::c_char
                        } else {
                            b" \0" as *const u8 as *const libc::c_char
                        },
                        addrstr.as_mut_ptr(),
                        maskstr.as_mut_ptr(),
                    );

                    if len < 0 || len >= ailen - ((*addrinfo).offset_from(cp) as i32) {
                        //define sudo_warnx(U_("internal error,%s overflow"),__func__);
                        sudo_debug_printf!(
                            SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
                            sudo_warn_gettext_v1(
                                b"sudo\0" as *const u8 as *const libc::c_char,
                                b"internal error,%s overflow\0" as *const u8 as *const libc::c_char
                            ),
                            function_name!()
                        );
                        sudo_warn_nodebug_v1(
                            sudo_warn_gettext_v1(
                                b"sudo\0" as *const u8 as *const libc::c_char,
                                b"internal error,%s overflow\0" as *const u8 as *const libc::c_char,
                            ),
                            function_name!(),
                        );
                        //end of define
                        break 'done;
                    }
                    cp = cp.offset(len as isize);
                } //end of match 2
                10 => {
                    sin6 = (*ifa).ifa_addr as *mut sockaddr_in6;
                    if inet_ntop(
                        10,
                        &mut (*sin6).sin6_addr as *mut in6_addr as *const libc::c_void,
                        addrstr.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 46]>() as socklen_t,
                    )
                    .is_null()
                    {
                        ifa = (*ifa).ifa_next;
                        continue;
                    }

                    sin6 = (*ifa).ifa_netmask as *mut sockaddr_in6;
                    if inet_ntop(
                        10,
                        &mut (*sin6).sin6_addr as *mut in6_addr as *const libc::c_void,
                        maskstr.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 46]>() as socklen_t,
                    )
                    .is_null()
                    {
                        ifa = (*ifa).ifa_next;
                        continue;
                    }

                    len = snprintf(
                        cp,
                        (ailen - (*addrinfo).offset_from(cp) as i32) as size_t,
                        b"%s%s/%s\0" as *const u8 as *const libc::c_char,
                        if cp == *addrinfo {
                            b"\0" as *const u8 as *const libc::c_char
                        } else {
                            b" \0" as *const u8 as *const libc::c_char
                        },
                        addrstr.as_mut_ptr(),
                        maskstr.as_mut_ptr(),
                    );

                    if len < 0 || len >= ailen - ((*addrinfo).offset_from(cp) as i32) {
                        //define sudo_warnx(U_("internal error,%s overflow"),__func__);
                        sudo_debug_printf!(
                            SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO,
                            sudo_warn_gettext_v1(
                                b"sudo\0" as *const u8 as *const libc::c_char,
                                b"internal error,%s overflow\0" as *const u8 as *const libc::c_char
                            ),
                            function_name!()
                        );
                        sudo_warn_nodebug_v1(
                            sudo_warn_gettext_v1(
                                b"sudo\0" as *const u8 as *const libc::c_char,
                                b"internal error,%s overflow\0" as *const u8 as *const libc::c_char,
                            ),
                            function_name!(),
                        );
                        //end of define
                        break 'done;
                    }
                    cp = cp.offset(len as isize);
                }
                _ => {}
            }

            ifa = (*ifa).ifa_next;
        } //same as while, as for
        break 'done;
    }

    freeifaddrs(ifaddrs);

    //dedine debug_return_int(num_interfaces);
    debug_return_int!(num_interfaces);
    //end of define
}
