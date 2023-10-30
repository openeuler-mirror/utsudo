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

unsafe extern "C" fn sudo_edit_open_nonwritable(
    mut path: *mut libc::c_char,
    mut oflags: libc::c_int,
    mut mode: mode_t,
    mut command_details: *mut command_details,
) -> libc::c_int {
    let dflags: libc::c_int = DIR_OPEN_FLAGS!() as libc::c_int;
    let mut dfd: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    let mut is_writable: libc::c_int = 0;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EDIT);
    if *path.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
        dfd = open(b"/\0" as *const u8 as *const libc::c_char, dflags);
        path = path.offset(1);
    } else {
        dfd = open(b".\0" as *const u8 as *const libc::c_char, dflags);
        if *path.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
            && *path.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
        {
            path = path.offset(2 as libc::c_int as isize);
        }
    }
    if dfd == -(1 as libc::c_int) {
        debug_return_int!(-1);
    }
    loop {
        let mut slash: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut subdfd: libc::c_int = 0;
        /*
         * Look up one component at a time, avoiding symbolic links in
         * writable directories.
         */
        is_writable = dir_is_writable(dfd, &mut user_details, command_details);
        if is_writable == -(1 as libc::c_int) {
            close(dfd);
            debug_return_int!(-(1 as libc::c_int));
        }
        while *path.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
            path = path.offset(1);
        }
        slash = strchr(path, '/' as i32);
        if slash.is_null() {
            break;
        }
        *slash = '\0' as i32 as libc::c_char;
        if is_writable != 0 {
            subdfd = sudo_edit_openat_nofollow(dfd, path, dflags, 0 as libc::c_int as mode_t);
        } else {
            subdfd = openat(dfd, path, dflags, 0 as libc::c_int);
        }
        *slash = '/' as i32 as libc::c_char; /* restore path */
        close(dfd);
        if subdfd == -(1 as libc::c_int) {
            debug_return_int!(-(1 as libc::c_int));
        }
        path = slash.offset(1 as libc::c_int as isize);
        dfd = subdfd;
    } // ! loop
    if is_writable != 0 {
        close(dfd);
        *__errno_location() = EISDIR;
        debug_return_int!(-(1 as libc::c_int));
    }
    /*
     * For "sudoedit /" we will receive ENOENT from openat() and sudoedit
     * will try to create a file with an empty name.  We treat an empty
     * path as the cwd so sudoedit can give a sensible error message.
     */
    fd = openat(
        dfd,
        if *path as libc::c_int != 0 {
            path
        } else {
            b".\0" as *const u8 as *const libc::c_char
        },
        oflags,
        mode,
    );
    close(dfd);
    debug_return_int!(fd);
}

unsafe extern "C" fn sudo_edit_open(
    mut path: *mut libc::c_char,
    mut oflags: libc::c_int,
    mut mode: mode_t,
    mut command_details: *mut command_details,
) -> libc::c_int {
    let sflags: libc::c_int = if !command_details.is_null() {
        (*command_details).flags
    } else {
        0 as libc::c_int
    };
    let mut fd: libc::c_int = 0;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EDIT);
    if ISSET!(sflags, CD_SUDOEDIT_FOLLOW) == 0 {
        oflags |= O_NOFOLLOW as libc::c_int;
    }
    if ISSET!(sflags, CD_SUDOEDIT_CHECKDIR) != 0 && user_details.uid != ROOT_UID as libc::c_uint {
        fd = sudo_edit_open_nonwritable(path, oflags | O_NONBLOCK, mode, command_details);
    } else {
        fd = open(path, oflags | O_NONBLOCK, mode);
    }
    if fd != -(1 as libc::c_int) && ISSET!(oflags, O_NONBLOCK) == 0 {
        fcntl(fd, F_SETFL, fcntl(fd, F_GETFL, 0) & !(O_NONBLOCK));
    }
    debug_return_int!(fd);
}

/*
 * Create temporary copies of files[] and store the temporary path name
 * along with the original name, size and mtime in tf.
 * Returns the number of files copied (which may be less than nfiles)
 * or -1 if a fatal error occurred.
 */
