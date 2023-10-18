/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(clashing_extern_declarations)]

use crate::struct_macro::*;

extern "C" {
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn tcsetpgrp(__fd: libc::c_int, __pgrp_id: __pid_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
}

static mut got_sigttou: sig_atomic_t = 0;
unsafe extern "C" fn sigttou(mut _signo: libc::c_int) {
    // got_sigttou = 1;
    ::core::ptr::write_volatile(&mut got_sigttou as *mut sig_atomic_t, 1 as libc::c_int);
}

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