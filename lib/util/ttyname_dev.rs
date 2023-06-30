
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