unsafe extern "C" fn sudo_edit_create_tfiles(
    mut command_details: *mut command_details,
    mut tf: *mut tempfile,
    mut files: *mut *mut libc::c_char,
    mut nfiles: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut tfd: libc::c_int = 0;
    let mut ofd: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut times: [timespec; 2] = [timespec {
        tv_sec: 0,
        tv_nsec: 0,
    }; 2];
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
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EDIT);
    /*
     * For each file specified by the user, make a temporary version
     * and copy the contents of the original to it.
     */
    i = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while i < nfiles {
        rc = -(1 as libc::c_int);
        switch_user(
            (*command_details).euid,
            (*command_details).egid,
            (*command_details).ngroups,
            (*command_details).groups,
        );
        ofd = sudo_edit_open(
            *files.offset(i as isize),
            O_RDONLY,
            (S_IRUSR!() | S_IWUSR!() | S_IRGRP!() | S_IROTH!()) as mode_t,
            command_details,
        );
        if ofd != -1 || *__errno_location() == ENOENT {
            if ofd == -(1 as libc::c_int) {
                /*
                 * New file, verify parent dir exists unless in cwd.
                 * This fails early so the user knows ahead of time if the
                 * edit won't succeed.  Additional checks are performed
                 * when copying the temporary file back to the origin.
                 */
                let mut slash: *mut libc::c_char = strrchr(*files.offset(i as isize), '/' as i32);
                if !slash.is_null() && slash != *files.offset(i as isize) {
                    let sflags: libc::c_int = (*command_details).flags;
                    let serrno: libc::c_int = *__errno_location();
                    let mut dfd: libc::c_int = 0;
                    /*
                     * The parent directory is allowed to be a symbolic
                     * link as long as *its* parent is not writable.
                     */
                    *slash = '\0' as i32 as libc::c_char;
                    SET!((*command_details).flags, CD_SUDOEDIT_FOLLOW);
                    dfd = sudo_edit_open(
                        *files.offset(i as isize),
                        DIR_OPEN_FLAGS!(),
                        (S_IRUSR!() | S_IWUSR!() | S_IRGRP!() | S_IROTH!()) as mode_t,
                        command_details,
                    );
                    (*command_details).flags = sflags;
                    if dfd != -(1 as libc::c_int) {
                        if fstat(dfd, &mut sb) == 0 && S_ISDIR!(sb.st_mode) {
                            memset(
                                &mut sb as *mut stat as *mut libc::c_void,
                                0 as libc::c_int,
                                ::std::mem::size_of::<stat>() as libc::c_ulong,
                            );
                            rc = 0 as libc::c_int;
                        }
                        close(dfd);
                    }
                    *slash = '/' as i32 as libc::c_char;
                    *__errno_location() = serrno;
                } else {
                    memset(
                        &mut sb as *mut stat as *mut libc::c_void,
                        0 as libc::c_int,
                        ::core::mem::size_of::<stat>() as libc::c_ulong,
                    );
                    rc = 0 as libc::c_int;
                }
            } else {
                rc = fstat(ofd, &mut sb);
            } // !  if ofd == -1
        } // ! if ofd != -1
        switch_user(
            0 as libc::c_int as uid_t,
            user_details.egid,
            user_details.ngroups,
            user_details.groups,
        );
        if ofd != -(1 as libc::c_int) && !(S_ISREG!(sb.st_mode)) {
            sudo_warnx!(
                b"%s: not a regular file\0" as *const u8 as *const libc::c_char,
                *files.offset(i as isize)
            );
            close(ofd);
            i += 1;
            continue;
        }
        if rc == -(1 as libc::c_int) {
            /* open() or fstat() error. */
            if ofd == -(1 as libc::c_int) && *__errno_location() == ELOOP {
                sudo_warnx!(
                    b"%s: editing symbolic links is not permitted\0" as *const u8
                        as *const libc::c_char,
                    *files.offset(i as isize)
                );
            } else if ofd == -1 && *__errno_location() == EISDIR {
                sudo_warnx!(
                    b"%s: editing files in a writable directory is not permitted\0" as *const u8
                        as *const libc::c_char,
                    *files.offset(i as isize),
                );
            } else {
                sudo_warn!(
                    b"%s\0" as *const u8 as *const libc::c_char,
                    *files.offset(i as isize),
                );
            }
            if ofd != -(1 as libc::c_int) {
                close(ofd);
            }
            i += 1;
            continue;
        }
        let ref mut fresh0 = (*tf.offset(j as isize)).ofile;
        *fresh0 = *files.offset(i as isize);
        (*tf.offset(j as isize)).osize = sb.st_size;
        (*tf.offset(j as isize)).omtim.tv_sec = sb.st_mtim.tv_sec;
        (*tf.offset(j as isize)).omtim.tv_nsec = sb.st_mtim.tv_nsec;
        sudo_debug_printf!(
            SUDO_DEBUG_INFO | SUDO_DEBUG_LINENO,
            b"seteuid(%u)\0" as *const u8 as *const libc::c_char,
            user_details.uid
        );
        if seteuid(user_details.uid) != 0 {
            sudo_fatal!(
                b"seteuid(%u)\0" as *const u8 as *const libc::c_char,
                user_details.uid
            );
        }
        tfd = sudo_edit_mktemp(
            (*tf.offset(j as isize)).ofile,
            &mut (*tf.offset(j as isize)).tfile,
        );
        sudo_debug_printf!(
            SUDO_DEBUG_INFO | SUDO_DEBUG_LINENO,
            b"seteuid(%u)\0" as *const u8 as *const libc::c_char,
            ROOT_UID
        );
        if seteuid(ROOT_UID as __uid_t) != 0 {
            sudo_fatal!(b"seteuid(ROOT_UID)\0" as *const u8 as *const libc::c_char,);
        }
        if tfd == -(1 as libc::c_int) {
            sudo_warn!(b"mkstemps\0" as *const u8 as *const libc::c_char,);
            if ofd != -(1 as libc::c_int) {
                close(ofd);
            }
            debug_return_int!(-(1 as libc::c_int));
        }
        if ofd != -(1 as libc::c_int) {
            if sudo_copy_file(
                (*tf.offset(j as isize)).ofile,
                ofd,
                (*tf.offset(j as isize)).osize,
                (*tf.offset(j as isize)).tfile,
                tfd,
                -(1 as libc::c_int) as off_t,
            ) == -(1 as libc::c_int)
            {
                close(ofd);
                close(tfd);
                debug_return_int!(-(1 as libc::c_int));
            }
            close(ofd);
        }
        /*
         * We always update the stashed mtime because the time
         * resolution of the filesystem the temporary file is on may
         * not match that of the filesystem where the file to be edited
         * resides.  It is OK if futimens() fails since we only use the
         * info to determine whether or not a file has been modified.
         */
        times[1 as libc::c_int as usize].tv_sec = (*tf.offset(j as isize)).omtim.tv_sec;
        times[0 as libc::c_int as usize].tv_sec = times[1 as libc::c_int as usize].tv_sec;
        times[1 as libc::c_int as usize].tv_nsec = (*tf.offset(j as isize)).omtim.tv_nsec;
        times[0 as libc::c_int as usize].tv_nsec = times[1 as libc::c_int as usize].tv_nsec;
        if futimens(tfd, times.as_mut_ptr() as *const timespec) == -(1 as libc::c_int) {
            if utimensat(
                AT_FDCWD,
                (*tf.offset(j as isize)).tfile,
                times.as_mut_ptr() as *const timespec,
                0,
            ) == -(1 as libc::c_int)
            {
                sudo_warn!(
                    b"%s\0" as *const u8 as *const libc::c_char,
                    (*tf.offset(j as isize)).tfile
                );
            }
        }
        rc = fstat(tfd, &mut sb);
        if rc == 0 {
            (*tf.offset(j as isize)).omtim.tv_sec = sb.st_mtim.tv_sec;
            (*tf.offset(j as isize)).omtim.tv_nsec = sb.st_mtim.tv_nsec;
        }
        close(tfd);
        j += 1;
        i += 1;
    } // ! while
    debug_return_int!(j);
}

