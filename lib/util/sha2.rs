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

#[no_mangle]
pub unsafe extern "C" fn sudo_SHA224Final(mut digest: *mut uint8_t, mut ctx: *mut SHA2_CTX) {
    sudo_SHA256Pad(ctx);
    if !digest.is_null() {
        let mut i: libc::c_uint = 0;
        i = 0;
        while i < 7 as libc::c_uint {
            BE32TO8!(
                digest.offset(i.wrapping_mul(4 as libc::c_uint) as isize),
                (*ctx).state.st32[i as usize]
            );
            i = i.wrapping_add(1);
        }
        memset(
            ctx as *mut libc::c_void,
            0,
            std::mem::size_of::<SHA2_CTX>() as libc::c_ulong,
        );
    }
}

static mut SHA256_K: [uint32_t; 64] = [
    0x428a2f98 as libc::c_ulong as uint32_t,
    0x71374491 as libc::c_ulong as uint32_t,
    0xb5c0fbcf as libc::c_ulong as uint32_t,
    0xe9b5dba5 as libc::c_ulong as uint32_t,
    0x3956c25b as libc::c_ulong as uint32_t,
    0x59f111f1 as libc::c_ulong as uint32_t,
    0x923f82a4 as libc::c_ulong as uint32_t,
    0xab1c5ed5 as libc::c_ulong as uint32_t,
    0xd807aa98 as libc::c_ulong as uint32_t,
    0x12835b01 as libc::c_ulong as uint32_t,
    0x243185be as libc::c_ulong as uint32_t,
    0x550c7dc3 as libc::c_ulong as uint32_t,
    0x72be5d74 as libc::c_ulong as uint32_t,
    0x80deb1fe as libc::c_ulong as uint32_t,
    0x9bdc06a7 as libc::c_ulong as uint32_t,
    0xc19bf174 as libc::c_ulong as uint32_t,
    0xe49b69c1 as libc::c_ulong as uint32_t,
    0xefbe4786 as libc::c_ulong as uint32_t,
    0xfc19dc6 as libc::c_ulong as uint32_t,
    0x240ca1cc as libc::c_ulong as uint32_t,
    0x2de92c6f as libc::c_ulong as uint32_t,
    0x4a7484aa as libc::c_ulong as uint32_t,
    0x5cb0a9dc as libc::c_ulong as uint32_t,
    0x76f988da as libc::c_ulong as uint32_t,
    0x983e5152 as libc::c_ulong as uint32_t,
    0xa831c66d as libc::c_ulong as uint32_t,
    0xb00327c8 as libc::c_ulong as uint32_t,
    0xbf597fc7 as libc::c_ulong as uint32_t,
    0xc6e00bf3 as libc::c_ulong as uint32_t,
    0xd5a79147 as libc::c_ulong as uint32_t,
    0x6ca6351 as libc::c_ulong as uint32_t,
    0x14292967 as libc::c_ulong as uint32_t,
    0x27b70a85 as libc::c_ulong as uint32_t,
    0x2e1b2138 as libc::c_ulong as uint32_t,
    0x4d2c6dfc as libc::c_ulong as uint32_t,
    0x53380d13 as libc::c_ulong as uint32_t,
    0x650a7354 as libc::c_ulong as uint32_t,
    0x766a0abb as libc::c_ulong as uint32_t,
    0x81c2c92e as libc::c_ulong as uint32_t,
    0x92722c85 as libc::c_ulong as uint32_t,
    0xa2bfe8a1 as libc::c_ulong as uint32_t,
    0xa81a664b as libc::c_ulong as uint32_t,
    0xc24b8b70 as libc::c_ulong as uint32_t,
    0xc76c51a3 as libc::c_ulong as uint32_t,
    0xd192e819 as libc::c_ulong as uint32_t,
    0xd6990624 as libc::c_ulong as uint32_t,
    0xf40e3585 as libc::c_ulong as uint32_t,
    0x106aa070 as libc::c_ulong as uint32_t,
    0x19a4c116 as libc::c_ulong as uint32_t,
    0x1e376c08 as libc::c_ulong as uint32_t,
    0x2748774c as libc::c_ulong as uint32_t,
    0x34b0bcb5 as libc::c_ulong as uint32_t,
    0x391c0cb3 as libc::c_ulong as uint32_t,
    0x4ed8aa4a as libc::c_ulong as uint32_t,
    0x5b9cca4f as libc::c_ulong as uint32_t,
    0x682e6ff3 as libc::c_ulong as uint32_t,
    0x748f82ee as libc::c_ulong as uint32_t,
    0x78a5636f as libc::c_ulong as uint32_t,
    0x84c87814 as libc::c_ulong as uint32_t,
    0x8cc70208 as libc::c_ulong as uint32_t,
    0x90befffa as libc::c_ulong as uint32_t,
    0xa4506ceb as libc::c_ulong as uint32_t,
    0xbef9a3f7 as libc::c_ulong as uint32_t,
    0xc67178f2 as libc::c_ulong as uint32_t,
];

