/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */
#![allow(unreachable_code, unused_variables, unreachable_patterns)]
use crate::struct_macro::*;
use utsudo_util::sudo_debug::*;
use utsudo_util::sudo_debug_macro::*;
use utsudo_util::*;
use crate::ISSET;
use crate::SET;
use crate::S_IRGRP;
use crate::S_IROTH;
use crate::S_IRUSR;
use crate::S_ISDIR;
use crate::S_ISREG;
use crate::S_IWUSR;
use crate::WEXITSTATUS;
use crate::WIFSIGNALED;
use crate::W_EXITCODE;
//#define	W_OK	2		/* Test for write permission.  */
pub const W_OK: libc::c_int = 2;
//# define AT_EACCESS		0x200	/* Test access permitted for effective IDs, not real IDs.  */
pub const AT_EACCESS: libc::c_int = 0x200;
//#define	ENAMETOOLONG	36	/* File name too long */
pub const ENAMETOOLONG: libc::c_int = 36;
// #define ELOOP       40  /* Too many symbolic links encountered */
pub const ELOOP: libc::c_int = 40;
// #  define AT_FDCWD	-100
pub const AT_FDCWD: libc::c_int = -100;
// # define O_CREAT	   0100	/* Not fcntl.  */
pub const O_CREAT: libc::c_int = 0o100;
pub const SESH_SUCCESS: libc::c_int = 0;
pub const SESH_ERR_FAILURE: libc::c_int = 1;
pub const SESH_ERR_KILLED: libc::c_int = 2;
pub const SESH_ERR_BAD_PATHS: libc::c_int = 31;
pub const SESH_ERR_NO_FILES: libc::c_int = 32;
pub const O_PATH: libc::c_int = 0o10000000;
pub const O_DIRECTORY: libc::c_int = 0o200000;
macro_rules! DIR_OPEN_FLAGS {
    () => {
        (O_PATH | O_DIRECTORY)
    };
}

#[macro_export]
macro_rules! sudo_timespeccmp {
    ($ts1:expr, $ts2:expr, $op:tt) => {{
    (if (*$ts1).tv_sec == (*$ts2).tv_sec {
        ((*$ts1).tv_nsec $op (*$ts2).tv_nsec) as libc::c_int
    } else {
        ((*$ts1).tv_sec $op (*$ts2).tv_sec) as libc::c_int
    })
    }};
}
