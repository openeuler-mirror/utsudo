
/*
 * Store the name of the tty to which the process is attached in name.
 * Returns name on success and NULL on failure, setting errno.
 */
#[no_mangle]
pub unsafe extern "C" fn get_process_ttyname(
    mut name: *mut libc::c_char,
    mut namelen: size_t,
) -> *mut libc::c_char {
    let path: [libc::c_char; 16] =
        *::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"/proc/self/stat\0");
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut serrno: libc::c_int = *__errno_location();
    let mut nread: ssize_t = 0 as ssize_t;
    let mut fd: libc::c_int = 0;
    debug_decl!(stdext::function_name!().as_ptr(), SUDO_DEBUG_UTIL);

    /*
     * Try to determine the tty from tty_nr in /proc/self/stat.
     * Ignore /proc/self/stat if it contains embedded NUL bytes.
     */
    fd = open(path.as_ptr(), O_RDONLY | O_NOFOLLOW);
    if fd != -(1 as libc::c_int) {
        cp = buf.as_mut_ptr();

        loop {
            nread = read(
                fd,
                cp as *mut libc::c_void,
                buf.as_mut_ptr()
                    .offset(::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as isize)
                    .offset_from(cp) as libc::c_long as size_t,
            );

            if !(nread != 0) {
                break;
            }

            if nread == -(1 as libc::c_int) as libc::c_long {
                if *__errno_location() == EAGAIN || *__errno_location() == EINTR {
                    continue;
                }
                break;
            }

            cp = cp.offset(nread as isize);
            if cp
                >= buf
                    .as_mut_ptr()
                    .offset(::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as isize)
            {
                break;
            }
        } // !loop

        if nread == 0
            && memchr(
                buf.as_mut_ptr() as *const libc::c_void,
                '\0' as i32,
                cp.offset_from(buf.as_mut_ptr()) as libc::c_long as libc::c_ulong,
            )
            .is_null()
        {
            /*
             * Field 7 is the tty dev (0 if no tty).
             * Since the process name at field 2 "(comm)" may include
             * whitespace (including newlines), start at the last ')' found.
             */
            *cp = '\0' as i32 as libc::c_char;
            cp = strrchr(buf.as_mut_ptr(), ')' as i32);

            if !cp.is_null() {
                let mut ep: *mut libc::c_char = cp;
                let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
                let mut field: libc::c_int = 1;

                loop {
                    ep = ep.offset(1);
                    if !(*ep as libc::c_int != '\0' as i32) {
                        break;
                    }
                    if *ep as libc::c_int == ' ' as i32 {
                        *ep = '\0' as i32 as libc::c_char;
                        field += 1;

                        if field == 7 as libc::c_int {
                            let mut tty_nr: libc::c_int =
                                sudo_strtonum(cp, INT_MIN!(), INT_MAX!(), &mut errstr)
                                    as libc::c_int;
                            if !errstr.is_null() {
                                sudo_debug_printf!(
                                    SUDO_DEBUG_ERROR | SUDO_DEBUG_LINENO,
                                    b"%s: tty device %s: %s\0" as *const u8 as *const libc::c_char,
                                    path,
                                    cp,
                                    errstr
                                );
                            }

                            if tty_nr != 0 {
                                /*
                                 * Avoid sign extension when assigning tdev.
                                 * tty_nr in /proc/self/stat is printed as a
                                 * signed int but the actual device number is an
                                 * unsigned int and dev_t is unsigned long long.
                                 */
                                let mut tdev: dev_t = tty_nr as libc::c_uint as dev_t;
                                *__errno_location() = serrno;
                                ret = sudo_ttyname_dev_v1(tdev, name, namelen);
                                // done:
                                if fd != -(1 as libc::c_int) {
                                    close(fd);
                                }
                                if ret.is_null() {
                                    sudo_debug_printf!(
                                        SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                                        b"unable to resolve tty via %s\0" as *const u8
                                            as *const libc::c_char,
                                        path.as_ptr()
                                    );
                                }
                                debug_return_str!(ret as *mut libc::c_char);
                            } //  ! if !(tty_nr == 0
                            break;
                        } // !if field
                        cp = ep.offset(1 as libc::c_int as isize);
                    }
                } // !loop
            } //  !cp.is_null()
        }
    } // if !fd;

    *__errno_location() = ENOENT;

    // done:
    if fd != -(1 as libc::c_int) {
        close(fd);
    }
    if ret.is_null() {
        sudo_debug_printf!(
            SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
            b"unable to resolve tty via %s\0" as *const u8 as *const libc::c_char,
            path.as_ptr()
        );
    }
    debug_return_str!(ret as *mut libc::c_char);
}