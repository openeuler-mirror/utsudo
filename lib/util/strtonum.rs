


pub const _ISspace: libc::c_uint = 8192;

#[derive(Eq, PartialEq)]
enum strtonum_err {
    STN_INITIAL = 0,
    STN_VALID = 1,
    STN_INVALID = 2,
    STN_TOOSMALL = 3,
    STN_TOOBIG = 4,
}



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

    if minval > maxval {
        errval = strtonum_err::STN_INVALID;
    } else {
        loop {
            let fresh0 = cp;
            cp = cp.offset(1);
            ch = *fresh0 as libc::c_uchar;
            if !(*(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                & _ISspace as libc::c_ushort as libc::c_int
                != 0)
            {
                break;
            }
        }

        match ch as u8 as char {
            '-' => {
                sign = '-' as i32 as libc::c_char;
                let fresh1 = cp;
                cp = cp.offset(1);
                ch = *fresh1 as libc::c_uchar;
            }
            '+' => {
                let fresh2 = cp;
                cp = cp.offset(1);
                ch = *fresh2 as libc::c_uchar;
                sign = '+' as i32 as libc::c_char;
            }
            _ => {
                sign = '+' as i32 as libc::c_char;
            }
        }







    }


    return result;
}





