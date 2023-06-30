
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












