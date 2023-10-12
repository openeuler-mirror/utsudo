/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */
#![allow(
    unused_must_use,
    unused_variables,
    clashing_extern_declarations,
    unreachable_patterns
)]
use crate::struct_macro::*;
use utsudo_util::sudo_debug::*;
use utsudo_util::sudo_debug_macro::*;
use utsudo_util::*;
use crate::errno;
use crate::ISSET;
use crate::SIG_IGN;
use crate::USER_SIGNALED;
use crate::WEXITSTATUS;
use crate::WIFEXITED;
use crate::WIFSIGNALED;
use crate::WIFSTOPPED;
use crate::WSTOPSIG;
use crate::WTERMSIG;

pub const WCONTINUED: libc::c_int = 8;






