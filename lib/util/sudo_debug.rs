/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use crate::S_IWUSR;

pub type size_t = usize;
pub type __pid_t = libc::c_int;
pub type pid_t = __pid_t;
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type __id_t = libc::c_uint;
pub type id_t = __id_t;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type uid_t = __uid_t;

/*
 * The debug priorities and subsystems are currently hard-coded.
 * In the future we might consider allowing plugins to register their
 * own subsystems and provide direct access to the debugging API.
 */

/* Note: this must match the order in sudo_debug.h */
static mut sudo_debug_priorities: [*const libc::c_char; 9] = [
    b"crit\0" as *const u8 as *const libc::c_char,
    b"err\0" as *const u8 as *const libc::c_char,
    b"warn\0" as *const u8 as *const libc::c_char,
    b"notice\0" as *const u8 as *const libc::c_char,
    b"diag\0" as *const u8 as *const libc::c_char,
    b"info\0" as *const u8 as *const libc::c_char,
    b"trace\0" as *const u8 as *const libc::c_char,
    b"debug\0" as *const u8 as *const libc::c_char,
    0 as *const u8 as *const libc::c_char,
];

/* Note: this must match the order in sudo_debug.h */
static mut sudo_debug_default_subsystems: [*const libc::c_char; 15] = [
    b"args\0" as *const u8 as *const libc::c_char,
    b"conv\0" as *const u8 as *const libc::c_char,
    b"edit\0" as *const u8 as *const libc::c_char,
    b"event\0" as *const u8 as *const libc::c_char,
    b"exec\0" as *const u8 as *const libc::c_char,
    b"hooks\0" as *const u8 as *const libc::c_char,
    b"main\0" as *const u8 as *const libc::c_char,
    b"netif\0" as *const u8 as *const libc::c_char,
    b"pcomm\0" as *const u8 as *const libc::c_char,
    b"plugin\0" as *const u8 as *const libc::c_char,
    b"pty\0" as *const u8 as *const libc::c_char,
    b"selinux\0" as *const u8 as *const libc::c_char,
    b"util\0" as *const u8 as *const libc::c_char,
    b"utmp\0" as *const u8 as *const libc::c_char,
    0 as *const u8 as *const libc::c_char,
];

#[macro_export]
macro_rules! O_WRONLY {
    () => {
        01
    };
}

#[macro_export]
macro_rules! O_APPEND {
    () => {
        02000
    };
}

#[macro_export]
macro_rules! O_CREAT {
    () => {
        0100
    };
}

#[macro_export]
macro_rules! S_IRUSR {
    () => {
        0400
    };
}

#[macro_export]
macro_rules! IGNORE_RESULT {
    ($x:expr) => {
        __typeof__(x) y = (x);
       (void)y;
    };
}

/* Set file descriptor flags.  */
#[macro_export]
macro_rules! F_SETFD {
    () => {
        2
    };
}

/* For F_[GET|SET]FD.  */
#[macro_export]
macro_rules! FD_CLOEXEC {
    /* Actually anything with low bit set goes */
    () => {
        1
    };
}

//#[macro_export]
//macro_rules! NBBY {
//    () => {
//         8
//    };
//}

pub const NBBY: libc::c_int = 8;

#[macro_export]
macro_rules! round_nfds {
    ($_n:expr) => {
        ((($_n) + (4 * NBBY) - 1) & !((4 * NBBY) - 1))
    };
}

macro_rules! sudo_isset {
    ($_a:expr, $_i:expr) => {{
        (*(($_a).offset((($_i) / NBBY) as isize)) & (1 << (($_i) % NBBY)))
    }};
}

macro_rules! sudo_clrbit {
    ($_a:expr, $_i:expr) => {{
        (*(($_a).offset((($_i) / NBBY) as isize)) &= !(1 << (($_i) % NBBY)))
    }};
}

macro_rules! sudo_setbit {
    ($_a:expr, $_i:expr) => {{
        (*(($_a).offset((($_i) / NBBY) as isize)) |= (1 << (($_i) % NBBY)))
    }};
}


/* Extract subsystem number and convert to an index. */
// #define SUDO_DEBUG_SUBSYS(n) (((n) >> 6) - 1)
#[macro_export]
macro_rules! SUDO_DEBUG_SUBSYS {
    ($_n:expr) => {
        ((($_n) >> 6) - 1)
    };
}

/* Extract priority number and convert to an index. */
// #define SUDO_DEBUG_PRI(n) (((n) & 0x0f) - 1)
#[macro_export]
macro_rules! SUDO_DEBUG_PRI {
    ($_n:expr) => {
        ((($_n) & 0x0f) - 1)
    };
}

/* Flag to include string version of errno in debug info. */
// #define SUDO_DEBUG_ERRNO (1<<4)
#[macro_export]
macro_rules! SUDO_DEBUG_ERRNO {
    () => {
        (1 << 4)
    };
}

/* Flag to include function, file and line number in debug info. */
// #define SUDO_DEBUG_LINENO    (1<<5)
#[macro_export]
macro_rules! SUDO_DEBUG_LINENO {
    () => {
        (1 << 5)
    };
}

/* Initializer for instance index to indicate that debugging is not setup. */
// #define SUDO_DEBUG_INSTANCE_INITIALIZER      -1
#[macro_export]
macro_rules! SUDO_DEBUG_INSTANCE_INITIALIZER {
    () => {
        -1
    };
}

