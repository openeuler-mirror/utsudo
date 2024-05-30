/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn gmtime(__timer: *const time_t) -> *mut tm;
    fn localtime(__timer: *const time_t) -> *mut tm;
}
//多处定义，后期统一处理
pub type __time_t = libc::c_long;
pub type time_t = __time_t;

//多处定义，后期统一处理
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
#[no_mangle]
pub unsafe extern "C" fn get_gmtoff(mut clock: *mut time_t) -> libc::c_long {
    let mut gm: *mut tm = 0 as *mut tm;
    let mut gmt: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut local: *mut tm = 0 as *mut tm;
    let mut offset: libc::c_long = 0;
    gm = gmtime(clock);
    if gm.is_null() {
        return 0 as libc::c_long;
    }
    gmt = *gm;
    local = localtime(clock);
    if local.is_null() {
        return 0 as libc::c_long;
    }
    offset = ((*local).tm_sec - gmt.tm_sec
        + ((*local).tm_min - gmt.tm_min) * 60
        + ((*local).tm_hour - gmt.tm_hour) * 3600) as libc::c_long;

    /* Timezone may cause year rollover to happen on a different day. */
    if (*local).tm_year < gmt.tm_year {
        offset -= (24 * 3600) as libc::c_long;
    } else if (*local).tm_year > gmt.tm_year {
        offset -= (24 * 3600) as libc::c_long;
    } else if (*local).tm_yday < gmt.tm_yday {
        offset -= (24 * 3600) as libc::c_long;
    } else if (*local).tm_yday > gmt.tm_yday {
        offset += (24 * 3600) as libc::c_long;
    }
    return offset;
}
