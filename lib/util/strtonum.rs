


pub const _ISspace: libc::c_uint = 8192;

#[no_mangle]
pub unsafe extern "C" fn sudo_strtonumx(
mut str: *const libc::c_char,
mut minval: libc::c_longlong,
mut maxval: libc::c_longlong,
mut endp: *mut *mut libc::c_char,
mut errstrp: *mut *const libc::c_char,        
) -> libc::c_longlong {
    let mut errval = strtonum_err::STN_INITIAL;
    let mut lastval: libc::c_longlong = 0;
    let mut result: libc::c_longlong = 0 as libc::c_longlong;
    let mut cp: *const libc::c_char = str;
    let mut ch: libc::c_uchar = 0;
    let mut remainder: libc::c_int = 0;
    let mut sign: libc::c_char = 0;













    return result;
}





