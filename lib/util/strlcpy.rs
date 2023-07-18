#![allow(non_camel_case_types, dead_code, unused_mut, unused_variables)]
pub type size_t = libc::c_ulong;

#[no_mangle]
fn sudo_strlcpy(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut dsize: size_t,
) -> size_t{
    let mut osrc: *const libc::c_char = src;
    let mut nleft: size_t = dsize;
}