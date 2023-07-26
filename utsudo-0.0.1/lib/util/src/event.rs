#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}

extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn __errno_location() -> *mut libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn pipe2(__pipedes: *mut libc::c_int, __flags: libc::c_int) -> libc::c_int;
    fn sudo_debug_enter_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        subsys: libc::c_int,
    );
    fn sudo_debug_exit_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        subsys: libc::c_int,
    );
}

pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}

pub type sig_atomic_t = __sig_atomic_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_8,
    pub _timer: C2RustUnnamed_7,
    pub _rt: C2RustUnnamed_6,
    pub _sigchld: C2RustUnnamed_5,
    pub _sigfault: C2RustUnnamed_2,
    pub _sigpoll: C2RustUnnamed_1,
    pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _addr_bnd: C2RustUnnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(libc::c_int) -> ()>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction:
        Option<unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> ()>,
}
pub type sudo_ev_callback_t =
    Option<unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> ()>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_ev_siginfo_container {
    pub closure: *mut libc::c_void,
    pub siginfo: *mut siginfo_t,
    pub si_buf: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_event {
    pub entries: C2RustUnnamed_12,
    pub active_entries: C2RustUnnamed_11,
    pub timeouts_entries: C2RustUnnamed_10,
    pub base: *mut sudo_event_base,
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
    pub flags: libc::c_short,
    pub pfd_idx: libc::c_short,
    pub callback: sudo_ev_callback_t,
    pub timeout: timespec,
    pub closure: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_event_base {
    pub events: sudo_event_list,
    pub active: sudo_event_list,
    pub timeouts: sudo_event_list,
    pub signal_event: sudo_event,
    pub signals: [sudo_event_list; 65],
    pub orig_handlers: [*mut sigaction; 65],
    pub siginfo: [*mut siginfo_t; 65],
    pub signal_pending: [sig_atomic_t; 65],
    pub signal_caught: sig_atomic_t,
    pub num_handlers: libc::c_int,
    pub signal_pipe: [libc::c_int; 2],
    pub pfds: *mut pollfd,
    pub pfd_max: libc::c_int,
    pub pfd_high: libc::c_int,
    pub pfd_free: libc::c_int,
    pub flags: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_event_list {
    pub tqh_first: *mut sudo_event,
    pub tqh_last: *mut *mut sudo_event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub tqe_next: *mut sudo_event,
    pub tqe_prev: *mut *mut sudo_event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub tqe_next: *mut sudo_event,
    pub tqe_prev: *mut *mut sudo_event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub tqe_next: *mut sudo_event,
    pub tqe_prev: *mut *mut sudo_event,
}
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
){
    let mut base: *mut sudo_event_base = v as *mut sudo_event_base;
    let mut ch: libc::c_uchar = 0;
    let mut nread: ssize_t = 0;
    let sudo_debug_subsys: libc::c_int = (4 as libc::c_int) << 6 as libc::c_int;
    sudo_debug_enter_v1(
        (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"signal_pipe_cb\0")).as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        151 as libc::c_int,
        sudo_debug_subsys,
    );

        loop {
        nread = read(
            fd,
            &mut ch as *mut libc::c_uchar as *mut libc::c_void,
            1 as libc::c_int as size_t,
        );
        if !(nread > 0 as libc::c_int as libc::c_long) {
            break;
        }
        sudo_debug_printf2_v1(
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"signal_pipe_cb\0"))
                .as_ptr(),
            b"event.c\0" as *const u8 as *const libc::c_char,
            160 as libc::c_int,
            6 as libc::c_int | sudo_debug_subsys,
            b"%s: received signal %d\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"signal_pipe_cb\0"))
                .as_ptr(),
            ch as libc::c_int,
        );
    }

        if nread == -(1 as libc::c_int) as libc::c_long && *__errno_location() != 11 as libc::c_int {
        sudo_debug_printf2_v1(
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"signal_pipe_cb\0"))
                .as_ptr(),
            b"event.c\0" as *const u8 as *const libc::c_char,
            164 as libc::c_int,
            2 as libc::c_int
                | (1 as libc::c_int) << 5 as libc::c_int
                | (1 as libc::c_int) << 4 as libc::c_int
                | sudo_debug_subsys,
            b"%s: error reading from signal pipe fd %d\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"signal_pipe_cb\0"))
                .as_ptr(),
            fd,
        );
    }
    sudo_ev_activate_sigevents(base);
    sudo_debug_exit_v1(
        (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"signal_pipe_cb\0")).as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        170 as libc::c_int,
        sudo_debug_subsys,
    );

}

unsafe extern "C" fn sudo_ev_base_init(mut base: *mut sudo_event_base) -> libc::c_int{
    let mut i: libc::c_int = 0;
    let sudo_debug_subsys: libc::c_int = (4 as libc::c_int) << 6 as libc::c_int;
    sudo_debug_enter_v1(
        (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"sudo_ev_base_init\0"))
            .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        177 as libc::c_int,
        sudo_debug_subsys,
    );
    (*base).events.tqh_first = 0 as *mut sudo_event;
    (*base).events.tqh_last = &mut (*base).events.tqh_first;
    (*base).timeouts.tqh_first = 0 as *mut sudo_event;
    (*base).timeouts.tqh_last = &mut (*base).timeouts.tqh_first;
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int + 1 as libc::c_int {
        (*base).signals[i as usize].tqh_first = 0 as *mut sudo_event;
        (*base).signals[i as usize].tqh_last =
            &mut (*((*base).signals).as_mut_ptr().offset(i as isize)).tqh_first;
        i += 1;
    }
        if sudo_ev_base_alloc_impl(base) != 0 as libc::c_int {
        sudo_debug_printf2_v1(
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"sudo_ev_base_init\0"))
                .as_ptr(),
            b"event.c\0" as *const u8 as *const libc::c_char,
            185 as libc::c_int,
            2 as libc::c_int | sudo_debug_subsys,
            b"%s: unable to allocate impl base\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"sudo_ev_base_init\0"))
                .as_ptr(),
        );
    } else if pipe2(
        ((*base).signal_pipe).as_mut_ptr(),
        0o4000 as libc::c_int | 0o2000000 as libc::c_int,
    ) != 0 as libc::c_int
    {
        sudo_debug_printf2_v1(
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"sudo_ev_base_init\0"))
                .as_ptr(),
            b"event.c\0" as *const u8 as *const libc::c_char,
            190 as libc::c_int,
            2 as libc::c_int | sudo_debug_subsys,
            b"%s: unable to create signal pipe\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"sudo_ev_base_init\0"))
                .as_ptr(),
        );
    } else {
        sudo_ev_init(
            &mut (*base).signal_event,
            (*base).signal_pipe[0 as libc::c_int as usize],
            (0x2 as libc::c_int | 0x8 as libc::c_int) as libc::c_short,
            Some(
                signal_pipe_cb
                    as unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> (),
            ),
            base as *mut libc::c_void,
        );
        let mut sudo_debug_ret: libc::c_int = 0 as libc::c_int;
        sudo_debug_exit_int_v1(
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"sudo_ev_base_init\0"))
                .as_ptr(),
            b"event.c\0" as *const u8 as *const libc::c_char,
            196 as libc::c_int,
            sudo_debug_subsys,
            sudo_debug_ret,
        );
        return sudo_debug_ret;
    }
        sudo_ev_base_free_impl(base);
    let mut sudo_debug_ret_0: libc::c_int = -(1 as libc::c_int);
    sudo_debug_exit_int_v1(
        (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"sudo_ev_base_init\0"))
            .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        200 as libc::c_int,
        sudo_debug_subsys,
        sudo_debug_ret_0,
    );
    return sudo_debug_ret_0;
}

