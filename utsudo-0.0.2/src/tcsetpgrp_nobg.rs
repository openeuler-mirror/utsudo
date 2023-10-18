/*
* Like tcsetpgrp() but restarts on EINTR _except_ for SIGTTOU.
* Returns 0 on success or -1 on failure, setting errno.
* Sets got_sigttou on failure if interrupted by SIGTTOU.
*/
#[no_mangle]
pub unsafe extern "C" fn tcsetpgrp_nobg(mut fd: libc::c_int, mut pgrp_id: pid_t) -> libc::c_int {
    let mut sa: sigaction = sigaction {
        __sigaction_handler: Signal_handler { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };

    let mut osa: sigaction = sigaction {
        __sigaction_handler: Signal_handler { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };

    let mut rc: libc::c_int = 0;

    /*
     * If we receive SIGTTOU from tcsetpgrp() it means we are
     * not in the foreground process group.
     * This avoid a TOCTOU race compared to using tcgetpgrp().
     */
    memset(
        &mut sa as *mut sigaction as *mut libc::c_void,
        0,
        std::mem::size_of::<sigaction>() as libc::c_ulong,
    );
    sigemptyset(&mut sa.sa_mask);
    sa.sa_flags = 0; /* do not restart syscalls */
    ::core::ptr::write_volatile(&mut got_sigttou as *mut sig_atomic_t, 0 as libc::c_int);
    sigaction(SIGTTOU, &mut sa, &mut osa);

    loop {
        rc = tcsetpgrp(fd, pgrp_id);
        if !(rc != 0 && *__errno_location() == EINTR && got_sigttou == 0) {
            break;
        }
    }
    sigaction(SIGTTOU, &mut osa, 0 as *mut sigaction);
    return rc;
}