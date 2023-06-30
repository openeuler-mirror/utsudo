
#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    clashing_extern_declarations,
    unused_variables  
)]

macro_rules! _PATH_DEV {
    () => {
        b"/dev/\0" as *const u8 as *const libc::c_char
    };
}

// console
macro_rules! _PATH_DEV_CONSOLE {
    () => {
        b"/dev/console\0" as *const u8 as *const libc::c_char
    };
}

// _PATH_DEV "pts"
macro_rules! _PATH_DEV_PTS {
    () => {
        b"/dev/pts\0" as *const u8 as *const libc::c_char
    };
}

pub const PATH_MAX: usize = 4096;
// #define	ENOMEM		12	/* Out of memory */
pub const ENOMEM: libc::c_int = 12;
pub const DT_CHR: libc::c_int = 2;
pub const DT_LNK: libc::c_int = 10;
pub const DT_UNKNOWN: libc::c_int = 0;

//     _PATH_DEV "stdin",
//     _PATH_DEV "stdout",
//     _PATH_DEV "stderr",
macro_rules! _PATH_DEV_STDIN {
    () => {
        b"/dev/stdin\0" as *const u8 as *const libc::c_char
    };
}

macro_rules! _PATH_DEV_STDOUT {
    () => {
        b"/dev/stdout\0" as *const u8 as *const libc::c_char
    };
}

macro_rules! _PATH_DEV_STDERR {
    () => {
        b"/dev/stderr\0" as *const u8 as *const libc::c_char
    };
}

macro_rules! __S_IFCHR {
    () => {
        0o020000
    };
}

macro_rules! __S_ISTYPE {
    ($mode:expr, $mask:expr) => {
        ((($mode) & _S_IFMT!()) == ($mask))
    };
}

macro_rules! S_ISCHR {
    ($mode:expr) => {
        __S_ISTYPE!(($mode), __S_IFCHR!())
    };
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __dirstream {
    _unused: [u8; 0],
}

#[inline]
unsafe extern "C" fn fstat(mut __fd: libc::c_int, mut __statbuf: *mut stat) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}

#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}

#[inline]
unsafe extern "C" fn gnu_dev_minor(mut __dev: __dev_t) -> libc::c_uint {
    let mut __minor: libc::c_uint = 0;
    __minor = ((__dev & 0xff as libc::c_uint as __dev_t) >> 0 as libc::c_int) as libc::c_uint;
    __minor = (__minor as libc::c_ulong
        | (__dev & 0xffffff00000 as libc::c_ulong) >> 12 as libc::c_int)
        as libc::c_uint;
    return __minor;
}

/*
 * Device nodes to ignore.
 */
static mut ignore_devs: [*const libc::c_char; 4] = [
    _PATH_DEV_STDIN!(),
    _PATH_DEV_STDOUT!(),
    _PATH_DEV_STDERR!(),
    0 as *const libc::c_char,
];

/*
 * Like ttyname() but uses a dev_t instead of an open fd.
 * Returns name on success and NULL on failure, setting errno.
 * Generic version.
 */
