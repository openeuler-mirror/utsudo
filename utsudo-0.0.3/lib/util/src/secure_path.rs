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
    unused_mut,
    unused_variables
)]

use crate::macro_struct::*;
use crate::sudo_debug::*;
use crate::sudo_debug_macro::*;

use crate::ISSET;
use crate::S_IWGRP;
use crate::S_IWOTH;
use crate::S_IWUSR;
use crate::_S_IFMT;

// #define	__S_IFREG	0100000	/* Regular file.  */
#[macro_export]
macro_rules! _S_IFREG {
    () => {
        0o100000
    };
}

// #define	__S_IFDIR	0040000	/* Directory.  */
#[macro_export]
macro_rules! _S_IFDIR {
    () => {
        0o40000
    };
}

extern "C" {
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}

#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    #[cfg(target_arch = "x86_64")]
    return __xstat(1 as libc::c_int, __path, __statbuf);
    #[cfg(not(target_arch = "x86_64"))]
    return __xstat(0 as libc::c_int, __path, __statbuf);
}

#[no_mangle]
pub unsafe extern "C" fn sudo_secure_path(
    mut path: *const libc::c_char,
    mut c_type: libc::c_uint,
    mut uid: uid_t,
    mut gid: gid_t,
    mut sbp: *mut stat,
) -> libc::c_int {
    let mut sb: stat = stat {
        st_dev: 0,
        st_ino: 0,
        #[cfg(target_arch = "x86_64")]
        st_nlink: 0,
        st_mode: 0,
        #[cfg(not(target_arch = "x86_64"))]
        st_nlink: 0,
        st_uid: 0,
        st_gid: 0,
        #[cfg(target_arch = "x86_64")]
        __pad0: 0,
        st_rdev: 0,
        #[cfg(not(target_arch = "x86_64"))]
        __pad1: 0,
        st_size: 0,
        st_blksize: 0,
        #[cfg(not(target_arch = "x86_64"))]
        __pad2: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        #[cfg(target_arch = "x86_64")]
        __glibc_reserved: [0; 3],
        #[cfg(not(target_arch = "x86_64"))]
        __glibc_reserved: [0; 2],
        };

    let mut ret: libc::c_int = SUDO_PATH_MISSING;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    if (!path.is_null()) && (stat(path, &mut sb) == 0) {
        if (sb.st_mode & _S_IFMT!()) != c_type {
            ret = SUDO_PATH_BAD_TYPE;
        } else if uid != -(1 as libc::c_int) as uid_t && sb.st_uid != uid {
            ret = SUDO_PATH_WRONG_OWNER;
        } else if (sb.st_mode & S_IWOTH!()) != 0 {
            ret = SUDO_PATH_WORLD_WRITABLE;
        } else if ISSET!(sb.st_mode, S_IWGRP!()) != 0
            && (gid == -(1 as libc::c_int) as gid_t || sb.st_gid != gid)
        {
            ret = SUDO_PATH_GROUP_WRITABLE;
        } else {
            ret = SUDO_PATH_SECURE;
        }
        if !sbp.is_null() {
            memcpy(
                sbp as *mut libc::c_void,
                &mut sb as *mut stat as *const libc::c_void,
                ::std::mem::size_of::<stat>() as libc::c_ulong,
            );
        }
    }

    debug_return_int!(ret)
}

/*
 * Verify that path is a regular file and not writable by other users.
 */
#[no_mangle]
pub unsafe extern "C" fn sudo_secure_file_v1(
    mut path: *const libc::c_char,
    mut uid: uid_t,
    mut gid: gid_t,
    mut sbp: *mut stat,
) -> libc::c_int {
    return sudo_secure_path(path, _S_IFREG!(), uid, gid, sbp);
}

/*
 * Verify that path is a directory and not writable by other users.
 */
#[no_mangle]
pub unsafe extern "C" fn sudo_secure_dir_v1(
    mut path: *const libc::c_char,
    mut uid: uid_t,
    mut gid: gid_t,
    mut sbp: *mut stat,
) -> libc::c_int {
    return sudo_secure_path(path, _S_IFDIR!(), uid, gid, sbp);
}
