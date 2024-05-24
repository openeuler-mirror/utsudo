/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

use crate::common::*;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn time(__timer: *mut time_t) -> time_t;
    fn gmtime(__timer: *const time_t) -> *mut tm;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
}
// 多处定义，后期统一处理
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
// 多处定义，后期统一处理
pub const _ISalnum: libc::c_uint = 8;
pub const _ISpunct: libc::c_uint = 4;
pub const _IScntrl: libc::c_uint = 2;
pub const _ISblank: libc::c_uint = 1;
pub const _ISgraph: libc::c_uint = 32768;
pub const _ISprint: libc::c_uint = 16384;
pub const _ISspace: libc::c_uint = 8192;
pub const _ISxdigit: libc::c_uint = 4096;
pub const _ISdigit: libc::c_uint = 2048;
pub const _ISalpha: libc::c_uint = 1024;
pub const _ISlower: libc::c_uint = 512;
pub const _ISupper: libc::c_uint = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _TABLE {
    pub name: *mut libc::c_char,
    pub type_0: libc::c_int,
    pub value: time_t,
}
pub type TABLE = _TABLE;
pub type _DSTMODE = libc::c_uint;
pub const DSTmaybe: _DSTMODE = 2;
pub const DSToff: _DSTMODE = 1;
pub const DSTon: _DSTMODE = 0;
pub type DSTMODE = _DSTMODE;
pub const MER24: libc::c_uint = 2;
pub const MERpm: libc::c_uint = 1;
pub const MERam: libc::c_uint = 0;
pub type MERIDIAN = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub union YYSTYPE {
    pub Number: time_t,
    pub Meridian: libc::c_uint,
}
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
static mut yyInput: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut yyDSTmode: DSTMODE = DSTon;
static mut yyDayOrdinal: time_t = 0;
static mut yyDayNumber: time_t = 0;
static mut yyHaveDate: libc::c_int = 0;
static mut yyHaveDay: libc::c_int = 0;
static mut yyHaveRel: libc::c_int = 0;
static mut yyHaveTime: libc::c_int = 0;
static mut yyHaveZone: libc::c_int = 0;
static mut yyTimezone: time_t = 0;
static mut yyDay: time_t = 0;
static mut yyHour: time_t = 0;
static mut yyMinutes: time_t = 0;
static mut yyMonth: time_t = 0;
static mut yySeconds: time_t = 0;
static mut yyYear: time_t = 0;
static mut yyMeridian: MERIDIAN = MERam;
static mut yyRelMonth: time_t = 0;
static mut yyRelSeconds: time_t = 0;
#[no_mangle]
pub static mut yylhs: [libc::c_short; 42] = [
    -1, 0, 0, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 6, 6, 6, 5, 5, 5, 5, 5, 5, 5, 5, 7, 7, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 8, 1, 1,
];
#[no_mangle]
pub static mut yylen: [libc::c_short; 42] = [
    2, 0, 2, 1, 1, 1, 1, 1, 1, 2, 4, 4, 6, 6, 1, 1, 2, 1, 2, 2, 3, 5, 3, 3, 2, 4, 2, 3, 2, 1, 2, 2,
    1, 2, 2, 1, 2, 2, 1, 1, 0, 1,
];
#[no_mangle]
pub static mut yydefred: [libc::c_short; 51] = [
    1, 0, 0, 15, 32, 0, 38, 35, 0, 0, 0, 2, 3, 4, 5, 6, 7, 8, 0, 18, 0, 31, 36, 33, 19, 9, 30, 0,
    37, 34, 0, 0, 0, 16, 28, 0, 23, 27, 22, 0, 0, 25, 41, 11, 0, 10, 0, 0, 21, 13, 12,
];
#[no_mangle]
pub static mut yydgoto: [libc::c_short; 10] = [1, 45, 11, 12, 13, 14, 15, 16, 17, 18];
#[no_mangle]
pub static mut yysindex: [libc::c_short; 51] = [
    0, -249, -38, 0, 0, -260, 0, 0, -240, -47, -248, 0, 0, 0, 0, 0, 0, 0, -237, 0, -18, 0, 0, 0, 0,
    0, 0, -262, 0, 0, -239, -238, -236, 0, 0, -235, 0, 0, 0, -56, -19, 0, 0, 0, -234, 0, -232,
    -258, 0, 0, 0,
];
#[no_mangle]
pub static mut yyrindex: [libc::c_short; 51] = [
    0, 0, 1, 0, 0, 0, 0, 0, 0, 69, 12, 0, 0, 0, 0, 0, 0, 0, 23, 0, 34, 0, 0, 0, 0, 0, 0, 67, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 56, 45, 0, 0, 0, 0, 0, 0, 56, 0, 0, 0,
];
#[no_mangle]
pub static mut yygindex: [libc::c_short; 10] = [0, -17, 0, 0, 0, 0, 0, 0, 0, 0];
#[no_mangle]
pub static mut yytable: [libc::c_short; 338] = [
    32, 17, 44, 42, 36, 37, 19, 20, 49, 2, 3, 31, 14, 4, 5, 6, 7, 8, 9, 10, 34, 33, 21, 29, 22, 23,
    35, 38, 46, 39, 50, 40, 41, 47, 24, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 43, 24,
    0, 0, 25, 26, 27, 28, 29, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 17, 0, 0, 17, 17, 17, 17, 17, 17, 17,
    14, 14, 0, 0, 14, 14, 14, 14, 14, 14, 14, 29, 29, 0, 0, 29, 29, 29, 29, 29, 29, 29, 24, 24, 0,
    0, 24, 24, 24, 24, 24, 24, 24, 20, 20, 0, 0, 20, 20, 20, 20, 20, 20, 20, 40, 40, 0, 0, 40, 40,
    40, 40, 0, 40, 40, 26, 26, 0, 39, 26, 26, 26, 26, 0, 0, 26, 39, 39,
];
#[no_mangle]
pub static mut yycheck: [libc::c_short; 338] = [
    47, 0, 58, 261, 266, 267, 44, 267, 266, 258, 259, 58, 0, 262, 263, 264, 265, 266, 267, 268,
    257, 269, 262, 0, 264, 265, 44, 266, 47, 267, 47, 267, 267, 267, 0, 267, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, 0, -1, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 261, -1, -1,
    -1, -1, 266, 258, -1, -1, 261, 262, 263, 264, 265, 266, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, 258, 259, -1, -1, 262, 263, 264, 265, 266, 267, 268, 258, 259, -1, -1, 262,
    263, 264, 265, 266, 267, 268, 258, 259, -1, -1, 262, 263, 264, 265, 266, 267, 268, 258, 259,
    -1, -1, 262, 263, 264, 265, 266, 267, 268, 258, 259, -1, -1, 262, 263, 264, 265, 266, 267, 268,
    258, 259, -1, -1, 262, 263, 264, 265, -1, 267, 268, 258, 259, -1, 259, 262, 263, 264, 265, -1,
    -1, 268, 267, 268,
];