#[no_mangle]
pub unsafe extern "C" fn sudo_ttyname_dev_v1(
    mut rdev: dev_t,
    mut buf: *mut libc::c_char,
    mut buflen: size_t,
) -> *mut libc::c_char {
    let mut devsearch: *const libc::c_char = 0 as *const libc::c_char;
    let mut devsearch_end: *const libc::c_char = 0 as *const libc::c_char;
    let mut path: [libc::c_char; PATH_MAX] = [0; PATH_MAX];
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    let mut ep: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = 0 as size_t;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);
    /*
     * First, check /dev/console.
     */
    ret = sudo_dev_check(rdev, _PATH_DEV_CONSOLE!(), buf, buflen);
    if ret.is_null() {
        /*
         * Then check the device search path.
         */
        devsearch = sudo_conf_devsearch_path_v1();
        devsearch_end = devsearch.offset(strlen(devsearch) as isize);
        cp = sudo_strsplit_v1(
            devsearch,
            devsearch_end,
            b":\0" as *const u8 as *const libc::c_char,
            &mut ep,
        );
        while !cp.is_null() {
            len = ep.offset_from(cp) as size_t;
            if len >= ::std::mem::size_of::<[libc::c_char; PATH_MAX]>() as libc::c_ulong {
                sudo_debug_printf!(
                    SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
                    b"devsearch entry %.*s too long\0" as *const u8 as *const libc::c_char,
                    len as libc::c_int,
                    cp
                );
                cp = sudo_strsplit_v1(
                    0 as *const libc::c_char,
                    devsearch_end,
                    b":\0" as *const u8 as *const libc::c_char,
                    &mut ep,
                );
                continue;
            } // if len >=
            memcpy(
                path.as_mut_ptr() as *mut libc::c_void,
                cp as *const libc::c_void,
                len,
            );
            path[len as usize] = '\u{0}' as i32 as libc::c_char;
            if strcmp(path.as_mut_ptr(), _PATH_DEV_PTS!()) == 0 {
                /* Special case /dev/pts */
                len = snprintf(
                    path.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; PATH_MAX]>() as libc::c_ulong,
                    b"%spts/%u\0" as *const u8 as *const libc::c_char,
                    _PATH_DEV!(),
                    gnu_dev_minor(rdev),
                ) as size_t;
                if len > ::std::mem::size_of::<[libc::c_char; PATH_MAX]>() as libc::c_ulong {
                    sudo_debug_printf!(
                        SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
                        b"devsearch entry %spts/%u too long\0" as *const u8 as *const libc::c_char,
                        _PATH_DEV!(),
                        gnu_dev_minor(rdev) as libc::c_uint
                    );
                    cp = sudo_strsplit_v1(
                        0 as *const libc::c_char,
                        devsearch_end,
                        b":\0" as *const u8 as *const libc::c_char,
                        &mut ep,
                    );
                    continue;
                } //if  len >
                ret = sudo_dev_check(rdev, path.as_mut_ptr(), buf, buflen);
                if !ret.is_null() {
                    break;
                }
            }
            // if strcmp(path
            else {
                /* Scan path, looking for rdev. */
                ret = sudo_ttyname_scan(path.as_mut_ptr(), rdev, buf, buflen);
                if !ret.is_null() || *__errno_location() == ENOMEM {
                    break;
                }
            }
            cp = sudo_strsplit_v1(
                0 as *const libc::c_char,
                devsearch_end,
                b":\0" as *const u8 as *const libc::c_char,
                &mut ep,
            );
        } // while !cp.is_null()
    } // ret.is_null()
    debug_return_str!(ret as *mut libc::c_char)
}

use crate::macro_struct::*;
use crate::S_IWGRP;
use crate::S_IWOTH;
use crate::S_IWUSR;
use crate::_S_IFMT;
use crate::sudo_debug::*;
use crate::sudo_debug_macro::*;

pub type DIR = __dirstream;

extern "C" {
        fn sudo_conf_devsearch_path_v1() -> *const libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn sudo_strsplit_v1(
        str: *const libc::c_char,
        endstr: *const libc::c_char,
        sep: *const libc::c_char,
        last: *mut *const libc::c_char,
    ) -> *const libc::c_char;





}

