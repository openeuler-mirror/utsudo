
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



