#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
)]

use crate::INT_MAX;
use crate::TIOCGWINSZ;

pub unsafe extern "C" fn get_ttysize_ioctl(
    rowp: *mut libc::c_int,
    colp: *mut libc::c_int,
) {




}

