/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(
    non_camel_case_types,
    unused_mut,
    unused_unsafe,
    dead_code,
    non_upper_case_globals,
    unused_variables,
    unused_assignments
)]

//call libc_func
use libc::abort;

//define
pub const RSBUFSZ: i32 = 1024;
pub const __SIZEOF_PTHREAD_MUTEX_T: usize = 40;
pub const KEYSZ: i32 = 32;
pub const IVSZ: i32 = 8;

//aliase of type
pub type pid_t = libc::c_int;
pub type uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __pthread_list_t = __pthread_internal_list;
pub type sig_atomic_t = __sig_atomic_t;
pub type __sig_atomic_t = libc::c_int;
pub type u8 = libc::c_uchar;
pub type u32 = libc::c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _rs {
    pub rs_have: size_t,
    pub rs_count: size_t,
}

//line 78
static mut rs: *mut _rs = 0 as *const _rs as *mut _rs;

//line 81
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _rsx {
    pub rs_chacha: chacha_ctx,
    pub rs_buf: [libc::c_uchar; 1024],
}

//line 82 included line81
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chacha_ctx {
    pub input: [u32; 16],
}

//line 84
static mut rsx: *mut _rsx = 0 as *const _rsx as *mut _rsx;

//called by struct __pthread_mutex_s -> __pthread_list_t
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}

//called by union pthread_mutex_t
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}

//called by _ARC4_LOCK
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; __SIZEOF_PTHREAD_MUTEX_T],
    pub __align: libc::c_long,
}

//arc4random.h_line34,init arc4random_mtx,called by 208
static mut arc4random_mtx: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_uint,
            __kind: 0 as libc::c_int,
            __spins: 0 as libc::c_short,
            __elision: 0 as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *mut __pthread_internal_list,
                    __next: 0 as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};

#[no_mangle]
unsafe fn _rs_stir() {
    let mut rnd: [libc::c_uchar; (KEYSZ + IVSZ) as usize] = [0; (KEYSZ + IVSZ) as usize];

    if getentropy(
        rnd.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_uchar; (KEYSZ + IVSZ) as usize]>() as libc::c_ulong,
    ) == -1
    {
        _getentropy_fail();
    }

    if rs.is_null() {
        _rs_init(
            rnd.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_uchar; (KEYSZ + IVSZ) as usize]>() as libc::c_ulong,
        );
    } else {
        _rs_rekey(
            rnd.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_uchar; (KEYSZ + IVSZ) as usize]>() as libc::c_ulong,
        );
    }

    sudo_memset_s(
        rnd.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_uchar; (KEYSZ + IVSZ) as usize]>() as libc::c_ulong,
        0,
        ::std::mem::size_of::<[libc::c_uchar; (KEYSZ + IVSZ) as usize]>() as libc::c_ulong,
    );

    (*rs).rs_have = 0;

    memset(
        (*rsx).rs_buf.as_mut_ptr() as *mut libc::c_void,
        0,
        ::std::mem::size_of::<[libc::c_uchar; (16 * RSBUFSZ) as usize]>() as libc::c_ulong,
    );
    (*rs).rs_count = 1600000;
}

extern "C" {
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    fn getentropy(__buffer: *mut libc::c_void, __length: size_t) -> libc::c_int;
    fn sudo_memset_s(v: *mut libc::c_void, smax: size_t, c: libc::c_int, n: size_t) -> libc::c_int;
    fn memset(__s: *mut libc::c_void, __s2: libc::c_int, __n: libc::c_ulong) -> *mut libc::c_void;
    fn memcpy(
        __dest: *mut libc::c_void,
        __rsc: *const libc::c_void,
        __n: size_t,
    ) -> *mut libc::c_void;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
}

//function _getentropy_fail
use libc::raise;
pub const SIGKILL: libc::c_int = 9;
pub unsafe fn _getentropy_fail() {
    raise(SIGKILL);
}

//function _rs_forkdetect
static mut wipeonfork: libc::c_int = 0;
use libc::getpid;
static mut _rs_forked: __sig_atomic_t = 0;
pub unsafe fn _rs_forkdetect() {
    if wipeonfork == 0 {
        static mut _rs_pid: pid_t = 0;
        let mut pid: pid_t = getpid();
        if _rs_pid == 0 || _rs_pid != pid || _rs_forked != 0 {
            _rs_pid = pid;
            _rs_forked = 0;
            if !rs.is_null() {
                memset(
                    rs as *mut libc::c_void,
                    0,
                    ::std::mem::size_of::<_rs>() as size_t,
                );
            }
        }
    }
}

