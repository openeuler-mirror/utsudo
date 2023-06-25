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
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;

use crate::sudo_debug::sudo_debug_enter_v1;
use crate::sudo_debug::sudo_debug_exit_int_v1;
use crate::sudo_debug_macro::SUDO_DEBUG_UTIL;


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

// #define SUDO_PATH_MISSING		-1
#[macro_export]
macro_rules! SUDO_PATH_MISSING {
    () => {
        -1
    };
}

// #define	__S_IFMT	0170000	/* These bits determine file type.  */
#[macro_export]
macro_rules! _S_IFMT {
    () => {
        0o170000
    };
}

// #define SUDO_PATH_BAD_TYPE		-2
#[macro_export]
macro_rules! SUDO_PATH_BAD_TYPE {
    () => {
        -2
    };
}

// #define SUDO_PATH_WRONG_OWNER		-3
#[macro_export]
macro_rules! SUDO_PATH_WRONG_OWNER {
    () => {
        -3
    };
}

// #define SUDO_PATH_WORLD_WRITABLE	-4
#[macro_export]
macro_rules! SUDO_PATH_WORLD_WRITABLE {
    () => {
        -4
    };
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,     /* Device.  */
    pub st_ino: __ino_t,     /* File serial number.	*/
    pub st_nlink: __nlink_t, /* Link count.  */
    pub st_mode: __mode_t,   /* File mode.  */
    pub st_uid: __uid_t,     /* User ID of the file's owner.	*/
    pub st_gid: __gid_t,     /* Group ID of the file's group.*/
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,        /* Device number, if device.  */
    pub st_size: __off_t,        /* Size of file, in bytes.  */
    pub st_blksize: __blksize_t, /* Optimal block size for I/O.  */
    pub st_blocks: __blkcnt_t,   /* Number 512-byte blocks allocated. */
    /* Nanosecond resolution timestamps are stored in a format
    equivalent to 'struct timespec'.  This is the type used
    whenever possible but the Unix namespace rules do not allow the
    identifier 'timespec' to appear in the <sys/stat.h> header.
    Therefore we have to handle the use of this header in strictly
    standard-compliant sources special.  */
    pub st_atim: timespec, /* Time of last access.  */
    pub st_mtim: timespec, /* Time of last modification.  */
    pub st_ctim: timespec, /* Time of last status change.  */
    pub __glibc_reserved: [__syscall_slong_t; 3],
}



#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}

/*
 * Verify that path is the right type and not writable by other users.
 */
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
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
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
        __glibc_reserved: [0; 3],
    };
    let mut ret: libc::c_int = SUDO_PATH_MISSING!();
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    if (!path.is_null()) && (stat(path, &mut sb) == 0) {
        if (sb.st_mode & _S_IFMT!()) != c_type {
            ret = SUDO_PATH_BAD_TYPE!();
        } else if uid != -(1 as libc::c_int) as uid_t && sb.st_uid != uid {
            ret = SUDO_PATH_WRONG_OWNER!();
        } else if (sb.st_mode & S_IWOTH!()) != 0 {
            ret = SUDO_PATH_WORLD_WRITABLE!();
        } else if ISSET!(sb.st_mode, S_IWGRP!()) != 0
            && (gid == -(1 as libc::c_int) as gid_t || sb.st_gid != gid)
        {
            ret = SUDO_PATH_GROUP_WRITABLE!();
        } else {
            ret = SUDO_PATH_SECURE!();
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