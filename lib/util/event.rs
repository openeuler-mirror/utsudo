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

}
