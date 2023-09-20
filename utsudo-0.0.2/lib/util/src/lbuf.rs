/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(
    non_camel_case_types,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    dead_code
)]

use crate::sudo_debug::sudo_debug_enter_v1;
use crate::sudo_debug::sudo_debug_exit_bool_v1;
use crate::sudo_debug::sudo_debug_exit_v1;
use crate::sudo_debug_macro::SUDO_DEBUG_ERROR;
use crate::sudo_debug_macro::SUDO_DEBUG_LINENO;
use crate::sudo_debug_macro::SUDO_DEBUG_UTIL;

extern "C" {
    fn free(__ptr: *mut libc::c_void);
    fn realloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn strlen(__s: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn memchr(_: *const libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn memrchr(__s: *const libc::c_void, __c: libc::c_int, __n: size_t) -> *mut libc::c_void;
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        lineno: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
}

pub const _ISblank: libc::c_uint = 1;

pub struct sudo_lbuf {
    pub output: Option<unsafe extern "C" fn(*const libc::c_char) -> libc::c_int>, //int (*output)(const char *);
    pub buf: *mut libc::c_char,
    pub continuation: *const libc::c_char,
    pub indent: libc::c_int,
    pub len: libc::c_int,
    pub size: libc::c_int,
    pub cols: libc::c_short,
    pub error: libc::c_short,
}

pub type sudo_lbuf_output_t = Option<unsafe extern "C" fn(*const libc::c_char) -> libc::c_int>;
pub type size_t = libc::c_ulong;

#[no_mangle]
pub unsafe extern "C" fn sudo_lbuf_init_v1(
    mut lbuf: *mut sudo_lbuf,
    mut output: sudo_lbuf_output_t,
    mut indent: libc::c_int,
    mut continuation: *const libc::c_char,
    mut cols: libc::c_int,
) {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    (*lbuf).output = output;
    (*lbuf).continuation = continuation;
    (*lbuf).indent = indent;
    (*lbuf).cols = cols as libc::c_short;
    (*lbuf).error = 0 as libc::c_short;
    (*lbuf).len = 0;
    (*lbuf).size = 0;
    (*lbuf).buf = 0 as *mut libc::c_char;

    debug_return!()
}

#[no_mangle]
pub unsafe extern "C" fn sudo_lbuf_destroy_v1(mut lbuf: *mut sudo_lbuf) {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    free((*lbuf).buf as *mut libc::c_void);
    (*lbuf).buf = 0 as *mut libc::c_char;

    debug_return!()
}

//static
unsafe extern "C" fn sudo_lbuf_expand(mut lbuf: *mut sudo_lbuf, mut extra: libc::c_int) -> bool {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    if (*lbuf).len + extra + 1 >= (*lbuf).size {
        let mut new_buf: *mut libc::c_char;
        let mut new_size: libc::c_int = (*lbuf).size;

        //do_while
        loop {
            new_size += 256;
            if !((*lbuf).len + extra + 1 >= new_size) {
                break;
            }
        }

        new_buf = realloc((*lbuf).buf as *mut libc::c_void, new_size as libc::c_ulong)
            as *mut libc::c_char;
        if new_buf.is_null() {
            sudo_debug_printf!(
                SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
                b"unable to allocate memory\0" as *const u8 as *const libc::c_char
            );

            (*lbuf).error = 1;
            debug_return_bool!(false);
        }
        (*lbuf).buf = new_buf;
        (*lbuf).size = new_size;
    }
    debug_return_bool!(true)
}

/* XXX - check output function return value */
unsafe extern "C" fn sudo_lbuf_println(
    mut lbuf: *mut sudo_lbuf,
    mut line: *mut libc::c_char,
    mut len: libc::c_int,
) {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut save: libc::c_char = 0;
    let mut i: libc::c_int = 0;
    let mut have: libc::c_int = 0;
    let mut contlen: libc::c_int = 0;
    let mut indent: libc::c_int = (*lbuf).indent;
    let mut is_comment: bool = false;

    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    if *line.offset(0 as isize) as libc::c_int == '#' as i32
        && *(*__ctype_b_loc())
            .offset(*line.offset(1 as isize) as libc::c_uchar as libc::c_int as isize)
            as libc::c_int
            & _ISblank as libc::c_ushort as libc::c_int
            != 0
    {
        is_comment = true;
        indent = 2;
    }

    if !((*lbuf).continuation).is_null() && !is_comment {
        contlen = strlen((*lbuf).continuation) as libc::c_int;
    }

    cp = line;
    have = (*lbuf).cols as libc::c_int;
    while !cp.is_null() && *cp as libc::c_int != '\0' as i32 {
        let mut ep: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut need: libc::c_int = len - cp.offset_from(line) as libc::c_long as libc::c_int;

        if need > have {
            have -= contlen;
            ep =
                memrchr(cp as *const libc::c_void, ' ' as i32, have as size_t) as *mut libc::c_char;
            if ep.is_null() {
                ep = memchr(
                    cp.offset(have as isize) as *const libc::c_void,
                    ' ' as i32,
                    (need - have) as libc::c_ulong,
                ) as *mut libc::c_char
            };
            if !ep.is_null() {
                need = ep.offset_from(cp) as libc::c_long as libc::c_int;
            }
        }
        if cp != line {
            if is_comment {
                ((*lbuf).output).expect("non-null function pointer")(
                    b"# \0" as *const u8 as *const libc::c_char,
                );
            } else {
                i = 0;
                while i < indent {
                    ((*lbuf).output).expect("non-null function pointer")(
                        b" \0" as *const u8 as *const libc::c_char,
                    );
                    i += 1;
                }
            }
        }
        save = *cp.offset(need as isize);
        *cp.offset(need as isize) = '\0' as i32 as libc::c_char;
        ((*lbuf).output).expect("non-null function pointer")(cp);
        *cp.offset(need as isize) = save;
        cp = ep;

        if !cp.is_null() {
            have = (*lbuf).cols as libc::c_int - indent;
            ep = line.offset(len as isize);
            while cp < ep
                && *(*__ctype_b_loc()).offset(*cp as libc::c_uchar as libc::c_int as isize)
                    as libc::c_int
                    & _ISblank as libc::c_int as libc::c_ushort as libc::c_int
                    != 0
            {
                cp = cp.offset(1);
            }
            if contlen != 0 {
                ((*lbuf).output).expect("non-null function pointer")((*lbuf).continuation);
            }
        }
        ((*lbuf).output).expect("non-null function pointer")(
            b"\n\0" as *const u8 as *const libc::c_char,
        );
    }
    debug_return!()
}

/*
 * Print the buffer with word wrap based on the tty width.
 * The lbuf is reset on return.
 * XXX - check output function return value
 */
#[no_mangle]
pub unsafe extern "C" fn sudo_lbuf_print_v1(mut lbuf: *mut sudo_lbuf) {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;

    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    if !(((*lbuf).buf).is_null() || (*lbuf).len == 0 as libc::c_int) {
        //len = lbuf->continuation ? strlen(lbuf->continuation) : 0;
        len = (if !((*lbuf).continuation).is_null() {
            strlen((*lbuf).continuation)
        } else {
            0 as libc::c_ulong
        }) as libc::c_int;

        if (*lbuf).cols as libc::c_int <= (*lbuf).indent + len + 20 as libc::c_int {
            if (*lbuf).len > 0 as libc::c_int {
                *((*lbuf).buf).offset((*lbuf).len as isize) = '\u{0}' as i32 as libc::c_char;
                ((*lbuf).output).expect("non-null function pointer")((*lbuf).buf);
                if *((*lbuf).buf).offset(((*lbuf).len - 1 as libc::c_int) as isize) as libc::c_int
                    != '\n' as i32
                {
                    ((*lbuf).output).expect("non-null function pointer")(
                        b"\n\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
        } else {
            //for
            cp = (*lbuf).buf;
            while !cp.is_null() && *cp as libc::c_int != '\u{0}' as i32 {
                if *cp as libc::c_int == '\n' as i32 {
                    ((*lbuf).output).expect("non-null function pointer")(
                        b"\n\0" as *const u8 as *const libc::c_char,
                    );
                    cp = cp.offset(1);
                } else {
                    len = ((*lbuf).len as libc::c_long
                        - cp.offset_from((*lbuf).buf) as libc::c_long)
                        as libc::c_int;
                    ep = memchr(cp as *const libc::c_void, '\n' as i32, len as libc::c_ulong)
                        as *mut libc::c_char;
                    if !ep.is_null() {
                        len = ep.offset_from(cp) as libc::c_long as libc::c_int;
                    }
                    if len != 0 {
                        sudo_lbuf_println(lbuf, cp, len);
                    }
                    cp = if !ep.is_null() {
                        ep.offset(1 as isize)
                    } else {
                        0 as *mut libc::c_char
                    };
                }
            }
        }
    }
    //done
    (*lbuf).len = 0;
    (*lbuf).error = 0 as libc::c_short;

    debug_return!()
}

#[no_mangle]
pub unsafe extern "C" fn sudo_lbuf_error_v1(mut lbuf: *mut sudo_lbuf) -> bool {
    if !lbuf.is_null() && (*lbuf).error as libc::c_int != 0 {
        return true;
    }
    return false;
}

#[no_mangle]
pub unsafe extern "C" fn sudo_lbuf_clearerr_v1(mut lbuf: *mut sudo_lbuf) {
    if !lbuf.is_null() {
        (*lbuf).error = 0 as libc::c_short;
    }
}