/*
 * Copy the temporary files specified in tf to the originals.
 * Returns the number of copy errors or 0 if completely successful.
 */
unsafe extern "C" fn sudo_edit_copy_tfiles(
    mut command_details: *mut command_details,
    mut tf: *mut tempfile,
    mut nfiles: libc::c_int,
    mut times: *mut timespec,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut tfd: libc::c_int = 0;
    let mut ofd: libc::c_int = 0;
    let mut errors: libc::c_int = 0 as libc::c_int;
    let mut ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
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
    let mut oldmask: mode_t = 0;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_EDIT);
    /* Copy contents of temp files to real ones. */
    while i < nfiles {
        sudo_debug_printf!(
            SUDO_DEBUG_INFO | SUDO_DEBUG_LINENO,
            b"seteuid(%u)\0" as *const u8 as *const libc::c_char,
            user_details.uid
        );
        if seteuid(user_details.uid) != 0 as libc::c_int {
            sudo_fatal!(
                b"seteuid(%u)\0" as *const u8 as *const libc::c_char,
                user_details.uid,
            );
        }
        tfd = sudo_edit_open(
            (*tf.offset(i as isize)).tfile,
            O_RDONLY,
            S_IRUSR!() | S_IWUSR!() | S_IRGRP!() | S_IROTH!(),
            0 as *mut command_details,
        );
        if seteuid(ROOT_UID as __uid_t) != 0 as libc::c_int {
            sudo_fatal!(b"seteuid(ROOT_UID)\0" as *const u8 as *const libc::c_char,);
        }
        sudo_debug_printf!(
            SUDO_DEBUG_INFO | SUDO_DEBUG_LINENO,
            b"seteuid(%u)\0" as *const u8 as *const libc::c_char,
            ROOT_UID
        );
        if tfd == -(1 as libc::c_int)
            || !sudo_check_temp_file(
                tfd,
                (*tf.offset(i as isize)).tfile,
                user_details.uid,
                &mut sb,
            )
        {
            sudo_warnx!(
                b"%s left unmodified\0" as *const u8 as *const libc::c_char,
                (*tf.offset(i as isize)).ofile
            );
            if tfd != -(1 as libc::c_int) {
                close(tfd);
            }
            errors += 1;
            i += 1;
            continue;
        }
        ts.tv_sec = sb.st_mtim.tv_sec;
        ts.tv_nsec = sb.st_mtim.tv_nsec;
        if (*tf.offset(i as isize)).osize == sb.st_size
            && sudo_timespeccmp!(&((*(tf.offset(i as isize))).omtim), &ts, ==) != 0
        {
            /*
             * If mtime and size match but the user spent no measurable
             * time in the editor we can't tell if the file was changed.
             */
            if sudo_timespeccmp!(times.offset(0 as libc::c_int as isize), times.offset(1 as libc::c_int as isize), !=)
                != 0
            {
                sudo_warnx!(
                    b"%s unchanged\0" as *const u8 as *const libc::c_char,
                    (*tf.offset(i as isize)).ofile
                );
                unlink((*tf.offset(i as isize)).tfile);
                close(tfd);
                i += 1;
                continue;
            }
        }
        switch_user(
            (*command_details).euid,
            (*command_details).egid,
            (*command_details).ngroups,
            (*command_details).groups,
        );
        oldmask = umask((*command_details).umask);
        ofd = sudo_edit_open(
            (*tf.offset(i as isize)).ofile,
            O_WRONLY | O_CREAT,
            (S_IRUSR!() | S_IWUSR!() | S_IRGRP!() | S_IROTH!()) as mode_t,
            command_details,
        );
        umask(oldmask);
        switch_user(
            ROOT_UID as uid_t,
            user_details.egid,
            user_details.ngroups,
            user_details.groups,
        );
        if ofd == -(1 as libc::c_int) {
            sudo_warn!(
                b"unable to write to %s\0" as *const u8 as *const libc::c_char,
                (*tf.offset(i as isize)).ofile
            );
            // goto bad;
            sudo_warnx!(
                b"contents of edit session left in %s\0" as *const u8 as *const libc::c_char,
                (*tf.offset(i as isize)).tfile
            );
            errors += 1;
            if ofd != -(1 as libc::c_int) {
                close(ofd);
            }
            close(tfd);
            i += 1;
            continue;
        }
        /* Overwrite the old file with the new contents. */
        if sudo_copy_file(
            (*tf.offset(i as isize)).tfile,
            tfd,
            sb.st_size,
            (*tf.offset(i as isize)).ofile,
            ofd,
            (*tf.offset(i as isize)).osize,
        ) == -(1 as libc::c_int)
        {
            sudo_warnx!(
                b"contents of edit session left in %s\0" as *const u8 as *const libc::c_char,
                (*tf.offset(i as isize)).tfile
            );
            errors += 1;
        }
        if ofd != -(1 as libc::c_int) {
            close(ofd);
        }
        close(tfd);
        i += 1;
    } // ! while i < nfiles
    debug_return_int!(errors);
}


