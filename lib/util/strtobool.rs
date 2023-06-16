#![allow(
    mutable_transmutes,
    non_camel_case_types,
    unused_assignments,
    unused_mut
)]

use crate::sudo_debug::*;
use crate::sudo_debug_macro::*;

extern "C" {
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        lineno: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
}

#[no_mangle]
pub unsafe extern "C" fn sudo_strtobool_v1(mut str: *const libc::c_char) -> libc::c_int {
    
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);
    match *str as u8 as char {
        '0' | '1' => {
            if *str.offset(1 as isize) as libc::c_int == '\u{0}' as i32 {
                debug_return_int!(*str as libc::c_int - '0' as i32);
            }
        }
        'y' | 'Y' => {
            if strcasecmp(str, b"yes\0" as *const u8 as *const libc::c_char) == 0 {
                debug_return_int!(1);
            }
        }
        't' | 'T' => {
            if strcasecmp(str, b"true\0" as *const u8 as *const libc::c_char) == 0 {
                debug_return_int!(1);
            }
        }
        'o' | 'O' => {
            if strcasecmp(str, b"on\0" as *const u8 as *const libc::c_char) == 0 {
                debug_return_int!(1);
            }
            if strcasecmp(str, b"off\0" as *const u8 as *const libc::c_char) == 0 {
                debug_return_int!(0);
            }
        }
        'n' | 'N' => {
            if strcasecmp(str, b"no\0" as *const u8 as *const libc::c_char) == 0 {
                debug_return_int!(0);
            }
        }
        'f' | 'F' => {
            if strcasecmp(str, b"false\0" as *const u8 as *const libc::c_char) == 0 {
                debug_return_int!(0);
            }
        }
        _ => {}
    }

}