macro_rules! rotrFixed {
    ($x:expr,$y:expr) => {
        if $y != 0 {
            $x >> $y
                | $x << (std::mem::size_of::<uint32_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_ulong)
                    .wrapping_sub($y as libc::c_ulong)
        } else {
            $x
        }
    };
}
macro_rules! S0 {
    ($x:expr) => {
        (rotrFixed!($x, 2)) ^ (rotrFixed!($x, 13)) ^ (rotrFixed!($x, 22))
    };
}
macro_rules! S1 {
    ($x:expr) => {
        (rotrFixed!($x, 6)) ^ (rotrFixed!($x, 11)) ^ (rotrFixed!($x, 25))
    };
}
macro_rules! s1 {
    ($x:expr) => {
        (rotrFixed!($x, 17)) ^ (rotrFixed!($x, 19)) ^ $x >> 10
    };
}
macro_rules! s0 {
    ($x:expr) => {
        (rotrFixed!($x, 7)) ^ (rotrFixed!($x, 18)) ^ $x >> 3
    };
}
macro_rules! R {
    ($x:expr) => {
        h!($x) = (h!($x)).wrapping_add(
            (S1!(e!($x)))
                .wrapping_add(Ch!(e!($x), f!($x), g!($x)))
                .wrapping_add(SHA256_K[($x as libc::c_uint).wrapping_add(j) as usize])
                .wrapping_add((if j != 0 { blk2!($x) } else { blk0!($x) })),
        ) as uint32_t as uint32_t;

        d!($x) = (d!($x)).wrapping_add(h!($x)) as uint32_t as uint32_t;

        h!($x) = (h!($x) as libc::c_uint).wrapping_add((S0!(a!($x))).wrapping_add(Maj!(
            a!($x),
            b!($x),
            c!($x)
        )) as u32) as uint32_t as uint32_t;
    };
}

#[no_mangle]
pub unsafe extern "C" fn sudo_SHA256Init(mut ctx: *mut SHA2_CTX) {
    memset(
        ctx as *mut libc::c_void,
        0,
        std::mem::size_of::<SHA2_CTX>() as libc::c_ulong,
    );
    (*ctx).state.st32[0] = 0x6a09e667 as libc::c_ulong as uint32_t;
    (*ctx).state.st32[1] = 0xbb67ae85 as libc::c_ulong as uint32_t;
    (*ctx).state.st32[2] = 0x3c6ef372 as libc::c_ulong as uint32_t;
    (*ctx).state.st32[3] = 0xa54ff53a as libc::c_ulong as uint32_t;
    (*ctx).state.st32[4] = 0x510e527f as libc::c_ulong as uint32_t;
    (*ctx).state.st32[5] = 0x9b05688c as libc::c_ulong as uint32_t;
    (*ctx).state.st32[6] = 0x1f83d9ab as libc::c_ulong as uint32_t;
    (*ctx).state.st32[7] = 0x5be0cd19 as libc::c_ulong as uint32_t;
}

