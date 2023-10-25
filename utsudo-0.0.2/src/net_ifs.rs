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