//function _rs_allocate
pub const PROT_READ: libc::c_short = 0x1;
pub const PROT_WRITE: libc::c_short = 0x2;
pub const MAP_ANON: libc::c_short = 0x20;
pub const MAP_FAILED: libc::c_int = -1;
pub const MAP_PRIVATE: libc::c_short = 0x02;
pub const MADV_WIPEONFORK: libc::c_int = 18;
use libc::madvise;
use libc::pthread_atfork;
pub unsafe extern "C" fn _rs_forkhandler() {
    _rs_forked = 1;
}
pub unsafe fn _rs_allocate(mut rsp: *mut *mut _rs, mut rsxp: *mut *mut _rsx) -> libc::c_int {
    *rsp = mmap(
        0 as *mut libc::c_void,
        (::std::mem::size_of::<_rs>()) as size_t,
        PROT_READ as libc::c_int | PROT_WRITE as libc::c_int,
        MAP_ANON as libc::c_int | MAP_PRIVATE as libc::c_int,
        -(1 as libc::c_int),
        0,
    ) as *mut _rs;
    if *rsp == MAP_FAILED as *mut _rs {
        return -1;
    }
    *rsxp = mmap(
        0 as *mut libc::c_void,
        (::std::mem::size_of::<_rsx>()) as size_t,
        PROT_READ as libc::c_int | PROT_WRITE as libc::c_int,
        MAP_ANON as libc::c_int | MAP_PRIVATE as libc::c_int,
        -(1 as libc::c_int),
        0,
    ) as *mut _rsx;
   if *rsxp == MAP_FAILED as *mut _rsx {
        munmap(
            *rsp as *mut libc::c_void,
            ::std::mem::size_of::<_rsx>() as size_t,
        );
        *rsp = 0 as *mut _rs;
        return -1;
    }
    if (madvise(
        *rsp as *mut libc::c_void,
        ::std::mem::size_of::<_rs>(),
        MADV_WIPEONFORK,
    ) == 0)
        && (madvise(
            *rsxp as *mut libc::c_void,
            ::std::mem::size_of::<_rsx>(),
            MADV_WIPEONFORK,
        ) == 0)
    {
        wipeonfork = 1;
    }

    pthread_atfork(
        None,
        None,
        Some(_rs_forkhandler as unsafe extern "C" fn() -> ()),
    );
    return 0;
}

//function chacha_keysetup
macro_rules! U8TO32_LITTLE {
    ($a:expr,$b:expr) => {{
        (*$a.offset($b as libc::c_int as isize)
            .offset(0 as libc::c_int as isize) as libc::c_uint)
            << 0 as libc::c_int
            | (*$a
                .offset($b as libc::c_int as isize)
                .offset(1 as libc::c_int as isize) as libc::c_uint)
                << 8 as libc::c_int
            | (*$a
                .offset($b as libc::c_int as isize)
                .offset(2 as libc::c_int as isize) as libc::c_uint)
                << 16 as libc::c_int
            | (*$a
                .offset($b as libc::c_int as isize)
                .offset(3 as libc::c_int as isize) as libc::c_uint)
                << 24 as libc::c_int
    }};
}

static mut sigma: [libc::c_char; 16] =
    unsafe { *::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"expand 32-byte k") };
static mut tau: [libc::c_char; 16] =
    unsafe { *::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"expand 16-byte k") };

#[no_mangle]
pub unsafe fn chacha_keysetup(
    mut x: *mut chacha_ctx,
    mut k: *const u8,
    mut kbits: u32,
    mut ivbits: u32,
) {
    let mut constants: *const libc::c_char = 0 as *const libc::c_char;

    (*x).input[4] = U8TO32_LITTLE!(k, 0);
    (*x).input[5] = U8TO32_LITTLE!(k, 4);
    (*x).input[6] = U8TO32_LITTLE!(k, 8);
    (*x).input[7] = U8TO32_LITTLE!(k, 12);

    if kbits == 256 {
        k = k.offset(16);
        constants = sigma.as_ptr();
    } else {
        constants = tau.as_ptr();
    }

    (*x).input[8] = U8TO32_LITTLE!(k, 0);
    (*x).input[9] = U8TO32_LITTLE!(k, 4);
    (*x).input[10] = U8TO32_LITTLE!(k, 8);
    (*x).input[11] = U8TO32_LITTLE!(k, 12);
    (*x).input[0] = U8TO32_LITTLE!(constants, 0);
    (*x).input[1] = U8TO32_LITTLE!(constants, 4);
    (*x).input[2] = U8TO32_LITTLE!(constants, 8);
    (*x).input[3] = U8TO32_LITTLE!(constants, 12);
}








