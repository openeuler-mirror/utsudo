static mut default_base: *mut sudo_event_base = 0 as *const sudo_event_base as *mut sudo_event_base;
static mut signal_base: *mut sudo_event_base = 0 as *const sudo_event_base as *mut sudo_event_base;

#[no_mangle]
pub unsafe extern "C" fn sudo_ev_activate(mut base: *mut sudo_event_base, mut ev: *mut sudo_event) {
    (*ev).active_entries.tqe_next = 0 as *mut sudo_event;
    (*ev).active_entries.tqe_prev = (*base).active.tqh_last;
    *(*base).active.tqh_last = ev;
    (*base).active.tqh_last = &mut (*ev).active_entries.tqe_next;
    (*ev).flags = ((*ev).flags as libc::c_int | 0x2 as libc::c_int) as libc::c_short;
}

#[inline]
unsafe extern "C" fn sudo_ev_deactivate(mut base: *mut sudo_event_base, mut ev: *mut sudo_event) {
    (*ev).flags = ((*ev).flags as libc::c_int & !(0x2 as libc::c_int)) as libc::c_short;
    if !((*ev).active_entries.tqe_next).is_null() {
        (*(*ev).active_entries.tqe_next).active_entries.tqe_prev = (*ev).active_entries.tqe_prev;
    } else {
        (*base).active.tqh_last = (*ev).active_entries.tqe_prev;
    }
    *(*ev).active_entries.tqe_prev = (*ev).active_entries.tqe_next;
}

unsafe extern "C" fn sudo_ev_deactivate_all(mut base: *mut sudo_event_base) {
    let mut ev: *mut sudo_event = 0 as *mut sudo_event;
    let sudo_debug_subsys: libc::c_int = (4 as libc::c_int) << 6 as libc::c_int;
    sudo_debug_enter_v1(
        (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(b"sudo_ev_deactivate_all\0"))
            .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        89 as libc::c_int,
        sudo_debug_subsys,
    );

        loop {
        ev = (*base).active.tqh_first;
        if ev.is_null() {
            break;
        }
        sudo_ev_deactivate(base, ev);
    }
    sudo_debug_exit_v1(
        (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(b"sudo_ev_deactivate_all\0"))
            .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        94 as libc::c_int,
        sudo_debug_subsys,
    );
}

unsafe extern "C" fn sudo_ev_activate_sigevents(mut base: *mut sudo_event_base) {
    let mut ev: *mut sudo_event = 0 as *mut sudo_event;
    let mut set: sigset_t = sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = sigset_t { __val: [0; 16] };
    let mut i: libc::c_int = 0;
    let sudo_debug_subsys: libc::c_int = (4 as libc::c_int) << 6 as libc::c_int;
    sudo_debug_enter_v1(
        (*::core::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
            b"sudo_ev_activate_sigevents\0",
        ))
        .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        107 as libc::c_int,
        sudo_debug_subsys,
    );

        sigfillset(&mut set);
    sigprocmask(0 as libc::c_int, &mut set, &mut oset);
    (*base).signal_caught = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int + 1 as libc::c_int {
        if !((*base).signal_pending[i as usize] == 0) {
            (*base).signal_pending[i as usize] = 0 as libc::c_int;
            ev = (*base).signals[i as usize].tqh_first;
            while !ev.is_null() {
                if (*ev).events as libc::c_int & 0x20 as libc::c_int != 0 {
                    let mut sc: *mut sudo_ev_siginfo_container =
                        (*ev).closure as *mut sudo_ev_siginfo_container;
                    if (*(*base).siginfo[i as usize]).si_signo == 0 as libc::c_int {
                        (*sc).siginfo = 0 as *mut siginfo_t;
                    } else {
                        (*sc).siginfo = ((*sc).si_buf).as_mut_ptr() as *mut siginfo_t;
                        memcpy(
                            (*sc).siginfo as *mut libc::c_void,
                            (*base).siginfo[i as usize] as *const libc::c_void,
                            ::core::mem::size_of::<siginfo_t>() as libc::c_ulong,
                        );
                    }
                }
                (*ev).revents = ((*ev).events as libc::c_int
                    & (0x10 as libc::c_int | 0x20 as libc::c_int))
                    as libc::c_short;
                (*ev).active_entries.tqe_next = 0 as *mut sudo_event;
                (*ev).active_entries.tqe_prev = (*base).active.tqh_last;
                *(*base).active.tqh_last = ev;
                (*base).active.tqh_last = &mut (*ev).active_entries.tqe_next;
                (*ev).flags = ((*ev).flags as libc::c_int | 0x2 as libc::c_int) as libc::c_short;
                ev = (*ev).entries.tqe_next;
            }
        }
        i += 1;
    }
    sigprocmask(2 as libc::c_int, &mut oset, 0 as *mut sigset_t);
    sudo_debug_exit_v1(
        (*::core::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
            b"sudo_ev_activate_sigevents\0",
        ))
        .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        139 as libc::c_int,
        sudo_debug_subsys,
    );
}

unsafe extern "C" fn signal_pipe_cb(
    mut fd: libc::c_int,
    mut _what: libc::c_int,
    mut v: *mut libc::c_void,
){}