#[no_mangle]
pub unsafe extern "C" fn sudo_ev_base_alloc_v1() -> *mut sudo_event_base {
    let mut base: *mut sudo_event_base = 0 as *mut sudo_event_base;
    let sudo_debug_subsys: libc::c_int = (4 as libc::c_int) << 6 as libc::c_int;
    sudo_debug_enter_v1(
        (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"sudo_ev_base_alloc_v1\0"))
            .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        207 as libc::c_int,
        sudo_debug_subsys,
    );
        base = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<sudo_event_base>() as libc::c_ulong,
    ) as *mut sudo_event_base;
    if base.is_null() {
        sudo_debug_printf2_v1(
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"sudo_ev_base_alloc_v1\0"))
                .as_ptr(),
            b"event.c\0" as *const u8 as *const libc::c_char,
            212 as libc::c_int,
            2 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int | sudo_debug_subsys,
            b"%s: unable to allocate base\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"sudo_ev_base_alloc_v1\0"))
                .as_ptr(),
        );
        let mut sudo_debug_ret: *mut libc::c_void = 0 as *mut libc::c_void;
        sudo_debug_exit_ptr_v1(
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"sudo_ev_base_alloc_v1\0"))
                .as_ptr(),
            b"event.c\0" as *const u8 as *const libc::c_char,
            213 as libc::c_int,
            sudo_debug_subsys,
            sudo_debug_ret,
        );
        return sudo_debug_ret as *mut sudo_event_base;
    }

    if sudo_ev_base_init(base) != 0 as libc::c_int {
        free(base as *mut libc::c_void);
        let mut sudo_debug_ret_0: *mut libc::c_void = 0 as *mut libc::c_void;
        sudo_debug_exit_ptr_v1(
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"sudo_ev_base_alloc_v1\0"))
                .as_ptr(),
            b"event.c\0" as *const u8 as *const libc::c_char,
            217 as libc::c_int,
            sudo_debug_subsys,
            sudo_debug_ret_0,
        );
        return sudo_debug_ret_0 as *mut sudo_event_base;
    }
        let mut sudo_debug_ret_1: *mut libc::c_void = base as *mut libc::c_void;
    sudo_debug_exit_ptr_v1(
        (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"sudo_ev_base_alloc_v1\0"))
            .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        219 as libc::c_int,
        sudo_debug_subsys,
        sudo_debug_ret_1,
    );
    return sudo_debug_ret_1 as *mut sudo_event_base;
}

#[no_mangle]
pub unsafe extern "C" fn sudo_ev_base_free_v1(mut base: *mut sudo_event_base) {
    let mut ev: *mut sudo_event = 0 as *mut sudo_event;
    let mut next: *mut sudo_event = 0 as *mut sudo_event;
    let mut i: libc::c_int = 0;
    let sudo_debug_subsys: libc::c_int = (4 as libc::c_int) << 6 as libc::c_int;
    sudo_debug_enter_v1(
        (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"sudo_ev_base_free_v1\0"))
            .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        227 as libc::c_int,
        sudo_debug_subsys,
    );
    if base.is_null() {
        sudo_debug_exit_v1(
            (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"sudo_ev_base_free_v1\0"))
                .as_ptr(),
            b"event.c\0" as *const u8 as *const libc::c_char,
            230 as libc::c_int,
            sudo_debug_subsys,
        );
        return;
    }
    if default_base == base {
        default_base = 0 as *mut sudo_event_base;
    }
    ev = (*base).events.tqh_first;
    while !ev.is_null() && {
        next = (*ev).entries.tqe_next;
        1 as libc::c_int != 0
    } {
        sudo_ev_del_v1(base, ev);
        ev = next;
    }
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int + 1 as libc::c_int {
        ev = (*base).signals[i as usize].tqh_first;
        while !ev.is_null() && {
            next = (*ev).entries.tqe_next;
            1 as libc::c_int != 0
        } {
            sudo_ev_del_v1(base, ev);
            ev = next;
        }
        free((*base).siginfo[i as usize] as *mut libc::c_void);
        free((*base).orig_handlers[i as usize] as *mut libc::c_void);
        i += 1;
    }
    sudo_ev_base_free_impl(base);
    close((*base).signal_pipe[0 as libc::c_int as usize]);
    close((*base).signal_pipe[1 as libc::c_int as usize]);
    free(base as *mut libc::c_void);
    sudo_debug_exit_v1(
        (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"sudo_ev_base_free_v1\0"))
            .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        252 as libc::c_int,
        sudo_debug_subsys,
    );
}

#[no_mangle]
pub unsafe extern "C" fn sudo_ev_base_setdef_v1(mut base: *mut sudo_event_base){
    let sudo_debug_subsys: libc::c_int = (4 as libc::c_int) << 6 as libc::c_int;
    sudo_debug_enter_v1(
        (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(b"sudo_ev_base_setdef_v1\0"))
            .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        258 as libc::c_int,
        sudo_debug_subsys,
    );
    default_base = base;
    sudo_debug_exit_v1(
        (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(b"sudo_ev_base_setdef_v1\0"))
            .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        262 as libc::c_int,
        sudo_debug_subsys,
    );
}

unsafe extern "C" fn sudo_ev_init(
    mut ev: *mut sudo_event,
    mut fd: libc::c_int,
    mut events: libc::c_short,
    mut callback: sudo_ev_callback_t,
    mut closure: *mut libc::c_void,
) {
    let sudo_debug_subsys: libc::c_int = (4 as libc::c_int) << 6 as libc::c_int;
    sudo_debug_enter_v1(
        (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"sudo_ev_init\0")).as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        272 as libc::c_int,
        sudo_debug_subsys,
    );
    memset(
        ev as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<sudo_event>() as libc::c_ulong,
    );
    (*ev).fd = fd;
    (*ev).events = events;
    (*ev).pfd_idx = -(1 as libc::c_int) as libc::c_short;
    (*ev).callback = callback;
    (*ev).closure = closure;
    sudo_debug_exit_v1(
        (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"sudo_ev_init\0")).as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        282 as libc::c_int,
        sudo_debug_subsys,
    );
}

#[no_mangle]
pub unsafe extern "C" fn sudo_ev_alloc_v1(
    mut fd: libc::c_int,
    mut events: libc::c_short,
    mut callback: sudo_ev_callback_t,
    mut closure: *mut libc::c_void,
) -> *mut sudo_event {
    let mut ev: *mut sudo_event = 0 as *mut sudo_event;
    let sudo_debug_subsys: libc::c_int = (4 as libc::c_int) << 6 as libc::c_int;
    sudo_debug_enter_v1(
        (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"sudo_ev_alloc_v1\0")).as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        289 as libc::c_int,
        sudo_debug_subsys,
    );
    ev = malloc(::core::mem::size_of::<sudo_event>() as libc::c_ulong) as *mut sudo_event;
    if ev.is_null() {
        sudo_debug_printf2_v1(
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"sudo_ev_alloc_v1\0"))
                .as_ptr(),
            b"event.c\0" as *const u8 as *const libc::c_char,
            294 as libc::c_int,
            2 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int | sudo_debug_subsys,
            b"%s: unable to allocate event\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"sudo_ev_alloc_v1\0"))
                .as_ptr(),
        );
        let mut sudo_debug_ret: *mut libc::c_void = 0 as *mut libc::c_void;
        sudo_debug_exit_ptr_v1(
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"sudo_ev_alloc_v1\0"))
                .as_ptr(),
            b"event.c\0" as *const u8 as *const libc::c_char,
            295 as libc::c_int,
            sudo_debug_subsys,
            sudo_debug_ret,
        );
        return sudo_debug_ret as *mut sudo_event;
    }
    if events as libc::c_int & 0x20 as libc::c_int != 0 {
        let mut container: *mut sudo_ev_siginfo_container = malloc(
            (::core::mem::size_of::<sudo_ev_siginfo_container>() as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<siginfo_t>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        )
            as *mut sudo_ev_siginfo_container;
        if container.is_null() {
            sudo_debug_printf2_v1(
                (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"sudo_ev_alloc_v1\0"))
                    .as_ptr(),
                b"event.c\0" as *const u8 as *const libc::c_char,
                303 as libc::c_int,
                2 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int | sudo_debug_subsys,
                b"%s: unable to allocate siginfo container\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"sudo_ev_alloc_v1\0"))
                    .as_ptr(),
            );
            free(ev as *mut libc::c_void);
            let mut sudo_debug_ret_0: *mut libc::c_void = 0 as *mut libc::c_void;
            sudo_debug_exit_ptr_v1(
                (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"sudo_ev_alloc_v1\0"))
                    .as_ptr(),
                b"event.c\0" as *const u8 as *const libc::c_char,
                305 as libc::c_int,
                sudo_debug_subsys,
                sudo_debug_ret_0,
            );
            return sudo_debug_ret_0 as *mut sudo_event;
        }
        (*container).closure = closure;
        closure = container as *mut libc::c_void;
    }
    sudo_ev_init(ev, fd, events, callback, closure);
    let mut sudo_debug_ret_1: *mut libc::c_void = ev as *mut libc::c_void;
    sudo_debug_exit_ptr_v1(
        (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"sudo_ev_alloc_v1\0")).as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        312 as libc::c_int,
        sudo_debug_subsys,
        sudo_debug_ret_1,
    );
    return sudo_debug_ret_1 as *mut sudo_event;
}

