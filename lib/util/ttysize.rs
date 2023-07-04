#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use crate::INT_MAX;
use crate::TIOCGWINSZ;

use crate::macro_struct::*;
use crate::sudo_debug::*;
use crate::sudo_debug_macro::*;

/* Standard file descriptors.  */
// #define	STDERR_FILENO	2	/* Standard error output.  */
#[macro_export]
macro_rules! STDERR_FILENO {
    () => {
        2
    };
}

extern "C" {
    fn ioctl(fd: libc::c_int, __request: libc::c_ulong, ...) -> libc::c_int;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn sudo_strtonum(
        _: *const libc::c_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *const libc::c_char,
    ) -> libc::c_longlong;
}

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

    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    if (ioctl(STDERR_FILENO!(), TIOCGWINSZ!(), &wsize) == 0)
        && wsize.ws_row != 0
        && wsize.ws_col != 0
    {
        *rowp = wsize.ws_row as libc::c_int;
        *colp = wsize.ws_col as libc::c_int;
        debug_return_int!(0);
    }
    debug_return_int!(-1)
}


#[no_mangle]
pub unsafe extern "C" fn sudo_get_ttysize_v1(
    mut rowp: *mut libc::c_int,
    mut colp: *mut libc::c_int,
) {
    if get_ttysize_ioctl(rowp, colp) == -(1 as libc::c_int) {
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        /* Fall back on $LINES and $COLUMNS. */
        p = getenv(b"LINES\0" as *const u8 as *const libc::c_char);
        if p.is_null() || {
            *rowp = sudo_strtonum(
                p,
                1,
                INT_MAX!() as libc::c_int as libc::c_longlong,
                0 as *mut *const libc::c_char,
            ) as libc::c_int;
            *rowp <= 0 as libc::c_int
        } {
            *rowp = 24;
        }
        p = getenv(b"COLUMNS\0" as *const u8 as *const libc::c_char);
        if p.is_null() || {
            *colp = sudo_strtonum(
                p,
                1,
                INT_MAX!() as libc::c_int as libc::c_longlong,
                0 as *mut *const libc::c_char,
            ) as libc::c_int;
            *colp <= 0 as libc::c_int
        } {
            *colp = 80;
        }
    }
    debug_return!()
}




