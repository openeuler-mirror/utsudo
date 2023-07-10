/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(
    non_camel_case_types,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]


#[no_mangle]
pub unsafe extern "C" fn sudo_str2sig(
    mut signame: *const libc::c_char,
    mut result: *mut libc::c_int,
) -> libc::c_int {
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut signo: libc::c_int = 0;

    if (*(*__ctype_b_loc()).offset(*signame.offset(0 as isize) as libc::c_uchar as isize)
        as libc::c_int)
        != 0
    {
        signo = sudo_strtonum(
            signame,
            0 as libc::c_longlong,
            (NSIG - 1) as libc::c_longlong,
            &mut errstr,
        ) as libc::c_int;
        
        *result = signo;
        return 0;
    }
    return 0;
}