#[no_mangle]
pub unsafe extern "C" fn sudo_ev_alloc_v1(
    mut fd: libc::c_int,
    mut events: libc::c_short,
    mut callback: sudo_ev_callback_t,
    mut closure: *mut libc::c_void,
) -> *mut sudo_event {
    let mut ev: *mut sudo_event = 0 as *mut sudo_event;
    let sudo_debug_subsys: libc::c_int = (4 as libc::c_int) << 6 as libc::c_int;
    sudo_debug_enter_v1(
        (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"sudo_ev_alloc_v1\0")).as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        289 as libc::c_int,
        sudo_debug_subsys,
    );
    ev = malloc(::core::mem::size_of::<sudo_event>() as libc::c_ulong) as *mut sudo_event;
    if ev.is_null() {
        sudo_debug_printf2_v1(
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"sudo_ev_alloc_v1\0"))
                .as_ptr(),
            b"event.c\0" as *const u8 as *const libc::c_char,
            294 as libc::c_int,
            2 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int | sudo_debug_subsys,
            b"%s: unable to allocate event\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"sudo_ev_alloc_v1\0"))
                .as_ptr(),
        );
        let mut sudo_debug_ret: *mut libc::c_void = 0 as *mut libc::c_void;
        sudo_debug_exit_ptr_v1(
            (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"sudo_ev_alloc_v1\0"))
                .as_ptr(),
            b"event.c\0" as *const u8 as *const libc::c_char,
            295 as libc::c_int,
            sudo_debug_subsys,
            sudo_debug_ret,
        );
        return sudo_debug_ret as *mut sudo_event;
    }
    if events as libc::c_int & 0x20 as libc::c_int != 0 {
        let mut container: *mut sudo_ev_siginfo_container = malloc(
            (::core::mem::size_of::<sudo_ev_siginfo_container>() as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<siginfo_t>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        )
            as *mut sudo_ev_siginfo_container;
        if container.is_null() {
            sudo_debug_printf2_v1(
                (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"sudo_ev_alloc_v1\0"))
                    .as_ptr(),
                b"event.c\0" as *const u8 as *const libc::c_char,
                303 as libc::c_int,
                2 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int | sudo_debug_subsys,
                b"%s: unable to allocate siginfo container\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"sudo_ev_alloc_v1\0"))
                    .as_ptr(),
            );
            free(ev as *mut libc::c_void);
            let mut sudo_debug_ret_0: *mut libc::c_void = 0 as *mut libc::c_void;
            sudo_debug_exit_ptr_v1(
                (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"sudo_ev_alloc_v1\0"))
                    .as_ptr(),
                b"event.c\0" as *const u8 as *const libc::c_char,
                305 as libc::c_int,
                sudo_debug_subsys,
                sudo_debug_ret_0,
            );
            return sudo_debug_ret_0 as *mut sudo_event;
        }
        (*container).closure = closure;
        closure = container as *mut libc::c_void;
    }
    sudo_ev_init(ev, fd, events, callback, closure);
    let mut sudo_debug_ret_1: *mut libc::c_void = ev as *mut libc::c_void;
    sudo_debug_exit_ptr_v1(
        (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"sudo_ev_alloc_v1\0")).as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        312 as libc::c_int,
        sudo_debug_subsys,
        sudo_debug_ret_1,
    );
    return sudo_debug_ret_1 as *mut sudo_event;
}
#[no_mangle]
pub unsafe extern "C" fn sudo_ev_free_v1(mut ev: *mut sudo_event) {
    let sudo_debug_subsys: libc::c_int = (4 as libc::c_int) << 6 as libc::c_int;
    sudo_debug_enter_v1(
        (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"sudo_ev_free_v1\0")).as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        318 as libc::c_int,
        sudo_debug_subsys,
    );
    if ev.is_null() {
        sudo_debug_exit_v1(
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"sudo_ev_free_v1\0"))
                .as_ptr(),
            b"event.c\0" as *const u8 as *const libc::c_char,
            321 as libc::c_int,
            sudo_debug_subsys,
        );
        return;
    }
    if (*ev).flags as libc::c_int & 0x1 as libc::c_int != 0 {
        sudo_ev_del_v1(0 as *mut sudo_event_base, ev);
    }
    if (*ev).events as libc::c_int & 0x20 as libc::c_int != 0 {
        free((*ev).closure);
    }
    free(ev as *mut libc::c_void);
    sudo_debug_exit_v1(
        (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"sudo_ev_free_v1\0")).as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        330 as libc::c_int,
        sudo_debug_subsys,
    );
}
unsafe extern "C" fn sudo_ev_handler(
    mut signo: libc::c_int,
    mut info: *mut siginfo_t,
    mut _context: *mut libc::c_void,
) {
    let mut ch: libc::c_uchar = signo as libc::c_uchar;
    if !signal_base.is_null() {
        if info.is_null() {
            memset(
                (*signal_base).siginfo[signo as usize] as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<siginfo_t>() as libc::c_ulong,
            );
        } else {
            memcpy(
                (*signal_base).siginfo[signo as usize] as *mut libc::c_void,
                info as *const libc::c_void,
                ::core::mem::size_of::<siginfo_t>() as libc::c_ulong,
            );
        }
        (*signal_base).signal_pending[signo as usize] = 1 as libc::c_int;
        (*signal_base).signal_caught = 1 as libc::c_int;
        let mut _y: ssize_t = write(
            (*signal_base).signal_pipe[1 as libc::c_int as usize],
            &mut ch as *mut libc::c_uchar as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
    }
}
unsafe extern "C" fn sudo_ev_add_signal(
    mut base: *mut sudo_event_base,
    mut ev: *mut sudo_event,
    mut tohead: bool,
) -> libc::c_int {
    let signo: libc::c_int = (*ev).fd;
    sudo_debug_enter_v1(
        (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"sudo_ev_add_signal\0"))
            .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        361 as libc::c_int,
        sudo_debug_subsys,
    );
    sudo_debug_printf2_v1(
        (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"sudo_ev_add_signal\0"))
            .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        365 as libc::c_int,
        6 as libc::c_int | sudo_debug_subsys,
        b"%s: adding event %p to base %p, signal %d, events %d\0" as *const u8
            as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"sudo_ev_add_signal\0"))
            .as_ptr(),
        ev,
        base,
        signo,
        (*ev).events as libc::c_int,
    );
    if signo >= 64 as libc::c_int + 1 as libc::c_int {
        sudo_debug_printf2_v1(
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"sudo_ev_add_signal\0"))
                .as_ptr(),
            b"event.c\0" as *const u8 as *const libc::c_char,
            368 as libc::c_int,
            2 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int | sudo_debug_subsys,
            b"%s: signo %d larger than max %d\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"sudo_ev_add_signal\0"))
                .as_ptr(),
            signo,
            64 as libc::c_int + 1 as libc::c_int - 1 as libc::c_int,
        );
        let mut sudo_debug_ret: libc::c_int = -(1 as libc::c_int);
        sudo_debug_exit_int_v1(
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"sudo_ev_add_signal\0"))
                .as_ptr(),
            b"event.c\0" as *const u8 as *const libc::c_char,
            369 as libc::c_int,
            sudo_debug_subsys,
            sudo_debug_ret,
        );
        return sudo_debug_ret;
    }
    if (*ev).events as libc::c_int
        & !(0x10 as libc::c_int | 0x20 as libc::c_int | 0x8 as libc::c_int)
        != 0 as libc::c_int
    {
        sudo_debug_printf2_v1(
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"sudo_ev_add_signal\0"))
                .as_ptr(),
            b"event.c\0" as *const u8 as *const libc::c_char,
            373 as libc::c_int,
            2 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int | sudo_debug_subsys,
            b"%s: invalid event set 0x%x\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"sudo_ev_add_signal\0"))
                .as_ptr(),
            (*ev).events as libc::c_int,
        );
        let mut sudo_debug_ret_0: libc::c_int = -(1 as libc::c_int);
        sudo_debug_exit_int_v1(
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"sudo_ev_add_signal\0"))
                .as_ptr(),
            b"event.c\0" as *const u8 as *const libc::c_char,
            374 as libc::c_int,
            sudo_debug_subsys,
            sudo_debug_ret_0,
        );
        return sudo_debug_ret_0;
    }
    if ((*base).siginfo[signo as usize]).is_null() {
        (*base).siginfo[signo as usize] =
            malloc(::core::mem::size_of::<siginfo_t>() as libc::c_ulong) as *mut siginfo_t;
        if ((*base).siginfo[signo as usize]).is_null() {
            sudo_debug_printf2_v1(
                (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                    b"sudo_ev_add_signal\0",
                ))
                .as_ptr(),
                b"event.c\0" as *const u8 as *const libc::c_char,
                385 as libc::c_int,
                2 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int | sudo_debug_subsys,
                b"%s: unable to allocate siginfo for signo %d\0" as *const u8
                    as *const libc::c_char,
                (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                    b"sudo_ev_add_signal\0",
                ))
                .as_ptr(),
                signo,
            );
            let mut sudo_debug_ret_1: libc::c_int = -(1 as libc::c_int);
            sudo_debug_exit_int_v1(
                (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                    b"sudo_ev_add_signal\0",
                ))
                .as_ptr(),
                b"event.c\0" as *const u8 as *const libc::c_char,
                386 as libc::c_int,
                sudo_debug_subsys,
                sudo_debug_ret_1,
            );
            return sudo_debug_ret_1;
        }
    }
    if ((*base).orig_handlers[signo as usize]).is_null() {
        (*base).orig_handlers[signo as usize] =
            malloc(::core::mem::size_of::<sigaction>() as libc::c_ulong) as *mut sigaction;
        if ((*base).orig_handlers[signo as usize]).is_null() {
            sudo_debug_printf2_v1(
                (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                    b"sudo_ev_add_signal\0",
                ))
                .as_ptr(),
                b"event.c\0" as *const u8 as *const libc::c_char,
                395 as libc::c_int,
                2 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int | sudo_debug_subsys,
                b"%s: unable to allocate orig_handlers for signo %d\0" as *const u8
                    as *const libc::c_char,
                (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                    b"sudo_ev_add_signal\0",
                ))
                .as_ptr(),
                signo,
            );
            let mut sudo_debug_ret_2: libc::c_int = -(1 as libc::c_int);
            sudo_debug_exit_int_v1(
                (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                    b"sudo_ev_add_signal\0",
                ))
                .as_ptr(),
                b"event.c\0" as *const u8 as *const libc::c_char,
                396 as libc::c_int,
                sudo_debug_subsys,
                sudo_debug_ret_2,
            );
            return sudo_debug_ret_2;
        }
    }
    if ((*base).signals[signo as usize].tqh_first).is_null() {
        let mut sa: sigaction = sigaction {
            __sigaction_handler: C2RustUnnamed_9 { sa_handler: None },
            sa_mask: sigset_t { __val: [0; 16] },
            sa_flags: 0,
            sa_restorer: None,
        };
        memset(
            &mut sa as *mut sigaction as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<sigaction>() as libc::c_ulong,
        );
        sigfillset(&mut sa.sa_mask);
        sa.sa_flags = 0x10000000 as libc::c_int | 4 as libc::c_int;
        sa.__sigaction_handler.sa_sigaction = Some(
            sudo_ev_handler
                as unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
        );
        if sigaction(signo, &mut sa, (*base).orig_handlers[signo as usize]) != 0 as libc::c_int {
            sudo_debug_printf2_v1(
                (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                    b"sudo_ev_add_signal\0",
                ))
                .as_ptr(),
                b"event.c\0" as *const u8 as *const libc::c_char,
                409 as libc::c_int,
                2 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int | sudo_debug_subsys,
                b"%s: unable to install handler for signo %d\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                    b"sudo_ev_add_signal\0",
                ))
                .as_ptr(),
                signo,
            );
            let mut sudo_debug_ret_3: libc::c_int = -(1 as libc::c_int);
            sudo_debug_exit_int_v1(
                (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                    b"sudo_ev_add_signal\0",
                ))
                .as_ptr(),
                b"event.c\0" as *const u8 as *const libc::c_char,
                410 as libc::c_int,
                sudo_debug_subsys,
                sudo_debug_ret_3,
            );
            return sudo_debug_ret_3;
        }
        (*base).num_handlers += 1;
    }
    (*ev).base = base;
    if tohead {
        (*ev).entries.tqe_next = (*base).signals[signo as usize].tqh_first;
        if !((*ev).entries.tqe_next).is_null() {
            (*(*base).signals[signo as usize].tqh_first)
                .entries
                .tqe_prev = &mut (*ev).entries.tqe_next;
        } else {
            (*base).signals[signo as usize].tqh_last = &mut (*ev).entries.tqe_next;
        }
        (*base).signals[signo as usize].tqh_first = ev;
        (*ev).entries.tqe_prev =
            &mut (*((*base).signals).as_mut_ptr().offset(signo as isize)).tqh_first;
    } else {
        (*ev).entries.tqe_next = 0 as *mut sudo_event;
        (*ev).entries.tqe_prev = (*base).signals[signo as usize].tqh_last;
        *(*base).signals[signo as usize].tqh_last = ev;
        (*base).signals[signo as usize].tqh_last = &mut (*ev).entries.tqe_next;
    }
    (*ev).events = ((*ev).events as libc::c_int | 0x8 as libc::c_int) as libc::c_short;
    (*ev).flags = ((*ev).flags as libc::c_int | 0x1 as libc::c_int) as libc::c_short;
    if (*base).signal_event.flags as libc::c_int & 0x1 as libc::c_int == 0 {
        sudo_ev_add_v2(
            base,
            &mut (*base).signal_event,
            0 as *mut timespec,
            1 as libc::c_int != 0,
        );
    }
    signal_base = base;
    let mut sudo_debug_ret_4: libc::c_int = 0 as libc::c_int;
    sudo_debug_exit_int_v1(
        (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"sudo_ev_add_signal\0"))
            .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        435 as libc::c_int,
        sudo_debug_subsys,
        sudo_debug_ret_4,
    );
    return sudo_debug_ret_4;
}

