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
