/* sha2.rs */

extern "C" {
    fn memset()
    fn memcpy()
    fn sudo_memset_s()
}

pub unsafe extern "C" fn sudo_SHA224Init

pub unsafe extern "C" fn sudo_SHA224Pad

pub unsafe extern "C" fn sudo_SHA224Final


pub unsafe extern "C" fn sudo_SHA256Init

pub unsafe extern "C" fn sudo_SHA256Transform

pub unsafe extern "C" fn sudo_SHA256Update

pub unsafe extern "C" fn sudo_SHA256Pad

pub unsafe extern "C" fn sudo_SHA256Final

pub unsafe extern "C" fn sudo_SHA384Init

pub unsafe extern "C" fn sudo_SHA384Transform

pub unsafe extern "C" fn sudo_SHA384Update

pub unsafe extern "C" fn sudo_SHA384Pad

pub unsafe extern "C" fn sudo_SHA384Final

pub unsafe extern "C" fn sudo_SHA512Init

pub unsafe extern "C" fn sudo_SHA512Transform

pub unsafe extern "C" fn sudo_SHA512Update

pub unsafe extern "C" fn sudo_SHA512Pad

pub unsafe extern "C" fn sudo_SHA512Final