#[no_mangle]
pub unsafe extern "C" fn sudo_ev_add_v1(
    mut base: *mut sudo_event_base,
    mut ev: *mut sudo_event,
    mut timo: *mut timeval,
    mut tohead: bool,
) -> libc::c_int {
    let mut tsbuf: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut ts: *mut timespec = 0 as *mut timespec;
    if !timo.is_null() {
        tsbuf.tv_sec = (*timo).tv_sec;
        tsbuf.tv_nsec = (*timo).tv_usec * 1000 as libc::c_int as libc::c_long;
        ts = &mut tsbuf;
    }
    return sudo_ev_add_v2(base, ev, ts, tohead);
}

#[no_mangle]
pub unsafe extern "C" fn sudo_ev_add_v2(
    mut base: *mut sudo_event_base,
    mut ev: *mut sudo_event,
    mut timo: *mut timespec,
    mut tohead: bool,
) -> libc::c_int {
    let sudo_debug_subsys: libc::c_int = (4 as libc::c_int) << 6 as libc::c_int;
    sudo_debug_enter_v1(
        (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"sudo_ev_add_v2\0")).as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        456 as libc::c_int,
        sudo_debug_subsys,
    );
    if base.is_null() {
        if !((*ev).base).is_null() {
            base = (*ev).base;
        } else if !default_base.is_null() {
            base = default_base;
        } else {
            sudo_debug_printf2_v1(
                (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"sudo_ev_add_v2\0"))
                    .as_ptr(),
                b"event.c\0" as *const u8 as *const libc::c_char,
                466 as libc::c_int,
                2 as libc::c_int | sudo_debug_subsys,
                b"%s: no base specified\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"sudo_ev_add_v2\0"))
                    .as_ptr(),
            );
            let mut sudo_debug_ret: libc::c_int = -(1 as libc::c_int);
            sudo_debug_exit_int_v1(
                (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"sudo_ev_add_v2\0"))
                    .as_ptr(),
                b"event.c\0" as *const u8 as *const libc::c_char,
                467 as libc::c_int,
                sudo_debug_subsys,
                sudo_debug_ret,
            );
            return sudo_debug_ret;
        }
    }
    if (*ev).flags as libc::c_int & 0x1 as libc::c_int != 0 {
        if timo.is_null() && (*ev).flags as libc::c_int & 0x4 as libc::c_int != 0 {
            sudo_debug_printf2_v1(
                (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"sudo_ev_add_v2\0"))
                    .as_ptr(),
                b"event.c\0" as *const u8 as *const libc::c_char,
                476 as libc::c_int,
                6 as libc::c_int | sudo_debug_subsys,
                b"%s: removing event %p from timeouts queue\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"sudo_ev_add_v2\0"))
                    .as_ptr(),
                ev,
            );
            (*ev).flags = ((*ev).flags as libc::c_int & !(0x4 as libc::c_int)) as libc::c_short;
            if !((*ev).timeouts_entries.tqe_next).is_null() {
                (*(*ev).timeouts_entries.tqe_next).timeouts_entries.tqe_prev =
                    (*ev).timeouts_entries.tqe_prev;
            } else {
                (*base).timeouts.tqh_last = (*ev).timeouts_entries.tqe_prev;
            }
            *(*ev).timeouts_entries.tqe_prev = (*ev).timeouts_entries.tqe_next;
        }
    } else {
        if (*ev).events as libc::c_int & (0x10 as libc::c_int | 0x20 as libc::c_int) != 0 {
            let mut sudo_debug_ret_0: libc::c_int = sudo_ev_add_signal(base, ev, tohead);
            sudo_debug_exit_int_v1(
                (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"sudo_ev_add_v2\0"))
                    .as_ptr(),
                b"event.c\0" as *const u8 as *const libc::c_char,
                483 as libc::c_int,
                sudo_debug_subsys,
                sudo_debug_ret_0,
            );
            return sudo_debug_ret_0;
        }
        sudo_debug_printf2_v1(
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"sudo_ev_add_v2\0"))
                .as_ptr(),
            b"event.c\0" as *const u8 as *const libc::c_char,
            488 as libc::c_int,
            6 as libc::c_int | sudo_debug_subsys,
            b"%s: adding event %p to base %p, fd %d, events %d\0" as *const u8
                as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"sudo_ev_add_v2\0"))
                .as_ptr(),
            ev,
            base,
            (*ev).fd,
            (*ev).events as libc::c_int,
        );
        if (*ev).events as libc::c_int & (0x2 as libc::c_int | 0x4 as libc::c_int) != 0 {
            if sudo_ev_add_impl(base, ev) != 0 as libc::c_int {
                let mut sudo_debug_ret_1: libc::c_int = -(1 as libc::c_int);
                sudo_debug_exit_int_v1(
                    (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"sudo_ev_add_v2\0",
                    ))
                    .as_ptr(),
                    b"event.c\0" as *const u8 as *const libc::c_char,
                    491 as libc::c_int,
                    sudo_debug_subsys,
                    sudo_debug_ret_1,
                );
                return sudo_debug_ret_1;
            }
        }
        (*ev).base = base;
        if tohead {
            (*ev).entries.tqe_next = (*base).events.tqh_first;
            if !((*ev).entries.tqe_next).is_null() {
                (*(*base).events.tqh_first).entries.tqe_prev = &mut (*ev).entries.tqe_next;
            } else {
                (*base).events.tqh_last = &mut (*ev).entries.tqe_next;
            }
            (*base).events.tqh_first = ev;
            (*ev).entries.tqe_prev = &mut (*base).events.tqh_first;
        } else {
            (*ev).entries.tqe_next = 0 as *mut sudo_event;
            (*ev).entries.tqe_prev = (*base).events.tqh_last;
            *(*base).events.tqh_last = ev;
            (*base).events.tqh_last = &mut (*ev).entries.tqe_next;
        }
        (*ev).flags = ((*ev).flags as libc::c_int | 0x1 as libc::c_int) as libc::c_short;
    }
    if !timo.is_null() {
        let mut evtmp: *mut sudo_event = 0 as *mut sudo_event;
        if (*ev).flags as libc::c_int & 0x4 as libc::c_int != 0 {
            if !((*ev).timeouts_entries.tqe_next).is_null() {
                (*(*ev).timeouts_entries.tqe_next).timeouts_entries.tqe_prev =
                    (*ev).timeouts_entries.tqe_prev;
            } else {
                (*base).timeouts.tqh_last = (*ev).timeouts_entries.tqe_prev;
            }
            *(*ev).timeouts_entries.tqe_prev = (*ev).timeouts_entries.tqe_next;
        }
        sudo_gettime_mono_v1(&mut (*ev).timeout);
        (*ev).timeout.tv_sec = (*ev).timeout.tv_sec + (*timo).tv_sec;
        (*ev).timeout.tv_nsec = (*ev).timeout.tv_nsec + (*timo).tv_nsec;
        while (*ev).timeout.tv_nsec >= 1000000000 as libc::c_int as libc::c_long {
            (*ev).timeout.tv_sec += 1;
            (*ev).timeout.tv_nsec -= 1000000000 as libc::c_int as libc::c_long;
        }
        evtmp = (*base).timeouts.tqh_first;
        while !evtmp.is_null() {
            if if (*ev).timeout.tv_sec == (*evtmp).timeout.tv_sec {
                ((*ev).timeout.tv_nsec < (*evtmp).timeout.tv_nsec) as libc::c_int
            } else {
                ((*ev).timeout.tv_sec < (*evtmp).timeout.tv_sec) as libc::c_int
            } != 0
            {
                break;
            }
            evtmp = (*evtmp).timeouts_entries.tqe_next;
        }
        if !evtmp.is_null() {
            (*ev).timeouts_entries.tqe_prev = (*evtmp).timeouts_entries.tqe_prev;
            (*ev).timeouts_entries.tqe_next = evtmp;
            *(*evtmp).timeouts_entries.tqe_prev = ev;
            (*evtmp).timeouts_entries.tqe_prev = &mut (*ev).timeouts_entries.tqe_next;
        } else {
            (*ev).timeouts_entries.tqe_next = 0 as *mut sudo_event;
            (*ev).timeouts_entries.tqe_prev = (*base).timeouts.tqh_last;
            *(*base).timeouts.tqh_last = ev;
            (*base).timeouts.tqh_last = &mut (*ev).timeouts_entries.tqe_next;
        }
        (*ev).flags = ((*ev).flags as libc::c_int | 0x4 as libc::c_int) as libc::c_short;
    }
    let mut sudo_debug_ret_2: libc::c_int = 0 as libc::c_int;
    sudo_debug_exit_int_v1(
        (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"sudo_ev_add_v2\0")).as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        522 as libc::c_int,
        sudo_debug_subsys,
        sudo_debug_ret_2,
    );
    return sudo_debug_ret_2;
}

