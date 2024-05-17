/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(
    dead_code,
    unused_imports,
    unused_attributes,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(register_tool)]

extern "C" {
    static mut sudoers_subsystem_ids: [libc::c_uint; 0];
}

use crate::common::*;
use stdext::function_name;
