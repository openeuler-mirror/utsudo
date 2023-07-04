#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
)]

use crate::INT_MAX;
use crate::TIOCGWINSZ;

#[no_mangle]
pub unsafe extern "C" fn get_ttysize_ioctl(
    rowp: *mut libc::c_int,
    colp: *mut libc::c_int,
)  -> libc::c_int {
    let mut wsize: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };



debug_return!()
}

