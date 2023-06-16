#[no_mangle]
pub unsafe extern "C" fn initprogname(mut name: *const libc::c_char) {
    if !__progname.is_null() && *__progname as libc::c_int != '\u{0}' as i32 {
        progname = __progname;
    } else {
        progname = strrchr(name, '/' as i32);
        if !progname.is_null() {
            progname = progname.offset(1); //progname++
        } else {
            progname = name;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn sudo_getprogname() -> *const libc::c_char {
    return progname;
}

