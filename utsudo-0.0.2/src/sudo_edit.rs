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

extern "C" {
    fn umask(__mask: __mode_t) -> __mode_t;
    fn utimensat(
        __fd: libc::c_int,
        __path: *const libc::c_char,
        __times: *const timespec,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn futimens(__fd: libc::c_int, __times: *const timespec) -> libc::c_int;
    fn __fxstat(__ver: libc::c_int, __fildes: libc::c_int, __stat_buf: *mut stat) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn waitpid(__pid: __pid_t, __stat_loc: *mut libc::c_int, __options: libc::c_int) -> __pid_t;
    fn asprintf(__ptr: *mut *mut libc::c_char, __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn reallocarray(__ptr: *mut libc::c_void, __nmemb: size_t, __size: size_t)
        -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn mkstemps(__template: *mut libc::c_char, __suffixlen: libc::c_int) -> libc::c_int;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn faccessat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __type: libc::c_int,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn fchown(__fd: libc::c_int, __owner: __uid_t, __group: __gid_t) -> libc::c_int;
    fn execve(
        __path: *const libc::c_char,
        __argv: *const *mut libc::c_char,
        __envp: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn _exit(_: libc::c_int) -> !;
    fn setuid(__uid: __uid_t) -> libc::c_int;
    fn seteuid(__uid: __uid_t) -> libc::c_int;
    fn setegid(__gid: __gid_t) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn openat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __oflag: libc::c_int,
        _: ...
    ) -> libc::c_int;
    fn sudo_gettime_real_v1(ts: *mut timespec) -> libc::c_int;
    fn sudo_warn_gettext_v1(
        domainname: *const libc::c_char,
        msgid: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn sudo_conf_sesh_path_v1() -> *const libc::c_char;
    fn sudo_debug_exit_bool_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        subsys: libc::c_int,
        ret: bool,
    );
    fn sudo_debug_fork_v1() -> pid_t;
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn sudo_setgroups_v1(ngids: libc::c_int, gids: *const gid_t) -> libc::c_int;
    fn run_command(details: *mut command_details) -> libc::c_int;
    static mut user_details: user_details;
    fn sudo_strlcpy(dst: *mut libc::c_char, src: *const libc::c_char, siz: size_t) -> size_t;
    fn selinux_setcon() -> libc::c_int;
    fn selinux_setup(
        role: *const libc::c_char,
        type_0: *const libc::c_char,
        ttyn: *const libc::c_char,
        ttyfd: libc::c_int,
        label_tty: bool,
    ) -> libc::c_int;
    fn sudo_fatal_nodebug_v1(fmt: *const libc::c_char, _: ...) -> !;
    fn sudo_fatalx_nodebug_v1(fmt: *const libc::c_char, _: ...) -> !;
    fn sudo_warn_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_warnx_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_copy_file(
        src: *const libc::c_char,
        src_fd: libc::c_int,
        src_len: off_t,
        dst: *const libc::c_char,
        dst_fd: libc::c_int,
        dst_len: off_t,
    ) -> libc::c_int;
    fn sudo_check_temp_file(
        tfd: libc::c_int,
        tname: *const libc::c_char,
        uid: uid_t,
        sb: *mut stat,
    ) -> bool;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct tempfile {
    pub tfile: *mut libc::c_char,
    pub ofile: *mut libc::c_char,
    pub osize: off_t,
    pub omtim: timespec,
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

#[inline]
unsafe extern "C" fn fstat(mut __fd: libc::c_int, mut __statbuf: *mut stat) -> libc::c_int {
        #[cfg(target_arch = "x86_64")]
        return __fxstat(1 as libc::c_int, __fd, __statbuf);
        #[cfg(not(target_arch = "x86_64"))]
        return __fxstat(0 as libc::c_int, __fd, __statbuf);
}

static mut edit_tmpdir: [libc::c_char; 10] = [0; 10];

unsafe extern "C" fn switch_user(
    mut euid: uid_t,
    mut egid: gid_t,
    mut ngroups: libc::c_int,
    mut groups: *mut gid_t,
) {
    let mut serrno: libc::c_int = *__errno_location();
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EDIT);
    sudo_debug_printf!(
        SUDO_DEBUG_INFO | SUDO_DEBUG_LINENO,
        b"set uid:gid to %u:%u(%u)\0" as *const u8 as *const libc::c_char,
        euid,
        egid,
        if ngroups != 0 {
            *groups.offset(0 as libc::c_int as isize)
        } else {
            egid
        }
    );
    /* When restoring root, change euid first; otherwise change it last. */
    if euid == ROOT_UID as libc::c_uint {
        if seteuid(ROOT_UID as __uid_t) != 0 {
            sudo_fatal!(b"seteuid(ROOT_UID)\0" as *const u8 as *const libc::c_char,);
        }
    }
    if setegid(egid) != 0 {
        sudo_fatal!(b"setegid(%d)\0" as *const u8 as *const libc::c_char, egid);
    }
    if ngroups != -(1 as libc::c_int) {
        if sudo_setgroups_v1(ngroups, groups) != 0 {
            sudo_fatal!(b"setgroups\0" as *const u8 as *const libc::c_char,);
        }
    }
    if euid != ROOT_UID as libc::c_uint {
        if seteuid(euid) != 0 {
            sudo_fatal!(b"seteuid(%u)\0" as *const u8 as *const libc::c_char, euid);
        }
    }
    *__errno_location() = serrno;
    debug_return!();
}

/*
 * Returns true if the open directory fd is writable by the user.
 */
unsafe extern "C" fn dir_is_writable(
    mut dfd: libc::c_int,
    mut ud: *mut user_details,
    mut cd: *mut command_details,
) -> libc::c_int {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EDIT);
    let mut rc: libc::c_int = 0;
    /* Change uid/gid/groups to invoking user, usually needs root perms. */
    if (*cd).euid != ROOT_UID as libc::c_uint {
        if seteuid(ROOT_UID as __uid_t) != 0 {
            sudo_fatal!(b"seteuid(ROOT_UID)\0" as *const u8 as *const libc::c_char,);
        }
    }
    switch_user((*ud).uid, (*ud).gid, (*ud).ngroups, (*ud).groups);
    /* Access checks are done using the euid/egid and group vector. */
    rc = faccessat(
        dfd,
        b".\0" as *const u8 as *const libc::c_char,
        W_OK,
        AT_EACCESS,
    );
    /* Change uid/gid/groups back to target user, may need root perms. */
    if (*ud).uid != ROOT_UID as libc::c_uint {
        if seteuid(ROOT_UID as __uid_t) != 0 {
            sudo_fatal!(b"seteuid(ROOT_UID)\0" as *const u8 as *const libc::c_char,);
        }
    }
    switch_user((*cd).euid, (*cd).egid, (*cd).ngroups, (*cd).groups);
    if rc == 0 {
        debug_return_int!(true as libc::c_int);
    }
    if *__errno_location() == EACCES {
        debug_return_int!(0 as libc::c_int);
    }
    debug_return_int!(-1);
}

/*
 * Find our temporary directory, one of /var/tmp, /usr/tmp, or /tmp
 * Returns true on success, else false;
 */
unsafe extern "C" fn set_tmpdir(mut command_details: *mut command_details) -> bool {
    let mut tdir: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmpdirs: [*const libc::c_char; 3] = [
        b"/var/tmp/\0" as *const u8 as *const libc::c_char,
        b"/usr/tmp/\0" as *const u8 as *const libc::c_char,
        b"/tmp/\0" as *const u8 as *const libc::c_char,
    ];
    let mut i: libc::c_uint = 0;
    let mut len: size_t = 0;
    let mut dfd: libc::c_int = 0;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EDIT);
    i = 0 as libc::c_uint;
    while tdir.is_null()
        && (i as libc::c_ulong)
            < (::std::mem::size_of::<[*const libc::c_char; 3]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {
        dfd = open(tmpdirs[i as usize], 0 as libc::c_int);
        if dfd != -(1 as libc::c_int) {
            if dir_is_writable(dfd, &mut user_details, command_details) == 1 as libc::c_int {
                tdir = tmpdirs[i as usize];
            }
            close(dfd);
        }
        i = i.wrapping_add(1);
    }
    if tdir.is_null() {
        sudo_fatalx!(
            b"no writable temporary directory found\0" as *const u8 as *const libc::c_char,
        );
    }
    len = sudo_strlcpy(
        edit_tmpdir.as_mut_ptr(),
        tdir,
        ::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
    );
    if len >= ::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong {
        *__errno_location() = ENAMETOOLONG;
        sudo_warn!(b"%s\0" as *const u8 as *const libc::c_char, tdir);
        debug_return_bool!(false);
    }
    while len > 0 && {
        len = len.wrapping_sub(1);
        edit_tmpdir[len as usize] as libc::c_int == '/' as i32
    } {
        edit_tmpdir[len as usize] = '\0' as i32 as libc::c_char;
    }
    debug_return_bool!(true)
}

/*
 * Construct a temporary file name for file and return an
 * open file descriptor.  The temporary file name is stored
 * in tfile which the caller is responsible for freeing.
 */
unsafe extern "C" fn sudo_edit_mktemp(
    mut ofile: *const libc::c_char,
    mut tfile: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    let mut suff: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: libc::c_int = 0;
    let mut tfd: libc::c_int = 0;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EDIT);
    cp = strrchr(ofile, '/' as i32);
    if !cp.is_null() {
        cp = cp.offset(1);
    } else {
        cp = ofile;
    }
    suff = strrchr(cp, '.' as i32);
    if !suff.is_null() {
        len = asprintf(
            tfile,
            b"%s/%.*sXXXXXXXX%s\0" as *const u8 as *const libc::c_char,
            edit_tmpdir.as_mut_ptr(),
            suff.offset_from(cp) as libc::c_long as size_t as libc::c_int,
            cp,
            suff,
        );
    } else {
        len = asprintf(
            tfile,
            b"%s/%s.XXXXXXXX\0" as *const u8 as *const libc::c_char,
            edit_tmpdir.as_mut_ptr(),
            cp,
        );
    }
    if len == -(1 as libc::c_int) {
        sudo_warnx!(
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr(),
            b"unable to allocate memory\0" as *const u8 as *const libc::c_char
        );
        debug_return_int!(-(1 as libc::c_int));
    }
    tfd = mkstemps(
        *tfile,
        (if !suff.is_null() {
            strlen(suff)
        } else {
            0 as libc::c_int as libc::c_ulong
        }) as libc::c_int,
    );
    sudo_debug_printf!(
        SUDO_DEBUG_INFO | SUDO_DEBUG_LINENO,
        b"%s -> %s, fd %d\0" as *const u8 as *const libc::c_char,
        ofile,
        *tfile,
        tfd
    );
    debug_return_int!(tfd);
}

unsafe extern "C" fn sudo_edit_openat_nofollow(
    mut dfd: libc::c_int,
    mut path: *mut libc::c_char,
    mut oflags: libc::c_int,
    mut mode: mode_t,
) -> libc::c_int {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EDIT);
    debug_return_int!(openat(dfd, path, oflags | O_NOFOLLOW, mode));
}


