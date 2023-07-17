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
    fn memset(__dest: *mut libc::c_void, __ch: libc::c_int, __len: size_t) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn sudo_memset_s(v: *mut libc::c_void, smax: size_t, c: libc::c_int, n: size_t) -> libc::c_int;
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
macro_rules! BE8TO32 {
    ($x:expr,$y:expr) => {
        $x = ((*$y.offset(0 as isize) & 255) as uint32_t) << 24
            | ((*$y.offset(1 as isize) & 255) as uint32_t) << 16
            | ((*$y.offset(2 as isize) & 255) as uint32_t) << 8
            | ((*$y.offset(3 as isize) & 255) as uint32_t)
    };
}
macro_rules! BE8TO64 {
    ($x:expr,$y:expr) => {
        $x = ((*$y.offset(0 as isize) & 255) as uint64_t) << 56
            | ((*$y.offset(1 as isize) & 255) as uint64_t) << 48
            | ((*$y.offset(2 as isize) & 255) as uint64_t) << 40
            | ((*$y.offset(3 as isize) & 255) as uint64_t) << 32
            | ((*$y.offset(4 as isize) & 255) as uint64_t) << 24
            | ((*$y.offset(5 as isize) & 255) as uint64_t) << 16
            | ((*$y.offset(6 as isize) & 255) as uint64_t) << 8
            | ((*$y.offset(7 as isize) & 255) as uint64_t)
    };
}
macro_rules! BE32TO8 {
    ($x:expr,$y:expr) => {
        *$x.offset(0 as isize) = ($y >> 24 & 255) as uint8_t;
        *$x.offset(1 as isize) = ($y >> 16 & 255) as uint8_t;
        *$x.offset(2 as isize) = ($y >> 8 & 255) as uint8_t;
        *$x.offset(3 as isize) = ($y & 255) as uint8_t;
    };
}
macro_rules! BE64TO8 {
    ($x:expr,$y:expr) => {
        *$x.offset(0 as isize) = ($y >> 56 & 255 as libc::c_ulong) as uint8_t;
        *$x.offset(1 as isize) = ($y >> 48 & 255 as libc::c_ulong) as uint8_t;
        *$x.offset(2 as isize) = ($y >> 40 & 255 as libc::c_ulong) as uint8_t;
        *$x.offset(3 as isize) = ($y >> 32 & 255 as libc::c_ulong) as uint8_t;
        *$x.offset(4 as isize) = ($y >> 24 & 255 as libc::c_ulong) as uint8_t;
        *$x.offset(5 as isize) = ($y >> 16 & 255 as libc::c_ulong) as uint8_t;
        *$x.offset(6 as isize) = ($y >> 8 & 255 as libc::c_ulong) as uint8_t;
        *$x.offset(7 as isize) = ($y & 255 as libc::c_ulong) as uint8_t;
    };
}
macro_rules! BE64TO81 {
    ($x:expr,$y:expr) => {
        $x[0 as usize] = ($y >> 56 & 255 as libc::c_ulong) as uint8_t;
        $x[1 as usize] = ($y >> 48 & 255 as libc::c_ulong) as uint8_t;
        $x[2 as usize] = ($y >> 40 & 255 as libc::c_ulong) as uint8_t;
        $x[3 as usize] = ($y >> 32 & 255 as libc::c_ulong) as uint8_t;
        $x[4 as usize] = ($y >> 24 & 255 as libc::c_ulong) as uint8_t;
        $x[5 as usize] = ($y >> 16 & 255 as libc::c_ulong) as uint8_t;
        $x[6 as usize] = ($y >> 8 & 255 as libc::c_ulong) as uint8_t;
        $x[7 as usize] = ($y & 255 as libc::c_ulong) as uint8_t;
    };
}
macro_rules! blk0 {
    ($x:expr) => {
        W[$x as usize]
    };
}
macro_rules! blk2 {
    ($x:expr) => {
        (W[($x & 15) as usize]).wrapping_add(
            (s1!(W[($x - 2 & 15) as usize]))
                .wrapping_add(W[($x - 7 & 15) as usize])
                .wrapping_add(s0!(W[($x - 15 & 15) as usize])),
        ) as uint32_t as uint32_t
    };
}
macro_rules! Ch {
    ($x:expr,$y:expr,$z:expr) => {
        $z ^ ($x & ($y ^ $z))
    };
}
macro_rules! Maj {
    ($x:expr,$y:expr,$z:expr) => {
        $y ^ (($x ^ $y) & ($y ^ $z))
    };
}
macro_rules! a {
    ($x:expr) => {
        T[(0 - $x & 7) as usize]
    };
}
macro_rules! b {
    ($x:expr) => {
        T[(1 - $x & 7) as usize]
    };
}
macro_rules! c {
    ($x:expr) => {
        T[(2 - $x & 7) as usize]
    };
}
macro_rules! d {
    ($x:expr) => {
        T[(3 - $x & 7) as usize]
    };
}
macro_rules! e {
    ($x:expr) => {
        T[(4 - $x & 7) as usize]
    };
}
macro_rules! f {
    ($x:expr) => {
        T[(5 - $x & 7) as usize]
    };
}
macro_rules! g {
    ($x:expr) => {
        T[(6 - $x & 7) as usize]
    };
}
macro_rules! h {
    ($x:expr) => {
        T[(7 - $x & 7) as usize]
    };
}

#[no_mangle]
pub unsafe extern "C" fn sudo_SHA224Init(mut ctx: *mut SHA2_CTX) {
    memset(
        ctx as *mut libc::c_void,
        0,
        std::mem::size_of::<SHA2_CTX>() as libc::c_ulong,
    );
    (*ctx).state.st32[0] = 0xc1059ed8 as libc::c_ulong as uint32_t;
    (*ctx).state.st32[1] = 0x367cd507 as libc::c_ulong as uint32_t;
    (*ctx).state.st32[2] = 0x3070dd17 as libc::c_ulong as uint32_t;
    (*ctx).state.st32[3] = 0xf70e5939 as libc::c_ulong as uint32_t;
    (*ctx).state.st32[4] = 0xffc00b31 as libc::c_ulong as uint32_t;
    (*ctx).state.st32[5] = 0x68581511 as libc::c_ulong as uint32_t;
    (*ctx).state.st32[6] = 0x64f98fa7 as libc::c_ulong as uint32_t;
    (*ctx).state.st32[7] = 0xbefa4fa4 as libc::c_ulong as uint32_t;
}

#[no_mangle]
pub unsafe extern "C" fn sudo_SHA224Transform(
    mut state: *mut uint32_t,
    mut buffer: *const uint8_t,
) {
    sudo_SHA256Transform(state, buffer);
}

#[no_mangle]
pub unsafe extern "C" fn sudo_SHA224Update(
    mut ctx: *mut SHA2_CTX,
    mut data: *const uint8_t,
    mut len: size_t,
) {
    sudo_SHA256Update(ctx, data, len);
}

#[no_mangle]
pub unsafe extern "C" fn sudo_SHA224Pad(mut ctx: *mut SHA2_CTX) {
    sudo_SHA256Pad(ctx);
}

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
