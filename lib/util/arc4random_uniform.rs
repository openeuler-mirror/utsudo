/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(unused_attributes, deprecated, dead_code, unused_assignments)]

use crate::arc4random::sudo_arc4random;
use libc::uint32_t;

#[no_mangle]
pub fn sudo_arc4random_uniform(upper_bound: uint32_t) -> uint32_t {
    let mut min: uint32_t = 0;
    let mut r: uint32_t = 0;
    if upper_bound < 2 {
        return 0;
    }
    min = upper_bound.wrapping_neg().wrapping_rem(upper_bound);

    loop {
        r = sudo_arc4random();
        if r >= min {
            break;
        }
    }
    return r.wrapping_rem(upper_bound);
}

