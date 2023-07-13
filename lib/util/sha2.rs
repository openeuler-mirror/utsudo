/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unused_must_use
)]
extern "C" {
    fn memset()
    fn memcpy()
    fn sudo_memset_s()
}

pub type __uint64_t = libc::c_ulong;
pub type uint64_t = __uint64_t;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint64_t;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type size_t = libc::c_ulong;
pub const SHA512_BLOCK_LENGTH: usize = 128;
pub const SHA256_BLOCK_LENGTH: libc::c_int = 64;
pub struct SHA2_CTX {
    pub state: state,
    pub count: [uint64_t; 2],
    pub buffer: [uint8_t; SHA512_BLOCK_LENGTH],
}
pub union state {
    pub st32: [uint32_t; 8],
    pub st64: [uint64_t; 8],
}

pub unsafe extern "C" fn sudo_SHA224Init

pub unsafe extern "C" fn sudo_SHA224Pad

pub unsafe extern "C" fn sudo_SHA224Final


pub unsafe extern "C" fn sudo_SHA256Init

pub unsafe extern "C" fn sudo_SHA256Transform

pub unsafe extern "C" fn sudo_SHA256Update

pub unsafe extern "C" fn sudo_SHA256Pad

pub unsafe extern "C" fn sudo_SHA256Final

pub unsafe extern "C" fn sudo_SHA384Init

pub unsafe extern "C" fn sudo_SHA384Transform

pub unsafe extern "C" fn sudo_SHA384Update

pub unsafe extern "C" fn sudo_SHA384Pad

pub unsafe extern "C" fn sudo_SHA384Final

pub unsafe extern "C" fn sudo_SHA512Init

pub unsafe extern "C" fn sudo_SHA512Transform

pub unsafe extern "C" fn sudo_SHA512Update

pub unsafe extern "C" fn sudo_SHA512Pad

pub unsafe extern "C" fn sudo_SHA512Final