pub const tAGO: libc::c_int = 257;
pub const tDAY: libc::c_int = 258;
pub const tDAYZONE: libc::c_int = 259;
pub const tID: libc::c_int = 260;
pub const tMERIDIAN: libc::c_int = 261;
pub const tMINUTE_UNIT: libc::c_int = 262;
pub const tMONTH: libc::c_int = 263;
pub const tMONTH_UNIT: libc::c_int = 264;
pub const tSEC_UNIT: libc::c_int = 265;
pub const tSNUMBER: libc::c_int = 266;
pub const tUNUMBER: libc::c_int = 267;
pub const tZONE: libc::c_int = 268;
pub const tDST: libc::c_int = 269;
pub const YYERRCODE: libc::c_int = 256;
pub const EPOCH: libc::c_int = 1970;
// #define SECSPERDAY	(24L * 60L * 60L)
pub const SECSPERDAY: libc::c_long = 86400;

#[macro_export]
macro_rules! TABLENew {
    ($name:expr, $type:expr, $value:expr) => {
        _TABLE {
            name: $name as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: $type as libc::c_int,
            value: $value as libc::c_int as time_t,
        }
    };
}

#[macro_export]
macro_rules! HOUR {
    ($x:expr) => {
        $x as libc::c_int as time_t * 60 as libc::c_int as libc::c_long
    };
}