#[no_mangle]
pub unsafe extern "C" fn sudo_ev_del_v1(
    mut base: *mut sudo_event_base,
    mut ev: *mut sudo_event,
) -> libc::c_int {
    let sudo_debug_subsys: libc::c_int = (4 as libc::c_int) << 6 as libc::c_int;
    sudo_debug_enter_v1(
        (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"sudo_ev_del_v1\0")).as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        532 as libc::c_int,
        sudo_debug_subsys,
    );
    if (*ev).flags as libc::c_int & 0x1 as libc::c_int == 0 {
        sudo_debug_printf2_v1(
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"sudo_ev_del_v1\0"))
                .as_ptr(),
            b"event.c\0" as *const u8 as *const libc::c_char,
            537 as libc::c_int,
            6 as libc::c_int | sudo_debug_subsys,
            b"%s: event %p not in queue\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"sudo_ev_del_v1\0"))
                .as_ptr(),
            ev,
        );
        let mut sudo_debug_ret: libc::c_int = 0 as libc::c_int;
        sudo_debug_exit_int_v1(
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"sudo_ev_del_v1\0"))
                .as_ptr(),
            b"event.c\0" as *const u8 as *const libc::c_char,
            538 as libc::c_int,
            sudo_debug_subsys,
            sudo_debug_ret,
        );
        return sudo_debug_ret;
    }
    if base.is_null() {
        if ((*ev).base).is_null() {
            sudo_debug_printf2_v1(
                (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"sudo_ev_del_v1\0"))
                    .as_ptr(),
                b"event.c\0" as *const u8 as *const libc::c_char,
                545 as libc::c_int,
                2 as libc::c_int | sudo_debug_subsys,
                b"%s: no base specified\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"sudo_ev_del_v1\0"))
                    .as_ptr(),
            );
            let mut sudo_debug_ret_0: libc::c_int = -(1 as libc::c_int);
            sudo_debug_exit_int_v1(
                (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"sudo_ev_del_v1\0"))
                    .as_ptr(),
                b"event.c\0" as *const u8 as *const libc::c_char,
                546 as libc::c_int,
                sudo_debug_subsys,
                sudo_debug_ret_0,
            );
            return sudo_debug_ret_0;
        }
        base = (*ev).base;
    } else if base != (*ev).base {
        sudo_debug_printf2_v1(
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"sudo_ev_del_v1\0"))
                .as_ptr(),
            b"event.c\0" as *const u8 as *const libc::c_char,
            551 as libc::c_int,
            2 as libc::c_int | sudo_debug_subsys,
            b"%s: mismatch base %p, ev->base %p\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"sudo_ev_del_v1\0"))
                .as_ptr(),
            base,
            (*ev).base,
        );
        let mut sudo_debug_ret_1: libc::c_int = -(1 as libc::c_int);
        sudo_debug_exit_int_v1(
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"sudo_ev_del_v1\0"))
                .as_ptr(),
            b"event.c\0" as *const u8 as *const libc::c_char,
            552 as libc::c_int,
            sudo_debug_subsys,
            sudo_debug_ret_1,
        );
        return sudo_debug_ret_1;
    }
    if (*ev).events as libc::c_int & (0x10 as libc::c_int | 0x20 as libc::c_int) != 0 {
        let signo: libc::c_int = (*ev).fd;
        sudo_debug_printf2_v1(
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"sudo_ev_del_v1\0"))
                .as_ptr(),
            b"event.c\0" as *const u8 as *const libc::c_char,
            560 as libc::c_int,
            6 as libc::c_int | sudo_debug_subsys,
            b"%s: removing event %p from base %p, signo %d, events %d\0" as *const u8
                as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"sudo_ev_del_v1\0"))
                .as_ptr(),
            ev,
            base,
            signo,
            (*ev).events as libc::c_int,
        );
        if !((*ev).entries.tqe_next).is_null() {
            (*(*ev).entries.tqe_next).entries.tqe_prev = (*ev).entries.tqe_prev;
        } else {
            (*base).signals[signo as usize].tqh_last = (*ev).entries.tqe_prev;
        }
        *(*ev).entries.tqe_prev = (*ev).entries.tqe_next;
        if ((*base).signals[signo as usize].tqh_first).is_null() {
            if sigaction(
                signo,
                (*base).orig_handlers[signo as usize],
                0 as *mut sigaction,
            ) != 0 as libc::c_int
            {
                sudo_debug_printf2_v1(
                    (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"sudo_ev_del_v1\0",
                    ))
                    .as_ptr(),
                    b"event.c\0" as *const u8 as *const libc::c_char,
                    568 as libc::c_int,
                    2 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int | sudo_debug_subsys,
                    b"%s: unable to restore handler for signo %d\0" as *const u8
                        as *const libc::c_char,
                    (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"sudo_ev_del_v1\0",
                    ))
                    .as_ptr(),
                    signo,
                );
                let mut sudo_debug_ret_2: libc::c_int = -(1 as libc::c_int);
                sudo_debug_exit_int_v1(
                    (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"sudo_ev_del_v1\0",
                    ))
                    .as_ptr(),
                    b"event.c\0" as *const u8 as *const libc::c_char,
                    569 as libc::c_int,
                    sudo_debug_subsys,
                    sudo_debug_ret_2,
                );
                return sudo_debug_ret_2;
            }
            (*base).num_handlers -= 1;
        }
        if (*base).num_handlers == 0 as libc::c_int {
            sudo_ev_del_v1(base, &mut (*base).signal_event);
        }
    } else {
        sudo_debug_printf2_v1(
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"sudo_ev_del_v1\0"))
                .as_ptr(),
            b"event.c\0" as *const u8 as *const libc::c_char,
            580 as libc::c_int,
            6 as libc::c_int | sudo_debug_subsys,
            b"%s: removing event %p from base %p, fd %d, events %d\0" as *const u8
                as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"sudo_ev_del_v1\0"))
                .as_ptr(),
            ev,
            base,
            (*ev).fd,
            (*ev).events as libc::c_int,
        );
        if (*ev).events as libc::c_int & (0x2 as libc::c_int | 0x4 as libc::c_int) != 0 {
            if sudo_ev_del_impl(base, ev) != 0 as libc::c_int {
                let mut sudo_debug_ret_3: libc::c_int = -(1 as libc::c_int);
                sudo_debug_exit_int_v1(
                    (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"sudo_ev_del_v1\0",
                    ))
                    .as_ptr(),
                    b"event.c\0" as *const u8 as *const libc::c_char,
                    585 as libc::c_int,
                    sudo_debug_subsys,
                    sudo_debug_ret_3,
                );
                return sudo_debug_ret_3;
            }
        }
        if !((*ev).entries.tqe_next).is_null() {
            (*(*ev).entries.tqe_next).entries.tqe_prev = (*ev).entries.tqe_prev;
        } else {
            (*base).events.tqh_last = (*ev).entries.tqe_prev;
        }
        *(*ev).entries.tqe_prev = (*ev).entries.tqe_next;
        if (*ev).flags as libc::c_int & 0x4 as libc::c_int != 0 {
            if !((*ev).timeouts_entries.tqe_next).is_null() {
                (*(*ev).timeouts_entries.tqe_next).timeouts_entries.tqe_prev =
                    (*ev).timeouts_entries.tqe_prev;
            } else {
                (*base).timeouts.tqh_last = (*ev).timeouts_entries.tqe_prev;
            }
            *(*ev).timeouts_entries.tqe_prev = (*ev).timeouts_entries.tqe_next;
        }
    }
    if (*ev).flags as libc::c_int & 0x2 as libc::c_int != 0 {
        if !((*ev).active_entries.tqe_next).is_null() {
            (*(*ev).active_entries.tqe_next).active_entries.tqe_prev =
                (*ev).active_entries.tqe_prev;
        } else {
            (*base).active.tqh_last = (*ev).active_entries.tqe_prev;
        }
        *(*ev).active_entries.tqe_prev = (*ev).active_entries.tqe_next;
    }
    (*ev).flags = 0 as libc::c_int as libc::c_short;
    (*ev).pfd_idx = -(1 as libc::c_int) as libc::c_short;
    let mut sudo_debug_ret_4: libc::c_int = 0 as libc::c_int;
    sudo_debug_exit_int_v1(
        (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"sudo_ev_del_v1\0")).as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        604 as libc::c_int,
        sudo_debug_subsys,
        sudo_debug_ret_4,
    );
    return sudo_debug_ret_4;
}

