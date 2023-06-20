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

#[macro_export]
macro_rules! SUDO_DSO_LAZY {
    () => {
        0x1
    };
}

#[macro_export]
macro_rules! SUDO_DSO_NOW {
    () => {
        0x2
    };
}

#[macro_export]
macro_rules! SUDO_DSO_GLOBAL {
    () => {
        0x3
    };
}

/* The MODE argument to `dlopen' contains one of the following: */
// #define RTLD_LAZY    0x00001 /* Lazy function call binding.  */
// #define RTLD_NOW     0x00002 /* Immediate function call binding.  */
#[macro_export]
macro_rules! RTLD_LAZY {
    () => {
        0x00001
    };
}