/* Error return for sudo_debug_register().  */
// #define SUDO_DEBUG_INSTANCE_ERROR	-2
#[macro_export]
macro_rules! SUDO_DEBUG_INSTANCE_ERROR {
    () => {
        -2
    };
}

/* Support up to 10 instances. */
// #define SUDO_DEBUG_INSTANCE_MAX 10
#[macro_export]
macro_rules! SUDO_DEBUG_INSTANCE_MAX {
    () => {
        10
    };
}

// #define SUDO_DEBUG_DEBUG	8	/* very verbose debugging */
#[macro_export]
macro_rules! SUDO_DEBUG_DEBUG {
    () => {
        8
    };
}

// #define EXEC_PREFIX "exec "
#[macro_export]
macro_rules! EXEC_PREFIX {
    () => {
        "exec "
    };
}

#[macro_export]
macro_rules! nitems {
    () => {
        (::std::mem::size_of::<[*const libc::c_char; 15]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            .wrapping_sub(1)
            .wrapping_sub(1) as libc::c_uint
    };
}

// #define NUM_DEF_SUBSYSTEMS (nitems(sudo_debug_default_subsystems) - 1)
#[macro_export]
macro_rules! NUM_DEF_SUBSYSTEMS {
    () => {
        (nitems!() - 1)
    };
}

// #define SUDO_DEBUG_TRACE	7	/* log function enter/exit */
#[macro_export]
macro_rules! SUDO_DEBUG_TRACE {
    () => {
        7
    };
}

extern "C" {
    fn free(__ptr: *mut libc::c_void);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn calloc(__nmemb: libc::size_t, __size: libc::size_t) -> *mut libc::c_void;
    fn reallocarray(_: *mut libc::c_void, _: libc::size_t, _: libc::size_t) -> *mut libc::c_void;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn open(_: *const libc::c_char, _: libc::c_int, ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn sudo_warn_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn sudo_warnx_nodebug_v1(fmt: *const libc::c_char, _: ...);
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, ...);
    fn realloc(__ptr: *mut libc::c_void, __size: libc::size_t) -> *mut libc::c_void;
    fn memset(__s: *mut libc::c_void, __c: libc::c_int, _n: libc::size_t) -> *mut libc::c_void;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn strchr(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn strcmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn malloc(__size: libc::size_t) -> *mut libc::c_void;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn fork() -> pid_t;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn sudo_getprogname() -> *const libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
     fn time(__timer: *mut time_t) -> time_t;
    fn ctime(__timer: *const time_t) -> *mut libc::c_char;
    fn writev(__fd: libc::c_int, __iovec: *const iovec, __count: libc::c_int) -> ssize_t;
    fn fchown(__fd: libc::c_int, __owner: __uid_t, __group: __gid_t) -> libc::c_int;
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        lineno: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_debug_output_sle {
    pub sle_next: *mut sudo_debug_output,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_debug_output {
    pub entries: sudo_debug_output_sle,
    pub filename: *mut libc::c_char,
    pub settings: *mut libc::c_int,
    pub fd: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_debug_output_list {
    pub slh_first: *mut sudo_debug_output,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_debug_instance {
    pub program: *mut libc::c_char,
    pub subsystems: *const *const libc::c_char,
    pub subsystem_ids: *const libc::c_uint,
    pub max_subsystem: libc::c_uint,
    pub outputs: sudo_debug_output_list,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_debug_file_list {
    pub tqe_next: *mut sudo_debug_file,
    pub tqe_prev: *mut *mut sudo_debug_file,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_debug_file {
    pub entries: sudo_debug_file_list,
    pub debug_file: *mut libc::c_char,
    pub debug_flags: *mut libc::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_conf_debug_file_list {
    tqh_first: *mut sudo_debug_file,
    tqh_last: *mut *mut sudo_debug_file,
}

static mut sudo_debug_fds_size: libc::c_int = -1;
static mut sudo_debug_fds: *mut libc::c_uchar = 0 as *const libc::c_char as *mut libc::c_uchar;
static mut sudo_debug_max_fd: libc::c_int = -1;
static mut sudo_debug_instances: [*mut sudo_debug_instance; SUDO_DEBUG_INSTANCE_MAX!()] =
    [0 as *const sudo_debug_instance as *mut sudo_debug_instance; SUDO_DEBUG_INSTANCE_MAX!()];
static mut sudo_debug_last_instance: libc::c_int = -1;
static mut sudo_debug_active_instance: libc::c_int = -(1 as libc::c_int);
static mut sudo_debug_pidstr: [libc::c_char; 14] = [0; 14];
static mut sudo_debug_pidlen: size_t = 0;

static mut sudo_debug_max_fd: libc::c_int = -1;
static mut sudo_debug_instances: [*mut sudo_debug_instance; SUDO_DEBUG_INSTANCE_MAX!()] =
    [0 as *const sudo_debug_instance as *mut sudo_debug_instance; SUDO_DEBUG_INSTANCE_MAX!()];
static mut sudo_debug_last_instance: libc::c_int = -1;
static mut sudo_debug_active_instance: libc::c_int = -(1 as libc::c_int);
static mut sudo_debug_pidstr: [libc::c_char; 14] = [0; 14];
static mut sudo_debug_pidlen: size_t = 0;


#[no_mangle]
pub unsafe extern "C" fn sudo_debug_free_output(output: *mut sudo_debug_output) {
    free((*output).filename as *mut libc::c_void);
    free((*output).settings as *mut libc::c_void);
    if ((*output).fd) != -1 {
        close((*output).fd);
    }
    free(output as *mut libc::c_void);
}

#[no_mangle]
pub unsafe extern "C" fn sudo_debug_new_output(
    instance: *mut sudo_debug_instance,
    debug_file: *mut sudo_debug_file,
) -> *mut sudo_debug_output {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut last: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut subsys: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pri: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut output: *mut sudo_debug_output = 0 as *mut sudo_debug_output;
    let j: libc::c_uint = 0;
    let i: libc::c_int = 0;
    let mut isbad: bool = false;

    output = calloc(1, std::mem::size_of::<sudo_debug_output>() as libc::size_t)
        as *mut sudo_debug_output;

    'oom: loop {
        if output == 0 as *mut sudo_debug_output {
            break 'oom;
        }

        (*output).fd = -1;
        (*output).settings = reallocarray(
            0 as *mut libc::c_void,
            ((*instance).max_subsystem + 1) as libc::size_t,
            std::mem::size_of::<libc::c_int>() as libc::size_t,
        ) as *mut libc::c_int;
        if ((*output).settings).is_null() {
            break 'oom;
        }

        (*output).filename = strdup((*debug_file).debug_file);
        if ((*output).filename).is_null() {
            break 'oom;
        }
        (*output).fd = -1;

           /* Init per-subsystems settings to -1 since 0 is a valid priority. */
        for j in 0..(*instance).max_subsystem + 1 {
            *((*output).settings).offset(j as isize) = -(1 as libc::c_int);
        }

        /* Open debug file. */
        (*output).fd = open(
            (*output).filename,
            O_WRONLY!() | O_APPEND!(),
            S_IRUSR!() | S_IWUSR!(),
        );

        if (*output).fd == -1 {
            /* Create debug file as needed and set group ownership. */
            if *__errno_location() == libc::ENOENT as libc::c_int {
                (*output).fd = open(
                    (*output).filename,
                    O_WRONLY!() | O_APPEND!() | O_CREAT!(),
                    S_IRUSR!() | S_IWUSR!(),
                );
            }
            if (*output).fd == -1 {
                sudo_warn_nodebug_v1((*output).filename);
                /*goto bad;*/
                isbad = true;
                break 'oom;
            }
            // ignore_result(fchown(output->fd, (uid_t)-1, 0));
            let mut _y: libc::c_int = fchown(
                (*output).fd,
                -(1 as libc::c_int) as uid_t,
                0 as libc::c_int as __gid_t,
            );
        }

        fcntl((*output).fd, F_SETFD!() | FD_CLOEXEC!());
        if sudo_debug_fds_size < (*output).fd {
            /* Bump fds size to the next multiple of 4 * NBBY. */
            let old_size = sudo_debug_fds_size / NBBY;
            let new_size = round_nfds!((*output).fd + 1) / NBBY;
            let new_fds: *mut libc::c_uchar;

            new_fds = realloc(
                sudo_debug_fds as *mut libc::c_void,
                new_size as libc::size_t,
            ) as *mut libc::c_uchar;
            if new_fds.is_null() {
                break 'oom;
            }
            memset(
                new_fds.offset(old_size as isize) as *mut libc::c_void,
                0,
                (new_size - old_size) as libc::size_t,
            );
            sudo_debug_fds = new_fds;
            sudo_debug_fds_size = new_size * NBBY;
        }

        sudo_setbit!(sudo_debug_fds, (*output).fd);
        if (*output).fd > sudo_debug_max_fd {
            sudo_debug_max_fd = (*output).fd;
        }

        /* Parse Debug conf string. */
        buf = strdup((*debug_file).debug_flags);
        if buf.is_null() {
            break 'oom;
        }
        cp = strtok_r(buf, b",\0" as *const u8 as *const libc::c_char, &mut last);
        while !cp.is_null() {
            cp = strtok_r(
                0 as *mut libc::c_char,
                b".\0" as *const u8 as *const libc::c_char,
                &mut last,
            );

            /* Should be in the form subsys@pri. */
            subsys = cp;
            pri = strchr(cp, '@' as libc::c_int);
            if pri.is_null() {
                continue;
            }

            pri = pri.offset(1);

             /* Look up priority and subsystem, fill in sudo_debug_settings[]. */
            while !(sudo_debug_priorities[i as usize]).is_null() {
                let mut ret = strcasecmp(
                    pri,
                    (sudo_debug_priorities[i as usize]) as *const libc::c_char,
                );
                if ret == 0 {
                    while !(*((*instance).subsystems).offset(j as isize)).is_null() {
                        ret = strcasecmp(subsys, b"all\0" as *const u8 as *const libc::c_char);
                        if ret == 0 {
                            let mut idx: libc::c_uint = if (*instance).subsystem_ids.is_null() {
                                SUDO_DEBUG_SUBSYS!(*((*instance).subsystem_ids).offset(j as isize))
                                    as libc::c_uint
                            } else {
                                j
                            };
                            if i > *((*output).settings).offset(idx as isize) {
                                *((*output).settings).offset(idx as isize) = i;
                            }
                            continue;
                        }

                        ret = strcasecmp(subsys, *((*instance).subsystems).offset(j as isize));
                        if ret == 0 {
                            let mut idx: libc::c_uint = if (*instance).subsystem_ids.is_null() {
                                SUDO_DEBUG_SUBSYS!(*((*instance).subsystem_ids).offset(j as isize))
                                    as libc::c_uint
                            } else {
                                j
                            };
                            if i > *((*output).settings).offset(idx as isize) {
                                *((*output).settings).offset(idx as isize) = i;
                            }
                            break;
                        }
                    }
                }
            }
        }
        break 'oom;
    }

    free(buf as *mut libc::c_void);

    if !isbad {
        sudo_warn_nodebug_v1(0 as *const libc::c_char);
    }

    /* bad: */
    if output.is_null() {
        sudo_debug_free_output(output);
    }
    return 0 as *mut sudo_debug_output;
}

#[no_mangle]
pub unsafe extern "C" fn sudo_debug_register_v1(
    mut program: *const libc::c_char,
    mut subsystems: *const *const libc::c_char,
    mut ids: *mut libc::c_uint,
    mut debug_files: *mut sudo_conf_debug_file_list,
) -> libc::c_int {
    let mut instance: *mut sudo_debug_instance = 0 as *mut sudo_debug_instance;
    let mut output: *mut sudo_debug_output = 0 as *mut sudo_debug_output;
    let mut debug_file: *mut sudo_debug_file = 0 as *mut sudo_debug_file;
    let mut idx: libc::c_int = -1;
    let mut free_idx: libc::c_int = -1;

    debug_decl_func!(sudo_debug_register);

    if debug_files.is_null() {
        return SUDO_DEBUG_INSTANCE_INITIALIZER!();
    }

    /* Use default subsystem names if none are provided. */
    if subsystems.is_null() {
        subsystems = sudo_debug_default_subsystems.as_ptr();
    } else if ids.is_null() {
        return SUDO_DEBUG_INSTANCE_ERROR!();
    }

     /* Search for existing instance. */
    for i in 0..sudo_debug_last_instance + 1 {
        if sudo_debug_instances[i as usize].is_null() {
            free_idx = i;
            continue;
        }

        if (*sudo_debug_instances[i as usize]).subsystems == subsystems
            && strcmp((*sudo_debug_instances[i as usize]).program, program) == 0
        {
            instance = sudo_debug_instances[i as usize];
            break;
        }
    }

    if instance.is_null() {
        let mut i: libc::c_uint = 0;
        let mut max_id: libc::c_uint = NUM_DEF_SUBSYSTEMS!() - 1;

        /* Fill in subsystem name -> id mapping as needed. */
        if !ids.is_null() {
            while !subsystems.offset(i as isize).is_null() {
                i += 1;
                /* Check default subsystems. */
                for mut j in 0..NUM_DEF_SUBSYSTEMS!() {
                    if strcmp(
                        *subsystems.offset(i as isize),
                        sudo_debug_default_subsystems[j as usize],
                    ) == 0
                    {
                        break;
                    }

                    if j == NUM_DEF_SUBSYSTEMS!() {
                        max_id += 1;
                        j = max_id;
                    }
                    *(ids.offset(i as isize)) = (j + 1) << 6;
                } // for j in NUM_DEF_SUBSYSTEMS!()
            } // !while !subsystems[i].is_null()
        } // !!ids.is_null()
        
        if free_idx != -1 {
            idx = free_idx;
        }
        if idx == SUDO_DEBUG_INSTANCE_MAX!() {
            /* XXX - realloc? */
            sudo_warnx_nodebug_v1(
                b"too many debug instances (max %d)\0" as *const u8 as *const libc::c_char,
                SUDO_DEBUG_INSTANCE_MAX!(),
            );
            return SUDO_DEBUG_INSTANCE_ERROR!();
        }

        if (idx != (sudo_debug_last_instance + 1)) && (idx != free_idx) {
            sudo_warnx_nodebug_v1(
                b"%s: instance number mismatch: expected %d or %d, got %d\0" as *const u8
                    as *const libc::c_char,
                stdext::function_name!().as_ptr() as *const libc::c_char,
                sudo_debug_last_instance + 1,
                free_idx,
                idx,
            );
            return SUDO_DEBUG_INSTANCE_ERROR!();
        }
        instance = malloc(std::mem::size_of::<sudo_debug_instance>() as libc::size_t)
            as *mut sudo_debug_instance;
        if instance.is_null() {
            return SUDO_DEBUG_INSTANCE_ERROR!();
        }

        (*instance).program = strdup(program);
        if (*instance).program.is_null() {
            free(instance as *mut libc::c_void);
            return SUDO_DEBUG_INSTANCE_ERROR!();
        }
        (*instance).subsystems = subsystems;
        (*instance).subsystem_ids = ids;
        (*instance).max_subsystem = max_id;
        (*instance).outputs.slh_first = 0 as *mut sudo_debug_output;
        sudo_debug_instances[idx as usize] = instance;
        if idx != free_idx {
            sudo_debug_last_instance += 1;
        }
    } else if !ids.is_null() && (*instance).subsystem_ids != ids {
        let mut i: libc::c_uint = 0;
        loop {
            if subsystems.offset(i as isize).is_null() {
                break;
            }
            *(ids.offset(i as isize)) = *((*instance).subsystem_ids).offset(i as isize);
            i += 1;
        }
    }

    debug_file = (*debug_files).tqh_first;
    while !debug_file.is_null() {
        output = sudo_debug_new_output(instance, debug_file);
        if output.is_null() {
            (*output).entries.sle_next = (*instance).outputs.slh_first;
            (*instance).outputs.slh_first = output;
        }
        debug_file = (*debug_file).entries.tqe_next;
    }

    /* Set active instance. */
    sudo_debug_active_instance = idx;

    /* Stash the pid string so we only have to format it once. */

    if sudo_debug_pidlen == 0 {
        snprintf(
            sudo_debug_pidstr.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong,
            b"[%d] \0" as *const u8 as *const libc::c_char,
            getpid(),
        );
        sudo_debug_pidlen = strlen(sudo_debug_pidstr.as_mut_ptr()) as size_t;
    }

    return idx;
}

#[no_mangle]
pub unsafe extern "C" fn sudo_debug_deregister_v1(mut idx: libc::c_int) -> libc::c_int {
    let mut instance: *mut sudo_debug_instance = 0 as *mut sudo_debug_instance;
    let mut output: *mut sudo_debug_output = 0 as *mut sudo_debug_output;
    let mut next: *mut sudo_debug_output = 0 as *mut sudo_debug_output;

    debug_decl_func!(sudo_debug_deregister);

    if idx < 0 || idx > sudo_debug_last_instance {
        sudo_warnx_nodebug_v1(
            b"%s: invalid instance ID %d, max %d\0" as *const u8 as *const libc::c_char,
            stdext::function_name!().as_ptr() as *const libc::c_char,
            idx,
            sudo_debug_last_instance,
        );
        return -1;
    }

     /* Reset active instance as needed. */
    if sudo_debug_active_instance == idx {
        sudo_debug_active_instance = -1;
    }

    instance = sudo_debug_instances[idx as usize];
    if instance.is_null() {
        return -1; /* already deregistered */
    }

    sudo_debug_instances[idx as usize] = 0 as *mut sudo_debug_instance;

    while output.is_null() && {
        next = (*output).entries.sle_next;
        1 as libc::c_int != 0
    } {
        close((*output).fd);
        free((*output).filename as *mut libc::c_void);
        free((*output).settings as *mut libc::c_void);
        free(output as *mut libc::c_void);
        output = next;
    }

    free((*instance).program as *mut libc::c_void);
    free(instance as *mut libc::c_void);

    if idx == sudo_debug_last_instance {
        sudo_debug_last_instance -= 1;
    }

    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn sudo_debug_get_instance_v1(
    mut program: *const libc::c_char,
) -> libc::c_int {
    for idx in 0..sudo_debug_last_instance {
        if sudo_debug_instances[idx as usize].is_null() {
            continue;
        }
        if strcmp((*sudo_debug_instances[idx as usize]).program, program) == 0 {
            return idx;
        }
    }
    return SUDO_DEBUG_INSTANCE_INITIALIZER!();
}

#[no_mangle]
pub unsafe extern "C" fn sudo_debug_fork_v1() -> pid_t {
    let mut pid: pid_t = 0 as pid_t;

    pid = fork();
    if pid == 0 {
        snprintf(
            sudo_debug_pidstr.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong,
            b"[%d] \0" as *const u8 as *const libc::c_char,
            getpid(),
        );
        sudo_debug_pidlen = strlen(sudo_debug_pidstr.as_mut_ptr()) as usize;
    }
    return pid;
}

#[no_mangle]
pub unsafe extern "C" fn sudo_debug_enter_v1(
    func: *const libc::c_char,
    file: *const libc::c_char,
    line: libc::c_int,
    subsys: libc::c_int,
) {
    sudo_debug_printf2_v1(
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0,
        subsys | SUDO_DEBUG_TRACE!(),
        b"-> %s @ %s:%d\0" as *const u8 as *const libc::c_char,
        func,
        file,
        line,
    );
}

#[no_mangle]
pub unsafe extern "C" fn sudo_debug_exit_v1(
    func: *const libc::c_char,
    file: *const libc::c_char,
    line: libc::c_int,
    subsys: libc::c_int,
) {
    sudo_debug_printf2_v1(
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0,
        subsys | SUDO_DEBUG_TRACE!(),
        b"<- %s @ %s:%d\0" as *const u8 as *const libc::c_char,
        func,
        file,
        line,
    );
}

#[no_mangle]
pub unsafe extern "C" fn sudo_debug_exit_int_v1(
    func: *const libc::c_char,
    file: *const libc::c_char,
    line: libc::c_int,
    subsys: libc::c_int,
    ret: libc::c_int,
) {
    sudo_debug_printf2_v1(
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0,
        subsys | SUDO_DEBUG_TRACE!(),
        b"<- %s @ %s:%d := %d\0" as *const u8 as *const libc::c_char,
        func,
        file,
        line,
        ret,
    );
}

#[no_mangle]
pub unsafe extern "C" fn sudo_debug_exit_long_v1(
    func: *const libc::c_char,
    file: *const libc::c_char,
    line: libc::c_int,
    subsys: libc::c_int,
    ret: libc::c_long,
) {
    sudo_debug_printf2_v1(
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0,
        subsys | SUDO_DEBUG_TRACE!(),
        b"<- %s @ %s:%d := %ld\0" as *const u8 as *const libc::c_char,
        func,
        file,
        line,
        ret,
    );
}

#[no_mangle]
pub unsafe extern "C" fn sudo_debug_exit_id_t_v1(
    func: *const libc::c_char,
    file: *const libc::c_char,
    line: libc::c_int,
    subsys: libc::c_int,
    ret: id_t,
) {
    sudo_debug_printf2_v1(
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0,
        subsys | SUDO_DEBUG_TRACE!(),
        b"<- %s @ %s:%d := %d\0" as *const u8 as *const libc::c_char,
        func,
        file,
        line,
        ret as libc::c_int,
    );
}

#[no_mangle]
pub unsafe extern "C" fn sudo_debug_exit_size_t_v1(
    func: *const libc::c_char,
    file: *const libc::c_char,
    line: libc::c_int,
    subsys: libc::c_int,
    ret: size_t,
) {
    sudo_debug_printf2_v1(
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0,
        subsys | SUDO_DEBUG_TRACE!(),
        b"<- %s @ %s:%d := %zu\0" as *const u8 as *const libc::c_char,
        func,
        file,
        line,
        ret,
    );
}

#[no_mangle]
pub unsafe extern "C" fn sudo_debug_exit_ssize_t_v1(
    func: *const libc::c_char,
    file: *const libc::c_char,
    line: libc::c_int,
    subsys: libc::c_int,
    ret: ssize_t,
) {
    sudo_debug_printf2_v1(
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0,
        subsys | SUDO_DEBUG_TRACE!(),
        b"<- %s @ %s:%d := %zd\0" as *const u8 as *const libc::c_char,
        func,
        file,
        line,
        ret,
    );
}


#[no_mangle]
pub unsafe extern "C" fn sudo_debug_exit_time_t_v1(
    func: *const libc::c_char,
    file: *const libc::c_char,
    line: libc::c_int,
    subsys: libc::c_int,
    ret: time_t,
) {
    sudo_debug_printf2_v1(
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0,
        subsys | SUDO_DEBUG_TRACE!(),
        b"<- %s @ %s:%d := %lld\0" as *const u8 as *const libc::c_char,
        func,
        file,
        line,
        ret as libc::c_longlong,
    );
}


#[no_mangle]
pub unsafe extern "C" fn sudo_debug_exit_bool_v1(
    func: *const libc::c_char,
    file: *const libc::c_char,
    line: libc::c_int,
    subsys: libc::c_int,
    ret: bool,
) {
    sudo_debug_printf2_v1(
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0,
        subsys | SUDO_DEBUG_TRACE!(),
        b"<- %s @ %s:%d := %s\0" as *const u8 as *const libc::c_char,
        func,
        file,
        line,
        if ret {
            b"true\0" as *const u8 as *const libc::c_char
        } else {
            b"false\0" as *const u8 as *const libc::c_char
        },
    );
}

#[no_mangle]
pub unsafe extern "C" fn sudo_debug_exit_str_v1(
    func: *const libc::c_char,
    file: *const libc::c_char,
    line: libc::c_int,
    subsys: libc::c_int,
    ret: *const libc::c_char,
) {
    sudo_debug_printf2_v1(
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0,
        subsys | SUDO_DEBUG_TRACE!(),
        b"<- %s @ %s:%d := %s\0" as *const u8 as *const libc::c_char,
        func,
        file,
        line,
        if ret.is_null() {
            ret
        } else {
            b"(null)\0" as *const u8 as *const libc::c_char
        },
    );
}

#[no_mangle]
pub unsafe extern "C" fn sudo_debug_exit_str_masked_v1(
    func: *const libc::c_char,
    file: *const libc::c_char,
    line: libc::c_int,
    subsys: libc::c_int,
    ret: *const libc::c_char,
) {
    static mut stars: [libc::c_char; 81] = unsafe {
        *::std::mem::transmute::<&[u8; 81], &[libc::c_char; 81]>(
            b"********************************************************************************\0",
        )
    };

    let mut len: libc::c_int = (if !ret.is_null() {
        strlen(ret)
    } else {
        (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong).wrapping_sub(1)
    }) as libc::c_int;

    sudo_debug_printf2_v1(
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0,
        subsys | SUDO_DEBUG_TRACE!(),
        b"<- %s @ %s:%d := %.*s\0" as *const u8 as *const libc::c_char,
        func,
        file,
        line,
        len,
        if !ret.is_null() {
            stars.as_ptr()
        } else {
            b"(null)\0" as *const u8 as *const libc::c_char
        },
    );
}


#[no_mangle]
pub unsafe extern "C" fn sudo_debug_exit_ptr_v1(
    func: *const libc::c_char,
    file: *const libc::c_char,
    line: libc::c_int,
    subsys: libc::c_int,
    ret: *const libc::c_void,
) {
    sudo_debug_printf2_v1(
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0,
        subsys | SUDO_DEBUG_TRACE!(),
        b"<- %s @ %s:%d := %p\0" as *const u8 as *const libc::c_char,
        func,
        file,
        line,
        ret,
    );
}

#[no_mangle]
pub unsafe extern "C" fn sudo_debug_write2_v1(
    mut fd: libc::c_int,
    mut func: *const libc::c_char,
    mut file: *const libc::c_char,
    mut lineno: libc::c_int,
    mut str: *const libc::c_char,
    mut len: libc::c_int,
    mut errnum: libc::c_int,
) {
    let mut timestr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut numbuf: [libc::c_char; 13] = [0; 13];

    let mut now: time_t = 0 as time_t;
    let mut iov: [iovec; 12] = [iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    }; 12];
    let mut iovcnt: libc::c_int = 3;

    iov[1].iov_base = sudo_getprogname() as *mut libc::c_char as *mut libc::c_void;
    iov[1].iov_len = strlen(iov[1 as usize].iov_base as *const libc::c_char) as size_t;
    iov[2].iov_base = sudo_debug_pidstr.as_mut_ptr() as *mut libc::c_void;
    iov[2].iov_len = sudo_debug_pidlen;

    while len > 0 && *(str.offset((len - 1) as isize)) == '\n' as i32 as libc::c_char {
        len -= 1;
    }

    if len > 0 {
        iov[iovcnt as usize].iov_base = str as *mut libc::c_char as *mut libc::c_void;
        iov[iovcnt as usize].iov_len = len as size_t;
        iovcnt += 1;
    }

    /* Append error string if errno is specified. */
    if errnum != 0 {
        if len > 0 {
            iov[iovcnt as usize].iov_base =
                b": \0" as *const u8 as *const libc::c_char as *mut libc::c_void;
            iov[iovcnt as usize].iov_len = 2;
        }
        iov[iovcnt as usize].iov_base =
            strerror(errnum) as *const libc::c_char as *mut libc::c_void;
        iov[iovcnt as usize].iov_len =
            strlen(iov[iovcnt as usize].iov_base as *const libc::c_char) as size_t;
        iovcnt += 1;
    } // !if errnum != 0

    /* If function, file and lineno are specified, append them. */
    if !func.is_null() && !file.is_null() && lineno != 0 {
        iov[iovcnt as usize].iov_base =
            b" @ \0" as *const u8 as *const libc::c_char as *mut libc::c_void;
        iov[iovcnt as usize].iov_len = 3;
        iovcnt += 1;

        iov[iovcnt as usize].iov_base = func as *mut libc::c_char as *mut libc::c_void;
        iov[iovcnt as usize].iov_len = strlen(func as *const libc::c_char) as size_t;
        iovcnt += 1;

        iov[iovcnt as usize].iov_base =
            b"() \0" as *const u8 as *const libc::c_char as *mut libc::c_void;
        iov[iovcnt as usize].iov_len = 3;
        iovcnt += 1;

        iov[iovcnt as usize].iov_base = file as *mut libc::c_char as *mut libc::c_void;
        iov[iovcnt as usize].iov_len = strlen(file as *const libc::c_char) as size_t;
        iovcnt += 1;

        snprintf(
            numbuf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong,
            b":%d\0" as *const u8 as *const libc::c_char,
            lineno,
        );
        iov[iovcnt as usize].iov_base = numbuf.as_mut_ptr() as *mut libc::c_void;
        iov[iovcnt as usize].iov_len = strlen(numbuf.as_mut_ptr()) as size_t;
        iovcnt += 1;
    }
}

//end
#[no_mangle]
pub unsafe extern "C" fn sudo_debug_execve2_v1(
    mut level: libc::c_int,
    mut path: *const libc::c_char,
    mut argv: *const *mut libc::c_char,
    mut envp: *const *mut libc::c_char,
) {
    let mut buflen: libc::c_int = 0;
    let mut pri: libc::c_int = 0;
    let mut saved_errno: libc::c_int = *__errno_location();
    let mut subsys: libc::c_uint = 0;
    let mut instance: *mut sudo_debug_instance = 0 as *mut sudo_debug_instance;
    let mut output: *mut sudo_debug_output = 0 as *mut sudo_debug_output;
    let mut av: *const *mut libc::c_char = 0 as *const *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut static_buf: [libc::c_char; 4096] = [0; 4096];
    let mut buf: *mut libc::c_char = static_buf.as_mut_ptr();
    let mut plen: size_t = 0;

    debug_decl_func!(sudo_debug_execve2);

    'out: loop {
        if sudo_debug_active_instance == -1 {
            break 'out;
        }

        pri = SUDO_DEBUG_PRI!(level);
        subsys = SUDO_DEBUG_SUBSYS!(level) as libc::c_uint;

        if sudo_debug_active_instance > sudo_debug_last_instance {
            sudo_warnx_nodebug_v1(
                b"%s: invalid instance ID %d, max %d\0" as *const u8 as *const libc::c_char,
                stdext::function_name!().as_ptr() as *const libc::c_char,
                sudo_debug_active_instance,
                sudo_debug_last_instance,
            );
            break 'out;
        }

        instance = sudo_debug_instances[sudo_debug_active_instance as usize];
        if instance.is_null() {
            sudo_warnx_nodebug_v1(
                b"%s: unregistered instance index %d\0" as *const u8 as *const libc::c_char,
                stdext::function_name!().as_ptr() as *const libc::c_char,
                sudo_debug_active_instance,
            );
            break 'out;
        }

        if subsys > (*instance).max_subsystem {
            break 'out;
        }

        output = (*instance).outputs.slh_first;
        while !output.is_null() {
            let mut log_envp: bool = false;

            /* Make sure we want debug info at this level. */
            if *((*output).settings).offset(subsys as isize) < pri {
                continue;
            }

            /* Log envp for debug level "debug". */
            if *((*output).settings).offset(subsys as isize) >= SUDO_DEBUG_DEBUG!() - 1
                && !(*envp.offset(0 as isize)).is_null()
            {
                log_envp = true;
            }

            /* Alloc and build up buffer. */
            plen = strlen(path) as size_t;
            let EXEC_PREFIX_len: libc::c_int =
                ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_int;
            buflen = EXEC_PREFIX_len
                .wrapping_sub(1)
                .wrapping_add(plen as libc::c_int) as libc::c_int;


            if argv.offset(0 as isize).is_null() {
                buflen += ::std::mem::size_of::<[libc::c_char; 4]>().wrapping_sub(1) as libc::c_int;

                av = argv;
                while !(*av).is_null() {
                    buflen = buflen.wrapping_add((strlen(*av)).wrapping_add(1) as libc::c_int)
                        as libc::c_int;
                    av = av.offset(1);
                }
                buflen -= 1;
            }

            if log_envp {
                buflen += ::std::mem::size_of::<[libc::c_char; 4]>().wrapping_sub(1) as libc::c_int;
                av = envp;
                while !(*av).is_null() {
                    buflen = buflen.wrapping_add((strlen(*av)).wrapping_add(1) as libc::c_int)
                        as libc::c_int;
                    av = av.offset(1);
                }
                buflen -= 1;
            }

            if buflen >= ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_int {
                buf = malloc((buflen + 1) as usize) as *mut libc::c_char;
                if buf.is_null() {
                    break 'out;
                }
            }

            /* Copy prefix and command. */
            memcpy(
                buf as *mut libc::c_void,
                b"exec \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                EXEC_PREFIX_len.wrapping_sub(1) as libc::c_ulong,
            );
            cp = buf.offset(EXEC_PREFIX_len.wrapping_sub(1) as isize);
            memcpy(
                cp as *mut libc::c_void,
                path as *mut libc::c_void,
                plen as libc::c_ulong,
            );
            cp = cp.offset(plen as isize);

            /* Copy argv. */
            if argv.offset(0).is_null() {
                cp = cp.offset(1);
                *cp = ' ' as i32 as libc::c_char;
                cp = cp.offset(1);
                *cp = '[' as i32 as libc::c_char;
                av = argv;
                while (*av).is_null() {
                    let mut avlen: size_t = strlen(*av) as size_t;
                    memcpy(
                        cp as *mut libc::c_void,
                        *av as *mut libc::c_void,
                        avlen as libc::c_ulong,
                    );
                    cp = cp.offset(avlen as isize);
                    cp = cp.offset(1);
                    *cp = ' ' as i32 as libc::c_char;
                    av = av.offset(1);
                } // !while
                *cp.offset(-(1 as libc::c_int) as isize) = ']' as i32 as libc::c_char;
            } // !if argv.offset(0)
        }
    }
}


/*
 * Returns the active instance or SUDO_DEBUG_INSTANCE_INITIALIZER
 * if no instance is active.
 */
#[no_mangle]
pub unsafe extern "C" fn sudo_debug_get_active_instance_v1() -> libc::c_int {
    return sudo_debug_active_instance;
}

/*
 * Sets a new active instance, returning the old one.
 * Note that the old instance may be SUDO_DEBUG_INSTANCE_INITIALIZER
 * if this is the only instance.
 */
#[no_mangle]
pub unsafe extern "C" fn sudo_debug_set_active_instance_v1(idx: libc::c_int) -> libc::c_int {
    let old_idx: libc::c_int = sudo_debug_active_instance;

    if idx >= -1 && idx <= sudo_debug_last_instance {
        sudo_debug_active_instance = idx;
    }

    return old_idx;
}

#[no_mangle]
pub unsafe extern "C" fn sudo_debug_update_fd_v1(ofd: libc::c_int, nfd: libc::c_int) {
    if ofd <= sudo_debug_max_fd && sudo_isset!(sudo_debug_fds, ofd) != 0 {
        /* Update sudo_debug_fds. */
        sudo_clrbit!(sudo_debug_fds, ofd);
        sudo_setbit!(sudo_debug_fds, nfd);

        /* Update the outputs. */
        for idx in 0..sudo_debug_last_instance + 1 {
            let mut instance: *mut sudo_debug_instance = 0 as *mut sudo_debug_instance;
            let mut output: *mut sudo_debug_output = 0 as *mut sudo_debug_output;

            instance = sudo_debug_instances[idx as usize];
            if instance.is_null() {
                continue;
            }

            output = (*instance).outputs.slh_first;
            while !output.is_null() {
                if (*output).fd == ofd {
                    (*output).fd = nfd;
                }
                output = (*output).entries.sle_next;
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn sudo_debug_get_fds_v1(mut fds: *mut *mut libc::c_uchar) -> libc::c_int {
    *fds = sudo_debug_fds;
    return sudo_debug_max_fd;
}