#[no_mangle]
pub unsafe extern "C" fn sudo_ev_dispatch_v1(mut base: *mut sudo_event_base) -> libc::c_int {
    return sudo_ev_loop_v1(base, 0 as libc::c_int);
}

#[no_mangle]
pub unsafe extern "C" fn sudo_ev_loop_v1(
    mut base: *mut sudo_event_base,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut ev: *mut sudo_event = 0 as *mut sudo_event;
    let mut nready: libc::c_int = 0;
    let mut rc: libc::c_int = 0 as libc::c_int;
    let sudo_debug_subsys: libc::c_int = (4 as libc::c_int) << 6 as libc::c_int;
    sudo_debug_enter_v1(
        (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"sudo_ev_loop_v1\0")).as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        623 as libc::c_int,
        sudo_debug_subsys,
    );
    (*base).flags |= (flags & 0x1 as libc::c_int) as libc::c_uint;
    (*base).flags &= (0x2 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint;
    '_rescan: loop {
        if ((*base).events.tqh_first).is_null() {
            rc = 1 as libc::c_int;
            break;
        } else {
            (*base).active.tqh_first = 0 as *mut sudo_event;
            (*base).active.tqh_last = &mut (*base).active.tqh_first;
            nready = sudo_ev_scan_impl(base, flags);
            match nready {
                -1 => {
                    if *__errno_location() == 12 as libc::c_int {
                        continue;
                    }
                    if *__errno_location() == 4 as libc::c_int {
                        if !((*base).signal_caught != 0) {
                            continue;
                        }
                        signal_pipe_cb(
                            (*base).signal_pipe[0 as libc::c_int as usize],
                            0x2 as libc::c_int,
                            base as *mut libc::c_void,
                        );
                    } else {
                        rc = -(1 as libc::c_int);
                        break;
                    }
                }
                0 => {
                    sudo_gettime_mono_v1(&mut now);
                    loop {
                        ev = (*base).timeouts.tqh_first;
                        if ev.is_null() {
                            break;
                        }
                        if if (*ev).timeout.tv_sec == now.tv_sec {
                            ((*ev).timeout.tv_nsec > now.tv_nsec) as libc::c_int
                        } else {
                            ((*ev).timeout.tv_sec > now.tv_sec) as libc::c_int
                        } != 0
                        {
                            break;
                        }
                        (*ev).flags =
                            ((*ev).flags as libc::c_int & !(0x4 as libc::c_int)) as libc::c_short;
                        if !((*ev).timeouts_entries.tqe_next).is_null() {
                            (*(*ev).timeouts_entries.tqe_next).timeouts_entries.tqe_prev =
                                (*ev).timeouts_entries.tqe_prev;
                        } else {
                            (*base).timeouts.tqh_last = (*ev).timeouts_entries.tqe_prev;
                        }
                        *(*ev).timeouts_entries.tqe_prev = (*ev).timeouts_entries.tqe_next;
                        (*ev).revents = 0x1 as libc::c_int as libc::c_short;
                        (*ev).active_entries.tqe_next = 0 as *mut sudo_event;
                        (*ev).active_entries.tqe_prev = (*base).active.tqh_last;
                        *(*base).active.tqh_last = ev;
                        (*base).active.tqh_last = &mut (*ev).active_entries.tqe_next;
                        (*ev).flags =
                            ((*ev).flags as libc::c_int | 0x2 as libc::c_int) as libc::c_short;
                    }
                    if flags & 0x2 as libc::c_int != 0 {
                        if ((*base).active.tqh_first).is_null() {
                            break;
                        }
                    }
                }
                _ => {}
            }
            loop {
                ev = (*base).active.tqh_first;
                if ev.is_null() {
                    break;
                }
                sudo_ev_deactivate(base, ev);
                if (*ev).events as libc::c_int & 0x8 as libc::c_int == 0 {
                    sudo_ev_del_v1(base, ev);
                }
                ((*ev).callback).expect("non-null function pointer")(
                    (*ev).fd,
                    (*ev).revents as libc::c_int,
                    if (*ev).closure == -(1 as libc::c_int) as *mut libc::c_void {
                        ev as *mut libc::c_void
                    } else {
                        (*ev).closure
                    },
                );
                if (*base).flags & 0x4 as libc::c_int as libc::c_uint != 0 {
                    (*base).flags |= 0x20 as libc::c_int as libc::c_uint;
                    sudo_ev_deactivate_all(base);
                    break '_rescan;
                } else {
                    if !((*base).flags & 0x8 as libc::c_int as libc::c_uint != 0) {
                        continue;
                    }
                    (*base).flags &= !(0x8 as libc::c_int) as libc::c_uint;
                    sudo_ev_deactivate_all(base);
                    continue '_rescan;
                }
            }
            if !((*base).flags & 0x1 as libc::c_int as libc::c_uint != 0) {
                continue;
            }
            if (*base).flags & 0x2 as libc::c_int as libc::c_uint != 0 {
                (*base).flags |= 0x10 as libc::c_int as libc::c_uint;
            }
            sudo_ev_deactivate_all(base);
            break;
        }
    }
    (*base).flags &= 0xf0 as libc::c_int as libc::c_uint;
    let mut sudo_debug_ret: libc::c_int = rc;
    sudo_debug_exit_int_v1(
        (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"sudo_ev_loop_v1\0")).as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        721 as libc::c_int,
        sudo_debug_subsys,
        sudo_debug_ret,
    );
    return sudo_debug_ret;
}

