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
    unused_mut
)]
use crate::common::*;
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn inet_pton(
        __af: libc::c_int,
        __cp: *const libc::c_char,
        __buf: *mut libc::c_void,
    ) -> libc::c_int;
    fn sudo_warnx_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_strsplit_v1(
        str: *const libc::c_char,
        endstr: *const libc::c_char,
        sep: *const libc::c_char,
        last: *mut *const libc::c_char,
    ) -> *const libc::c_char;
    static mut sudo_printf:
        Option<unsafe extern "C" fn(libc::c_int, *const libc::c_char, ...) -> libc::c_int>;
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    static mut sudoers_subsystem_ids: [libc::c_uint; 0];
}



pub type sudo_printf_t =
    Option<unsafe extern "C" fn(libc::c_int, *const libc::c_char, ...) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sudo_in_addr_un {
    pub ip4: in_addr,
    pub ip6: in6_addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct interface {
    pub entries: C2RustUnnamed_0,
    pub family: libc::c_uint,
    pub addr: sudo_in_addr_un,
    pub netmask: sudo_in_addr_un,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub sle_next: *mut interface,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct interface_list {
    pub slh_first: *mut interface,
}
static mut interfaces: interface_list = {
    let mut init = interface_list {
        slh_first: 0 as *const interface as *mut interface,
    };
    init
};
/*
 * Parse a space-delimited list of IP address/netmask pairs and
 * store in a list of interface structures.  Returns true on
 * success and false on parse error or memory allocation error.
 */
#[no_mangle]
pub unsafe extern "C" fn set_interfaces(mut ai: *const libc::c_char) -> bool {
    let mut addrinfo: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut addr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mask: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut last: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ifp: *mut interface = 0 as *mut interface;
    let mut ret: bool = 0 as libc::c_int != 0;
    debug_decl!(SUDOERS_DEBUG_NETIF!());
    addrinfo = strdup(ai);
    if addrinfo.is_null() {
        debug_return_bool!(false);
    }
    addr = strtok_r(
        addrinfo,
        b" \t\0" as *const u8 as *const libc::c_char,
        &mut last,
    );
    'done: loop {
        loop {
            if addr.is_null() {
                break;
            }
            /* Separate addr and mask. */
            mask = strchr(addr, '/' as i32);
            if !mask.is_null() {
                let fresh0 = mask;
                mask = mask.offset(1);
                *fresh0 = '\0' as i32 as libc::c_char;
                /* Parse addr and store in list. */
                ifp = calloc(
                    1 as libc::c_int as libc::c_ulong,
                    ::core::mem::size_of::<interface>() as libc::c_ulong,
                ) as *mut interface;
                if ifp.is_null() {
                    sudo_warnx!(
                        b"%s: %s\0" as *const u8 as *const libc::c_char,
                        get_function_name!(),
                        b"unable to allocate memory\0" as *const u8 as *const libc::c_char
                    );
                    break 'done;
                }
                if !(strchr(addr, ':' as i32)).is_null() {
                    /* IPv6 */
                    (*ifp).family = AF_INET6 as libc::c_uint;
                    if inet_pton(
                        AF_INET6,
                        addr,
                        &mut (*ifp).addr.ip6 as *mut in6_addr as *mut libc::c_void,
                    ) != 1
                    {
                        sudo_warnx!(
                            b"unable to parse IP address \"%s\"\0" as *const u8
                                as *const libc::c_char,
                            addr
                        );
                        free(ifp as *mut libc::c_void);
                        break 'done;
                    }
                    if inet_pton(
                        AF_INET6,
                        mask,
                        &mut (*ifp).netmask.ip6 as *mut in6_addr as *mut libc::c_void,
                    ) != 1
                    {
                        sudo_warnx!(
                            b"unable to parse netmask \"%s\"\0" as *const u8 as *const libc::c_char,
                            mask
                        );
                        free(ifp as *mut libc::c_void);
                        break 'done;
                    }
                } else {
                    /* IPv4 */
                    (*ifp).family = AF_INET as libc::c_uint;
                    if inet_pton(
                        AF_INET,
                        addr,
                        &mut (*ifp).addr.ip4 as *mut in_addr as *mut libc::c_void,
                    ) != 1
                    {
                        sudo_warnx!(
                            b"unable to parse IP address \"%s\"\0" as *const u8
                                as *const libc::c_char,
                            addr
                        );
                        free(ifp as *mut libc::c_void);
                        break 'done;
                    }
                    if inet_pton(
                        AF_INET,
                        mask,
                        &mut (*ifp).netmask.ip4 as *mut in_addr as *mut libc::c_void,
                    ) != 1
                    {
                        sudo_warnx!(
                            b"unable to parse netmask \"%s\"\0" as *const u8 as *const libc::c_char,
                            mask
                        );
                        free(ifp as *mut libc::c_void);
                        break 'done;
                    }
                }
                (*ifp).entries.sle_next = interfaces.slh_first;
                interfaces.slh_first = ifp;
            }
            addr = strtok_r(
                0 as *mut libc::c_char,
                b" \t\0" as *const u8 as *const libc::c_char,
                &mut last,
            );
        } // !loop
        ret = true;
        break 'done;
    } // ! done loop
    free(addrinfo as *mut libc::c_void);
    debug_return_bool!(ret);
}


#[no_mangle]
pub unsafe extern "C" fn get_interfaces() -> *mut interface_list {
    return &mut interfaces;
}
#[no_mangle]
pub unsafe extern "C" fn dump_interfaces(mut ai: *const libc::c_char) {
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    let mut ep: *const libc::c_char = 0 as *const libc::c_char;
    let mut ai_end: *const libc::c_char = ai.offset(strlen(ai) as isize);
    debug_decl!(SUDOERS_DEBUG_NETIF!());
    sudo_printf.expect("non-null function pointer")(
        SUDO_CONV_INFO_MSG,
        b"Local IP address and netmask pairs:\n\0" as *const u8 as *const libc::c_char,
    );
    cp = sudo_strsplit_v1(
        ai,
        ai_end,
        b" \t\0" as *const u8 as *const libc::c_char,
        &mut ep,
    );
    while !cp.is_null() {
        sudo_printf.expect("non-null function pointer")(
            SUDO_CONV_INFO_MSG,
            b"\t%.*s\n\0" as *const u8 as *const libc::c_char,
            ep.offset_from(cp) as libc::c_long as libc::c_int,
            cp,
        );
        cp = sudo_strsplit_v1(
            0 as *const libc::c_char,
            ai_end,
            b" \t\0" as *const u8 as *const libc::c_char,
            &mut ep,
        );
    }
    debug_return!();
}
