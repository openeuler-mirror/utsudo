#[no_mangle]
pub unsafe extern "C" fn initprogname(mut name: *const libc::c_char) {

}

#[no_mangle]
pub unsafe extern "C" fn sudo_getprogname() -> *const libc::c_char {
    return progname;
}
