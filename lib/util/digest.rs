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

//define
pub const EINVAL: libc::c_int = 22;

extern "C" {
    fn __errno_location() -> *mut libc::c_int;
}

pub struct digest_function {
    pub digest_len: libc::c_uint,
    pub init: Option<unsafe extern "C" fn(*mut SHA2_CTX) -> ()>,
    pub update: Option<unsafe extern "C" fn(*mut SHA2_CTX, *const libc::c_uchar, size_t) -> ()>,
    pub final_0: Option<unsafe extern "C" fn(*mut libc::c_uchar, *mut SHA2_CTX) -> ()>,
}

pub struct sudo_digest {
    pub func: *mut digest_function,
    pub ctx: SHA2_CTX,
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