#[no_mangle]
pub unsafe extern "C" fn sudo_SHA256Transform(mut state: *mut uint32_t, mut data: *const uint8_t) {
    static mut W: [uint32_t; 16] = [0; 16];
    static mut T: [uint32_t; 8] = [0; 8];
    static mut j: libc::c_uint = 0;

    /* Copy context state to working vars. */
    memcpy(
        T.as_mut_ptr() as *mut libc::c_void,
        state as *const libc::c_void,
        std::mem::size_of::<[uint32_t; 8]>() as libc::c_ulong,
    );
    while j < 16 {
        BE8TO32!(W[j as usize], data);
        data = data.offset(4 as isize);
        j = j.wrapping_add(1);
    }
    /* 64 operations, partially loop unrolled. */
    while j < 64 {
        R!(0);
        R!(1);
        R!(2);
        R!(3);
        R!(4);
        R!(5);
        R!(6);
        R!(7);
        R!(8);
        R!(9);
        R!(10);
        R!(11);
        R!(12);
        R!(13);
        R!(14);
        R!(15);
        j = j.wrapping_add(16);
    }
    /* Add the working vars back into context state. */
    let ref mut state0 = *state.offset(0 as isize);
    *state0 = (*state0 as libc::c_uint).wrapping_add(a!(0) as libc::c_uint) as uint32_t as uint32_t;
    let ref mut state1 = *state.offset(0 as isize);
    *state1 = (*state1 as libc::c_uint).wrapping_add(b!(0) as libc::c_uint) as uint32_t as uint32_t;
    let ref mut state2 = *state.offset(0 as isize);
    *state2 = (*state2 as libc::c_uint).wrapping_add(c!(0) as libc::c_uint) as uint32_t as uint32_t;
    let ref mut state3 = *state.offset(0 as isize);
    *state3 = (*state3 as libc::c_uint).wrapping_add(d!(0) as libc::c_uint) as uint32_t as uint32_t;
    let ref mut state4 = *state.offset(0 as isize);
    *state4 = (*state4 as libc::c_uint).wrapping_add(e!(0) as libc::c_uint) as uint32_t as uint32_t;
    let ref mut state5 = *state.offset(0 as isize);
    *state5 = (*state5 as libc::c_uint).wrapping_add(f!(0) as libc::c_uint) as uint32_t as uint32_t;
    let ref mut state6 = *state.offset(0 as isize);
    *state6 = (*state6 as libc::c_uint).wrapping_add(g!(0) as libc::c_uint) as uint32_t as uint32_t;
    let ref mut state7 = *state.offset(0 as isize);
    *state7 = (*state7 as libc::c_uint).wrapping_add(h!(0) as libc::c_uint) as uint32_t as uint32_t;

    /* Cleanup */
    sudo_memset_s(
        T.as_mut_ptr() as *mut libc::c_void,
        std::mem::size_of::<[uint32_t; 8]>() as libc::c_ulong,
        0,
        std::mem::size_of::<[uint32_t; 8]>() as libc::c_ulong,
    );
    sudo_memset_s(
        W.as_mut_ptr() as *mut libc::c_void,
        std::mem::size_of::<[uint32_t; 16]>() as libc::c_ulong,
        0,
        std::mem::size_of::<[uint32_t; 16]>() as libc::c_ulong,
    );
}

#[no_mangle]
pub unsafe extern "C" fn sudo_SHA256Update(
    mut ctx: *mut SHA2_CTX,
    mut data: *const uint8_t,
    mut len: size_t,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    j = (*ctx).count[0 as usize] >> 3 & (SHA256_BLOCK_LENGTH - 1) as libc::c_ulong;
    let ref mut ctx0 = (*ctx).count[0 as usize];
    *ctx0 = (*ctx0 as libc::c_ulong).wrapping_add(len << 3) as uint64_t as uint64_t;

    if j.wrapping_add(len) > (SHA256_BLOCK_LENGTH - 1) as libc::c_ulong {
        i = (SHA256_BLOCK_LENGTH as libc::c_ulong).wrapping_sub(j);
        memcpy(
            &mut *((*ctx).buffer).as_mut_ptr().offset(j as isize) as *mut uint8_t
                as *mut libc::c_void,
            data as *const libc::c_void,
            i,
        );
        sudo_SHA256Transform(
            ((*ctx).state.st32).as_mut_ptr(),
            ((*ctx).buffer).as_mut_ptr() as *const uint8_t,
        );
        while i
            .wrapping_add(SHA256_BLOCK_LENGTH as libc::c_ulong)
            .wrapping_sub(1 as libc::c_ulong)
            < len
        {
            sudo_SHA256Transform(
                ((*ctx).state.st32).as_mut_ptr(),
                &*data.offset(i as isize) as *const uint8_t as *mut uint8_t as *const uint8_t,
            );
            i = (i as libc::c_ulong).wrapping_add(SHA256_BLOCK_LENGTH as libc::c_ulong) as size_t
                as size_t;
        }
        j = 0;
    }
    memcpy(
        &mut *((*ctx).buffer).as_mut_ptr().offset(j as isize) as *mut uint8_t as *mut libc::c_void,
        &*data.offset(i as isize) as *const uint8_t as *const libc::c_void,
        len.wrapping_sub(i),
    );
}

