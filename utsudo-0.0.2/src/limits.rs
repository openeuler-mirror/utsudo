/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */
pub type rlim_t = libc::c_ulong;
//struct rlimit
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlimit {
    pub rlim_cur: rlim_t, //rlim_t = ulong
    pub rlim_max: rlim_t,
}
//struct saved_limit
#[derive(Copy, Clone)]
#[repr(C)]
pub struct saved_limit {
    pub resource: libc::c_int,
    pub saved: bool,
    pub limit: rlimit,
}
//等同c宏
pub const RLIMIT_AS: libc::c_int = 9;
pub const RLIMIT_CPU: libc::c_int = 0;
pub const RLIMIT_DATA: libc::c_int = 2;
pub const RLIMIT_FSIZE: libc::c_int = 1;
pub const RLIMIT_NOFILE: libc::c_int = 7;
pub const RLIMIT_NPROC: libc::c_int = 6;
pub const RLIMIT_RSS: libc::c_int = 5;
pub const RLIMIT_STACK: libc::c_int = 3;
pub const RLIMIT_CORE: libc::c_int = 4;
pub const PR_GET_DUMPABLE: libc::c_int = 3;
pub const PR_SET_DUMPABLE: libc::c_int = 4;
pub const RLIM_INFINITY: libc::c_int = -1;

//#[derive(Copy,Clone)]
//#[repr(C)]
static mut saved_limits: [saved_limit; 8] = [
    {
        let mut init = saved_limit {
            resource: RLIMIT_AS,
            saved: false,
            limit: rlimit {
                rlim_cur: 0,
                rlim_max: 0,
            },
        };
        init
    },
    {
        let mut init = saved_limit {
            resource: RLIMIT_CPU,
            saved: false,
            limit: rlimit {
                rlim_cur: 0,
                rlim_max: 0,
            },
        };
        init
    },
    {
        let mut init = saved_limit {
            resource: RLIMIT_DATA,
            saved: false,
            limit: rlimit {
                rlim_cur: 0,
                rlim_max: 0,
            },
        };
        init
    },
    {
        let mut init = saved_limit {
            resource: RLIMIT_FSIZE,
            saved: false,
            limit: rlimit {
                rlim_cur: 0,
                rlim_max: 0,
            },
        };
        init
    },
    {
        let mut init = saved_limit {
            resource: RLIMIT_NOFILE,
            saved: false,
            limit: rlimit {
                rlim_cur: 0,
                rlim_max: 0,
            },
        };
        init
    },
    {
        let mut init = saved_limit {
            resource: RLIMIT_NPROC,
            saved: false,
            limit: rlimit {
                rlim_cur: 0,
                rlim_max: 0,
            },
        };
        init
    },
    {
        let mut init = saved_limit {
            resource: RLIMIT_RSS,
            saved: false,
            limit: rlimit {
                rlim_cur: 0,
                rlim_max: 0,
            },
        };
        init
    },
    {
        let mut init = saved_limit {
            resource: RLIMIT_STACK,
            saved: false,
            limit: rlimit {
                rlim_cur: 0,
                rlim_max: 0,
            },
        };
        init
    },
];

#[no_mangle]
unsafe extern "C" fn restore_limits() {
    let mut idx: libc::c_uint = 0;
    //define debug_decl(restore_limits,SUDO_DEBUG_UTIL)
    debug_decl!(restore_limits, SUDO_DEBUG_UTIL);
    //end of define
    //sizeof使用方法，第二行ptr,offset函数加在那里的原因，第三行中结构体前为何要＊，
    while (idx as libc::c_ulong)
        < (::std::mem::size_of::<[saved_limit; 8]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<saved_limit>() as libc::c_ulong)
    {
        let mut lim: *mut saved_limit = &mut *saved_limits.as_mut_ptr().offset(idx as isize);
        if (*lim).saved {
            if setrlimit((*lim).resource, &mut (*lim).limit) == -1 {
                //define sudo_warn("setrlimit(%d)",lim.resource);
                sudo_debug_printf!(
                    SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                    b"setrlimit(%d)\0" as *const u8 as *const libc::c_char,
                    (*lim).resource
                );
                sudo_warn_nodebug_v1(
                    b"setrlimit(%d)\0" as *const u8 as *const libc::c_char,
                    (*lim).resource,
                );
                //end of define
            }
        }
        idx += 1;
    }
    restore_coredump();
    //define debug_return;
    debug_return!();
    //end of define;
}