#[no_mangle]
unsafe extern "C" fn sudo_ttyname_scan(
    mut dir: *const libc::c_char,
    mut rdev: dev_t,
    mut name: *mut libc::c_char,
    mut namelen: size_t,
) -> *mut libc::c_char {
    let mut sdlen: size_t = 0 as size_t;
    let mut pathbuf: [libc::c_char; PATH_MAX] = [0; PATH_MAX];
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dp: *mut dirent = 0 as *mut dirent;

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

    let mut i: libc::c_int = 0 as libc::c_int;
    let mut d: *mut DIR = 0 as *mut DIR;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    'done: loop {
        if *dir.offset(0 as isize) as libc::c_int == '\u{0}' as i32 {
            *__errno_location() = ENOENT;
            break 'done;
        }
        d = opendir(dir);
        if d.is_null() {
            break 'done;
        }
        if fstat(dirfd(d), &mut sb) == -1 {
            sudo_debug_printf!(
                SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
                b"unable to fstat %s\0" as *const u8 as *const libc::c_char,
                dir
            );
            break 'done;
        }
        if (sb.st_mode & S_IWOTH!()) != 0 {
            sudo_debug_printf!(
                SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
                b"ignoring world-writable directory %s\0" as *const u8 as *const libc::c_char,
                dir
            );
            *__errno_location() = ENOENT;
            break 'done;
        }
        sudo_debug_printf!(
            SUDO_DEBUG_INFO | SUDO_DEBUG_LINENO,
            b"scanning for dev %u in %s\0" as *const u8 as *const libc::c_char,
            rdev as libc::c_uint,
            dir
        );
        sdlen = strlen(dir);
        while sdlen > 0 && dir.offset((sdlen - 1) as isize) as libc::c_int == '/' as i32 {
            sdlen -= 1;
        }
        if (sdlen + 1) >= ::std::mem::size_of::<[libc::c_char; PATH_MAX]>() as libc::c_ulong {
            *__errno_location() = ERANGE;
            break 'done;
        }
        memcpy(
            pathbuf.as_mut_ptr() as *mut libc::c_void,
            dir as *const libc::c_void,
            sdlen,
        );
        sdlen += 1;
        pathbuf[sdlen as usize] = '/' as i32 as libc::c_char;
        dp = readdir(d);
        while !dp.is_null() {
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
            /* Skip anything starting with "." */
            if (*dp).d_name[0 as usize] as libc::c_int == '.' as i32 {
                continue;
            }
            pathbuf[sdlen as usize] = '\u{0}' as i32 as libc::c_char;
            if sudo_strlcat(
                pathbuf.as_mut_ptr(),
                ((*dp).d_name).as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; PATH_MAX]>() as libc::c_ulong,
            ) >= ::std::mem::size_of::<[libc::c_char; PATH_MAX]>() as libc::c_ulong
            {
                sudo_debug_printf!(
                    SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
                    b"%s%s is too big to fit in pathbuf\0" as *const u8 as *const libc::c_char,
                    pathbuf,
                    (*dp).d_name
                );
                continue;
            }
            /* Ignore device nodes listed in ignore_devs[]. */
            i = 0;
            while !ignore_devs[i as usize].is_null() {
                if strcmp(pathbuf.as_mut_ptr(), ignore_devs[i as usize]) == 0 {
                    break;
                }
                i += 1;
            }
            if !ignore_devs[i as usize].is_null() {
                sudo_debug_printf!(
                    SUDO_DEBUG_DEBUG | SUDO_DEBUG_LINENO,
                    b"ignoring %s\0" as *const u8 as *const libc::c_char,
                    pathbuf
                );
                continue;
            }
            /*
             * Avoid excessive stat() calls by checking dp->d_type.
             */
            match (*dp).d_type as libc::c_int {
                DT_CHR | DT_LNK | DT_UNKNOWN => { /* 为空*/ }
                _ => {
                    /* Not a character device or link, skip it. */
                    sudo_debug_printf!(
                        SUDO_DEBUG_DEBUG | SUDO_DEBUG_LINENO,
                        b"skipping non-device %s\0" as *const u8 as *const libc::c_char,
                        pathbuf
                    );
                    continue;
                }
            }
            if stat(pathbuf.as_mut_ptr(), &mut sb) == -1 {
                sudo_debug_printf!(
                    SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                    b"unable to stat %s\0" as *const u8 as *const libc::c_char,
                    pathbuf
                );
                continue;
            }
            if S_ISCHR!(sb.st_mode) && sb.st_rdev == rdev {
                sudo_debug_printf!(
                    SUDO_DEBUG_INFO | SUDO_DEBUG_LINENO,
                    b"resolved dev %u as %s\0" as *const u8 as *const libc::c_char,
                    rdev as libc::c_uint,
                    pathbuf
                );
                if sudo_strlcpy(name, pathbuf.as_mut_ptr(), namelen) < namelen {
                    ret = name;
                } else {
                    sudo_debug_printf!(
                        SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
                        b"unable to store %s, have %zu, need %zu\0" as *const u8
                            as *const libc::c_char,
                        pathbuf,
                        namelen,
                        strlen(pathbuf.as_ptr() as *const libc::c_char) + 1
                    );
                    *__errno_location() = ERANGE;
                }
                break 'done;
            }
        } //while ((dp
        break 'done;
    } // 'done loop













    debug_return_str!(ret)
}














