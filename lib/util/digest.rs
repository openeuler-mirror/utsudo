/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

//call other file's func
use crate::sha2::sudo_SHA224Final;
use crate::sha2::sudo_SHA224Init;
use crate::sha2::sudo_SHA224Update;
use crate::sha2::sudo_SHA256Final;
use crate::sha2::sudo_SHA256Init;
use crate::sha2::sudo_SHA256Update;
use crate::sha2::sudo_SHA384Final;
use crate::sha2::sudo_SHA384Init;
use crate::sha2::sudo_SHA384Update;
use crate::sha2::sudo_SHA512Final;
use crate::sha2::sudo_SHA512Init;
use crate::sha2::sudo_SHA512Update;
use crate::sha2::SHA2_CTX;
use crate::sudo_debug::sudo_debug_enter_v1;
use crate::sudo_debug::sudo_debug_exit_int_v1;
use crate::sudo_debug::sudo_debug_exit_ptr_v1;
use crate::sudo_debug::sudo_debug_exit_v1;
use crate::sudo_debug_macro::SUDO_DEBUG_UTIL;
use libc::free;
use libc::malloc;

pub type size_t = libc::c_ulong;
pub type uint8_t = libc::c_uchar;

//define
pub const EINVAL: libc::c_int = 22;
pub const SHA256_DIGEST_LENGTH: libc::c_uint = 32;
pub const SHA512_DIGEST_LENGTH: libc::c_uint = 64;

extern "C" {
    fn __errno_location() -> *mut libc::c_int;
}

pub struct digest_function {
    pub digest_len: libc::c_uint,
    pub init: Option<unsafe extern "C" fn(*mut SHA2_CTX) -> ()>,
    pub update: Option<unsafe extern "C" fn(*mut SHA2_CTX, *const libc::c_uchar, size_t) -> ()>,
    pub final_0: Option<unsafe extern "C" fn(*mut libc::c_uchar, *mut SHA2_CTX) -> ()>,
}

static mut digest_functions: [digest_function; 3] = {
    [
        {
            let mut init = digest_function {
                digest_len: SHA256_DIGEST_LENGTH as libc::c_uint,
                init: Some(sudo_SHA256Init as unsafe extern "C" fn(*mut SHA2_CTX)),
                update: Some(
                    sudo_SHA256Update
                        as unsafe extern "C" fn(*mut SHA2_CTX, *const uint8_t, size_t),
                ),
                final_0: Some(
                    sudo_SHA256Final as unsafe extern "C" fn(*mut uint8_t, *mut SHA2_CTX),
                ),
            };
            init
        },
        {
            let mut init = digest_function {
                digest_len: SHA512_DIGEST_LENGTH as libc::c_uint,
                init: Some(sudo_SHA512Init as unsafe extern "C" fn(*mut SHA2_CTX)),
                update: Some(
                    sudo_SHA512Update
                        as unsafe extern "C" fn(*mut SHA2_CTX, *const uint8_t, size_t),
                ),
                final_0: Some(
                    sudo_SHA512Final as unsafe extern "C" fn(*mut uint8_t, *mut SHA2_CTX),
                ),
            };
            init
        },
        {
            let mut init = digest_function {
                digest_len: 0 as libc::c_uint,
                init: None,
                update: None,
                final_0: None,
            };
            init
        },
    ]
};

pub struct sudo_digest {
    pub func: *mut digest_function,
    pub ctx: SHA2_CTX,
}

//line 97
#[no_mangle]
unsafe extern "C" fn sudo_digest_alloc_v1(mut digest_type: libc::c_int) -> *mut sudo_digest {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    //line 100
    let mut func: *mut digest_function = 0 as *mut digest_function;
    let mut dig: *mut sudo_digest = 0 as *mut sudo_digest;
    let mut i: libc::c_int = 0;

    //line104-109
    // i = 0;
    while digest_functions[i as usize].digest_len != 0 {
        if digest_type == i {
            func = &mut *digest_functions.as_mut_ptr().offset(i as isize) as *mut digest_function;
            break;
        } else {
            i += 1;
        }
    }
    
    //line110-113
    if func.is_null() {
        *__errno_location() = EINVAL;
        debug_return_ptr!(0 as *mut sudo_digest);
    }

    //115
    dig = malloc(::std::mem::size_of::<sudo_digest>() as libc::size_t) as *mut sudo_digest;
    if dig.is_null() {
        debug_return_ptr!(0 as *mut sudo_digest);
    }

    //line 117 118
    ((*func).init).expect("is not a function pointer")(&mut (*dig).ctx);
    (*dig).func = func;

    //line 120
    debug_return_ptr!(dig);
}

#[no_mangle]
unsafe fn sudo_digest_free_v1(mut dig: *mut sudo_digest) {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    free(dig as *mut libc::c_void);

    debug_return!()
}

#[no_mangle]
unsafe fn sudo_digest_reset_v1(mut dig: *mut sudo_digest) {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    ((*(*dig).func).init).expect("is not func pointer")(&mut (*dig).ctx);

    debug_return!()
}

#[no_mangle]
unsafe fn sudo_digest_getlen_v1(mut digest_type: libc::c_int) -> libc::c_int {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    let mut i: libc::c_int = 0;

    //line 149
    while digest_functions[i as usize].digest_len != 0 {
        if digest_type == i {
            debug_return_int!(digest_functions[i as usize].digest_len as libc::c_int);
        }
        i += 1;
    }

    //line 154
    debug_return_int!(-1)
}

#[no_mangle]
unsafe fn sudo_digest_update_v1(
    mut dig: *mut sudo_digest,
    mut data: *const libc::c_void,
    mut len: size_t,
) {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    //162
    ((*(*dig).func).update).expect("is not null func point")(
        &mut (*dig).ctx,
        data as *const libc::c_uchar,
        len,
    );

    //line164
    debug_return!()
}

//line168
#[no_mangle]
unsafe fn sudo_digest_final_v1(mut dig: *mut sudo_digest, mut md: *mut libc::c_uchar) {
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    ((*(*dig).func).final_0).expect("is not func point")(md, &mut (*dig).ctx);

    debug_return!()
}