#[no_mangle]
pub unsafe extern "C" fn sudo_SHA256Pad(mut ctx: *mut SHA2_CTX) {
    let mut finalcount: [uint8_t; 8] = [0; 8];

    /* Store unpadded message length in bits in big endian format. */
    BE64TO81!(finalcount, (*ctx).count[0 as usize]);

    /* Append a '1' bit (0x80) to the message. */
    sudo_SHA256Update(
        ctx,
        b"\x80\0" as *const u8 as *const libc::c_char as *mut uint8_t,
        1 as size_t,
    );

    /* Pad message such that the resulting length modulo 512 is 448. */
    while (*ctx).count[0] & 504 != 448 {
        sudo_SHA256Update(
            ctx,
            b"\0\0" as *const u8 as *const libc::c_char as *mut uint8_t,
            1 as size_t,
        );
    }

    /* Append length of message in bits and do final SHA256Transform(). */
    sudo_SHA256Update(
        ctx,
        finalcount.as_mut_ptr(),
        std::mem::size_of::<[uint8_t; 8]>() as libc::c_ulong,
    );
}

#[no_mangle]
pub unsafe extern "C" fn sudo_SHA256Final(mut digest: *mut uint8_t, mut ctx: *mut SHA2_CTX) {
    sudo_SHA256Pad(ctx);
    if !digest.is_null() {
        let mut i: libc::c_uint = 0;
        i = 0;
        while i < 8 {
            BE32TO8!(
                digest.offset(i.wrapping_mul(4) as isize),
                (*ctx).state.st32[i as usize]
            );
            i = i.wrapping_add(1);
        }
        memset(
            ctx as *mut libc::c_void,
            0,
            std::mem::size_of::<SHA2_CTX>() as libc::c_ulong,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn sudo_SHA384Init(mut ctx: *mut SHA2_CTX) {
    memset(
        ctx as *mut libc::c_void,
        0,
        std::mem::size_of::<SHA2_CTX>() as libc::c_ulong,
    );
    (*ctx).state.st64[0] = 0xcbbb9d5dc1059ed8 as libc::c_ulonglong as uint64_t;
    (*ctx).state.st64[1] = 0x629a292a367cd507 as libc::c_ulonglong as uint64_t;
    (*ctx).state.st64[2] = 0x9159015a3070dd17 as libc::c_ulonglong as uint64_t;
    (*ctx).state.st64[3] = 0x152fecd8f70e5939 as libc::c_ulonglong as uint64_t;
    (*ctx).state.st64[4] = 0x67332667ffc00b31 as libc::c_ulonglong as uint64_t;
    (*ctx).state.st64[5] = 0x8eb44a8768581511 as libc::c_ulonglong as uint64_t;
    (*ctx).state.st64[6] = 0xdb0c2e0d64f98fa7 as libc::c_ulonglong as uint64_t;
    (*ctx).state.st64[7] = 0x47b5481dbefa4fa4 as libc::c_ulonglong as uint64_t;
}

#[no_mangle]
pub unsafe extern "C" fn sudo_SHA384Transform(mut state: *mut uint64_t, mut data: *const uint8_t) {
    sudo_SHA512Transform(state, data);
}

#[no_mangle]
pub unsafe extern "C" fn sudo_SHA384Update(
    mut ctx: *mut SHA2_CTX,
    mut data: *const uint8_t,
    mut len: size_t,
) {
    sudo_SHA512Update(ctx, data, len);
}

#[no_mangle]
pub unsafe extern "C" fn sudo_SHA384Pad(mut ctx: *mut SHA2_CTX) {
    sudo_SHA512Pad(ctx);
}

#[no_mangle]
pub unsafe extern "C" fn sudo_SHA384Final(mut digest: *mut uint8_t, mut ctx: *mut SHA2_CTX) {
    sudo_SHA384Pad(ctx);
    if !digest.is_null() {
        let mut i: libc::c_uint = 0;
        while i < 6 {
            BE64TO8!(
                digest.offset(i.wrapping_mul(8 as libc::c_uint) as isize),
                (*ctx).state.st64[i as usize]
            );
            i.wrapping_add(1 as libc::c_uint);
        }
        memset(
            ctx as *mut libc::c_void,
            0,
            std::mem::size_of::<SHA2_CTX>() as libc::c_ulong,
        );
    }
}

static mut SHA512_K: [uint64_t; 80] = [
    0x428a2f98d728ae22 as libc::c_ulonglong as uint64_t,
    0x7137449123ef65cd as libc::c_ulonglong as uint64_t,
    0xb5c0fbcfec4d3b2f as libc::c_ulonglong as uint64_t,
    0xe9b5dba58189dbbc as libc::c_ulonglong as uint64_t,
    0x3956c25bf348b538 as libc::c_ulonglong as uint64_t,
    0x59f111f1b605d019 as libc::c_ulonglong as uint64_t,
    0x923f82a4af194f9b as libc::c_ulonglong as uint64_t,
    0xab1c5ed5da6d8118 as libc::c_ulonglong as uint64_t,
    0xd807aa98a3030242 as libc::c_ulonglong as uint64_t,
    0x12835b0145706fbe as libc::c_ulonglong as uint64_t,
    0x243185be4ee4b28c as libc::c_ulonglong as uint64_t,
    0x550c7dc3d5ffb4e2 as libc::c_ulonglong as uint64_t,
    0x72be5d74f27b896f as libc::c_ulonglong as uint64_t,
    0x80deb1fe3b1696b1 as libc::c_ulonglong as uint64_t,
    0x9bdc06a725c71235 as libc::c_ulonglong as uint64_t,
    0xc19bf174cf692694 as libc::c_ulonglong as uint64_t,
    0xe49b69c19ef14ad2 as libc::c_ulonglong as uint64_t,
    0xefbe4786384f25e3 as libc::c_ulonglong as uint64_t,
    0xfc19dc68b8cd5b5 as libc::c_ulonglong as uint64_t,
    0x240ca1cc77ac9c65 as libc::c_ulonglong as uint64_t,
    0x2de92c6f592b0275 as libc::c_ulonglong as uint64_t,
    0x4a7484aa6ea6e483 as libc::c_ulonglong as uint64_t,
    0x5cb0a9dcbd41fbd4 as libc::c_ulonglong as uint64_t,
    0x76f988da831153b5 as libc::c_ulonglong as uint64_t,
    0x983e5152ee66dfab as libc::c_ulonglong as uint64_t,
    0xa831c66d2db43210 as libc::c_ulonglong as uint64_t,
    0xb00327c898fb213f as libc::c_ulonglong as uint64_t,
    0xbf597fc7beef0ee4 as libc::c_ulonglong as uint64_t,
    0xc6e00bf33da88fc2 as libc::c_ulonglong as uint64_t,
    0xd5a79147930aa725 as libc::c_ulonglong as uint64_t,
    0x6ca6351e003826f as libc::c_ulonglong as uint64_t,
    0x142929670a0e6e70 as libc::c_ulonglong as uint64_t,
    0x27b70a8546d22ffc as libc::c_ulonglong as uint64_t,
    0x2e1b21385c26c926 as libc::c_ulonglong as uint64_t,
    0x4d2c6dfc5ac42aed as libc::c_ulonglong as uint64_t,
    0x53380d139d95b3df as libc::c_ulonglong as uint64_t,
    0x650a73548baf63de as libc::c_ulonglong as uint64_t,
    0x766a0abb3c77b2a8 as libc::c_ulonglong as uint64_t,
    0x81c2c92e47edaee6 as libc::c_ulonglong as uint64_t,
    0x92722c851482353b as libc::c_ulonglong as uint64_t,
    0xa2bfe8a14cf10364 as libc::c_ulonglong as uint64_t,
    0xa81a664bbc423001 as libc::c_ulonglong as uint64_t,
    0xc24b8b70d0f89791 as libc::c_ulonglong as uint64_t,
    0xc76c51a30654be30 as libc::c_ulonglong as uint64_t,
    0xd192e819d6ef5218 as libc::c_ulonglong as uint64_t,
    0xd69906245565a910 as libc::c_ulonglong as uint64_t,
    0xf40e35855771202a as libc::c_ulonglong as uint64_t,
    0x106aa07032bbd1b8 as libc::c_ulonglong as uint64_t,
    0x19a4c116b8d2d0c8 as libc::c_ulonglong as uint64_t,
    0x1e376c085141ab53 as libc::c_ulonglong as uint64_t,
    0x2748774cdf8eeb99 as libc::c_ulonglong as uint64_t,
    0x34b0bcb5e19b48a8 as libc::c_ulonglong as uint64_t,
    0x391c0cb3c5c95a63 as libc::c_ulonglong as uint64_t,
    0x4ed8aa4ae3418acb as libc::c_ulonglong as uint64_t,
    0x5b9cca4f7763e373 as libc::c_ulonglong as uint64_t,
    0x682e6ff3d6b2b8a3 as libc::c_ulonglong as uint64_t,
    0x748f82ee5defb2fc as libc::c_ulonglong as uint64_t,
    0x78a5636f43172f60 as libc::c_ulonglong as uint64_t,
    0x84c87814a1f0ab72 as libc::c_ulonglong as uint64_t,
    0x8cc702081a6439ec as libc::c_ulonglong as uint64_t,
    0x90befffa23631e28 as libc::c_ulonglong as uint64_t,
    0xa4506cebde82bde9 as libc::c_ulonglong as uint64_t,
    0xbef9a3f7b2c67915 as libc::c_ulonglong as uint64_t,
    0xc67178f2e372532b as libc::c_ulonglong as uint64_t,
    0xca273eceea26619c as libc::c_ulonglong as uint64_t,
    0xd186b8c721c0c207 as libc::c_ulonglong as uint64_t,
    0xeada7dd6cde0eb1e as libc::c_ulonglong as uint64_t,
    0xf57d4f7fee6ed178 as libc::c_ulonglong as uint64_t,
    0x6f067aa72176fba as libc::c_ulonglong as uint64_t,
    0xa637dc5a2c898a6 as libc::c_ulonglong as uint64_t,
    0x113f9804bef90dae as libc::c_ulonglong as uint64_t,
    0x1b710b35131c471b as libc::c_ulonglong as uint64_t,
    0x28db77f523047d84 as libc::c_ulonglong as uint64_t,
    0x32caab7b40c72493 as libc::c_ulonglong as uint64_t,
    0x3c9ebe0a15c9bebc as libc::c_ulonglong as uint64_t,
    0x431d67c49c100d4c as libc::c_ulonglong as uint64_t,
    0x4cc5d4becb3e42b6 as libc::c_ulonglong as uint64_t,
    0x597f299cfc657e2a as libc::c_ulonglong as uint64_t,
    0x5fcb6fab3ad6faec as libc::c_ulonglong as uint64_t,
    0x6c44198c4a475817 as libc::c_ulonglong as uint64_t,
];

#[no_mangle]
pub unsafe extern "C" fn sudo_SHA512Init(mut ctx: *mut SHA2_CTX) {
    memset(
        ctx as *mut libc::c_void,
        0,
        std::mem::size_of::<SHA2_CTX>() as libc::c_ulong,
    );
    (*ctx).state.st64[0] = 0x6a09e667f3bcc908 as libc::c_ulonglong as uint64_t;
    (*ctx).state.st64[1] = 0xbb67ae8584caa73b as libc::c_ulonglong as uint64_t;
    (*ctx).state.st64[2] = 0x3c6ef372fe94f82b as libc::c_ulonglong as uint64_t;
    (*ctx).state.st64[3] = 0xa54ff53a5f1d36f1 as libc::c_ulonglong as uint64_t;
    (*ctx).state.st64[4] = 0x510e527fade682d1 as libc::c_ulonglong as uint64_t;
    (*ctx).state.st64[5] = 0x9b05688c2b3e6c1f as libc::c_ulonglong as uint64_t;
    (*ctx).state.st64[6] = 0x1f83d9abfb41bd6b as libc::c_ulonglong as uint64_t;
    (*ctx).state.st64[7] = 0x5be0cd19137e2179 as libc::c_ulonglong as uint64_t;
}

/* Round macros for SHA512 */
macro_rules! blk2 {
    ($x:expr) => {
        (W[($x & 15) as usize]).wrapping_add(
            (s1!(W[($x - 2 & 15) as usize]))
                .wrapping_add(W[($x - 7 & 15) as usize])
                .wrapping_add(s0!(W[($x - 15 & 15) as usize])),
        ) as uint64_t as uint64_t
    };
}
macro_rules! rotrFixed {
    ($x:expr,$y:expr) => {
        if $y != 0 {
            $x >> $y
                | $x << (std::mem::size_of::<uint64_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_ulong)
                    .wrapping_sub($y as libc::c_ulong)
        } else {
            $x
        }
    };
}
macro_rules! S0 {
    ($x:expr) => {
        (rotrFixed!($x, 28)) ^ (rotrFixed!($x, 34)) ^ (rotrFixed!($x, 39))
    };
}
macro_rules! S1 {
    ($x:expr) => {
        (rotrFixed!($x, 14)) ^ (rotrFixed!($x, 18)) ^ (rotrFixed!($x, 41))
    };
}
macro_rules! s0 {
    ($x:expr) => {
        (rotrFixed!($x, 1)) ^ (rotrFixed!($x, 8)) ^ $x >> 7
    };
}
macro_rules! s1 {
    ($x:expr) => {
        (rotrFixed!($x, 19)) ^ (rotrFixed!($x, 61)) ^ $x >> 6
    };
}
macro_rules! R {
    ($x:expr) => {
        h!($x) = (h!($x) as libc::c_ulong).wrapping_add(
            (S1!(e!($x)))
                .wrapping_add(Ch!(e!($x), f!($x), g!($x)))
                .wrapping_add(SHA512_K[($x as libc::c_uint).wrapping_add(j) as usize])
                .wrapping_add((if j != 0 { blk2!($x) } else { blk0!($x) })),
        ) as uint64_t as uint64_t;
        d!($x) = (d!($x) as libc::c_ulong).wrapping_add(h!($x)) as uint64_t as uint64_t;
        h!($x) = (h!($x) as libc::c_ulong).wrapping_add((S0!(a!($x))).wrapping_add(Maj!(
            a!($x),
            b!($x),
            c!($x)
        ))) as uint64_t as uint64_t;
    };
}

#[no_mangle]
pub unsafe extern "C" fn sudo_SHA512Transform(mut state: *mut uint64_t, mut data: *const uint8_t) {
    static mut W: [uint64_t; 16] = [0; 16];
    static mut T: [uint64_t; 8] = [0; 8];
    static mut j: libc::c_uint = 0;
    memcpy(
        T.as_mut_ptr() as *mut libc::c_void,
        state as *const libc::c_void,
        std::mem::size_of::<[uint64_t; 8]>() as libc::c_ulong,
    );
    while j < 16 {
        BE8TO64!(W[j as usize], data);
        data = data.offset(8 as isize);
        j = j.wrapping_add(1);
    }

pub unsafe extern "C" fn sudo_SHA512Update

pub unsafe extern "C" fn sudo_SHA512Pad

pub unsafe extern "C" fn sudo_SHA512Final