#[no_mangle]
pub unsafe extern "C" fn sudo_ev_loopexit_v1(mut base: *mut sudo_event_base) {
    let sudo_debug_subsys: libc::c_int = (4 as libc::c_int) << 6 as libc::c_int;
    sudo_debug_enter_v1(
        (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"sudo_ev_loopexit_v1\0"))
            .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        727 as libc::c_int,
        sudo_debug_subsys,
    );
    if base.is_null() {
        base = default_base;
        if base.is_null() {
            sudo_debug_exit_v1(
                (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(
                    b"sudo_ev_loopexit_v1\0",
                ))
                .as_ptr(),
                b"event.c\0" as *const u8 as *const libc::c_char,
                731 as libc::c_int,
                sudo_debug_subsys,
            );
            return;
        }
    }
    if (*base).flags & 0x4 as libc::c_int as libc::c_uint == 0 {
        (*base).flags &= !(0x8 as libc::c_int) as libc::c_uint;
        (*base).flags |= (0x2 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint;
    }
    sudo_debug_exit_v1(
        (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"sudo_ev_loopexit_v1\0"))
            .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        740 as libc::c_int,
        sudo_debug_subsys,
    );
}

#[no_mangle]
pub unsafe extern "C" fn sudo_ev_loopbreak_v1(mut base: *mut sudo_event_base) {
    let sudo_debug_subsys: libc::c_int = (4 as libc::c_int) << 6 as libc::c_int;
    sudo_debug_enter_v1(
        (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"sudo_ev_loopbreak_v1\0"))
            .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        746 as libc::c_int,
        sudo_debug_subsys,
    );
    if base.is_null() {
        base = default_base;
        if base.is_null() {
            sudo_debug_exit_v1(
                (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(
                    b"sudo_ev_loopbreak_v1\0",
                ))
                .as_ptr(),
                b"event.c\0" as *const u8 as *const libc::c_char,
                750 as libc::c_int,
                sudo_debug_subsys,
            );
            return;
        }
    }
    (*base).flags &=
    !(0x8 as libc::c_int | 0x2 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint;
    (*base).flags |= 0x4 as libc::c_int as libc::c_uint;
    sudo_debug_exit_v1(
        (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"sudo_ev_loopbreak_v1\0"))
            .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        756 as libc::c_int,
        sudo_debug_subsys,
    );
}

