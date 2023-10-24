/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */
pub type rlim_t = libc::c_ulong;
//struct rlimit
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlimit {
    pub rlim_cur: rlim_t, //rlim_t = ulong
    pub rlim_max: rlim_t,
}
//struct saved_limit
#[derive(Copy, Clone)]
#[repr(C)]
pub struct saved_limit {
    pub resource: libc::c_int,
    pub saved: bool,
    pub limit: rlimit,
}
//等同c宏
pub const RLIMIT_AS: libc::c_int = 9;
pub const RLIMIT_CPU: libc::c_int = 0;
pub const RLIMIT_DATA: libc::c_int = 2;
pub const RLIMIT_FSIZE: libc::c_int = 1;
pub const RLIMIT_NOFILE: libc::c_int = 7;
pub const RLIMIT_NPROC: libc::c_int = 6;
pub const RLIMIT_RSS: libc::c_int = 5;
pub const RLIMIT_STACK: libc::c_int = 3;
pub const RLIMIT_CORE: libc::c_int = 4;
pub const PR_GET_DUMPABLE: libc::c_int = 3;
pub const PR_SET_DUMPABLE: libc::c_int = 4;
pub const RLIM_INFINITY: libc::c_int = -1;
