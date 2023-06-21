/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(unused_variables, unused_mut, unused_assignments)]

use crate::sudo_debug::sudo_debug_enter_v1;
use crate::sudo_debug::sudo_debug_exit_ptr_v1;
use crate::sudo_debug_macro::SUDO_DEBUG_UTIL;

unsafe fn sudo_strsplit_v1(
    mut str: *const libc::c_char,
    mut last: *mut *const libc::c_char,
) -> *const libc::c_char {
}
