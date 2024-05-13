
/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(
    unused_variables,
    unused_mut,
    dead_code,
    unused_assignments,
    non_camel_case_types,
    non_upper_case_globals
)]
use libc::dlclose;
use libc::dlopen;
use libc::exit;
use libc::fprintf;
use libc::getopt;
use libc::getpwnam;
use libc::perror;
use libc::printf;
use libc::strchr;
use libc::strpbrk;
use libc::strtok_r;
use libc::FILE;

#[macro_use]
mod common;
use crate::common::*;
#[link(name = "utsudo_util")]
#[link(name = "util_variadic")]
#[link(name = "group_file_variadic")]