#[no_mangle]
pub unsafe extern "C" fn sudo_ev_loopcontinue_v1(mut base: *mut sudo_event_base) {
    let sudo_debug_subsys: libc::c_int = (4 as libc::c_int) << 6 as libc::c_int;
    sudo_debug_enter_v1(
        (*::core::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(b"sudo_ev_loopcontinue_v1\0"))
            .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        762 as libc::c_int,
        sudo_debug_subsys,
    );
    if base.is_null() {
        base = default_base;
        if base.is_null() {
            sudo_debug_exit_v1(
                (*::core::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                    b"sudo_ev_loopcontinue_v1\0",
                ))
                .as_ptr(),
                b"event.c\0" as *const u8 as *const libc::c_char,
                766 as libc::c_int,
                sudo_debug_subsys,
            );
            return;
        }
    }
    if (*base).flags & (0x1 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint == 0 {
        (*base).flags |= 0x8 as libc::c_int as libc::c_uint;
    }
    sudo_debug_exit_v1(
        (*::core::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(b"sudo_ev_loopcontinue_v1\0"))
            .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        773 as libc::c_int,
        sudo_debug_subsys,
    );
}

#[no_mangle]
pub unsafe extern "C" fn sudo_ev_got_exit_v1(mut base: *mut sudo_event_base) -> bool {
    let sudo_debug_subsys: libc::c_int = (4 as libc::c_int) << 6 as libc::c_int;
    sudo_debug_enter_v1(
        (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"sudo_ev_got_exit_v1\0"))
            .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        779 as libc::c_int,
        sudo_debug_subsys,
    );
    if base.is_null() {
        base = default_base;
        if base.is_null() {
            let mut sudo_debug_ret: bool = 0 as libc::c_int != 0;
            sudo_debug_exit_bool_v1(
                (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(
                    b"sudo_ev_got_exit_v1\0",
                ))
                .as_ptr(),
                b"event.c\0" as *const u8 as *const libc::c_char,
                783 as libc::c_int,
                sudo_debug_subsys,
                sudo_debug_ret,
            );
            return sudo_debug_ret;
        }
    }
    let mut sudo_debug_ret_0: bool = (*base).flags & 0x10 as libc::c_int as libc::c_uint != 0;
    sudo_debug_exit_bool_v1(
        (*::core::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"sudo_ev_got_exit_v1\0"))
            .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        785 as libc::c_int,
        sudo_debug_subsys,
        sudo_debug_ret_0,
    );
    return sudo_debug_ret_0;
}

#[no_mangle]
pub unsafe extern "C" fn sudo_ev_got_break_v1(mut base: *mut sudo_event_base) -> bool {
    let sudo_debug_subsys: libc::c_int = (4 as libc::c_int) << 6 as libc::c_int;
    sudo_debug_enter_v1(
        (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"sudo_ev_got_break_v1\0"))
            .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        791 as libc::c_int,
        sudo_debug_subsys,
    );
    if base.is_null() {
        base = default_base;
        if base.is_null() {
            let mut sudo_debug_ret: bool = 0 as libc::c_int != 0;
            sudo_debug_exit_bool_v1(
                (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(
                    b"sudo_ev_got_break_v1\0",
                ))
                .as_ptr(),
                b"event.c\0" as *const u8 as *const libc::c_char,
                795 as libc::c_int,
                sudo_debug_subsys,
                sudo_debug_ret,
            );
            return sudo_debug_ret;
        }
    }
    let mut sudo_debug_ret_0: bool = (*base).flags & 0x20 as libc::c_int as libc::c_uint != 0;
    sudo_debug_exit_bool_v1(
        (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"sudo_ev_got_break_v1\0"))
            .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        797 as libc::c_int,
        sudo_debug_subsys,
        sudo_debug_ret_0,
    );
    return sudo_debug_ret_0;
}
#[no_mangle]
pub unsafe extern "C" fn sudo_ev_get_timeleft_v1(
    mut ev: *mut sudo_event,
    mut tv: *mut timeval,
) -> libc::c_int {
    let mut ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut ret: libc::c_int = 0;
    ret = sudo_ev_get_timeleft_v2(ev, &mut ts);
    (*tv).tv_sec = ts.tv_sec;
    (*tv).tv_usec = ts.tv_nsec / 1000 as libc::c_int as libc::c_long;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn sudo_ev_get_timeleft_v2(
    mut ev: *mut sudo_event,
    mut ts: *mut timespec,
) -> libc::c_int {
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let sudo_debug_subsys: libc::c_int = (4 as libc::c_int) << 6 as libc::c_int;
    sudo_debug_enter_v1(
        (*::core::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(b"sudo_ev_get_timeleft_v2\0"))
            .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        816 as libc::c_int,
        sudo_debug_subsys,
    );
    if (*ev).flags as libc::c_int & 0x4 as libc::c_int == 0 {
        (*ts).tv_nsec = 0 as libc::c_int as __syscall_slong_t;
        (*ts).tv_sec = (*ts).tv_nsec;
        let mut sudo_debug_ret: libc::c_int = -(1 as libc::c_int);
        sudo_debug_exit_int_v1(
            (*::core::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"sudo_ev_get_timeleft_v2\0",
            ))
            .as_ptr(),
            b"event.c\0" as *const u8 as *const libc::c_char,
            820 as libc::c_int,
            sudo_debug_subsys,
            sudo_debug_ret,
        );
        return sudo_debug_ret;
    }
    sudo_gettime_mono_v1(&mut now);
    (*ts).tv_sec = (*ev).timeout.tv_sec - now.tv_sec;
    (*ts).tv_nsec = (*ev).timeout.tv_nsec - now.tv_nsec;
    while (*ts).tv_nsec < 0 as libc::c_int as libc::c_long {
        (*ts).tv_sec -= 1;
        (*ts).tv_nsec += 1000000000 as libc::c_int as libc::c_long;
    }
    if (*ts).tv_sec < 0 as libc::c_int as libc::c_long {
        (*ts).tv_nsec = 0 as libc::c_int as __syscall_slong_t;
        (*ts).tv_sec = (*ts).tv_nsec;
    }
    let mut sudo_debug_ret_0: libc::c_int = 0 as libc::c_int;
    sudo_debug_exit_int_v1(
        (*::core::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(b"sudo_ev_get_timeleft_v2\0"))
            .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        827 as libc::c_int,
        sudo_debug_subsys,
        sudo_debug_ret_0,
    );
    return sudo_debug_ret_0;
}