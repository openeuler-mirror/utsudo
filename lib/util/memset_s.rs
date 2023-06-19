/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

pub type size_t = libc::c_ulong;
pub const SIZE_MAX: libc::c_ulong = 18446744073709551615;
pub const RSIZE_MAX: libc::c_ulong = SIZE_MAX >> 1;

pub fn sudo_memset_s()

