/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

pub type rlim_t = libc::c_ulong;

//struct rlimit
//作为以下函数成员
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlimit {
    pub rlim_cur: rlim_t, //rlim_t = ulong
    pub rlim_max: rlim_t,
}

//struct saved_limit
//结构体中变量前pub,每行逗号分隔,结构体定义前加的两行属性说明，目前不清楚原因
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

//8个结构体成员初始化，bool类型默认false
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

//c里面static修饰的变量，包括结构体之类的转换后还是static
static mut corelimit: rlimit = rlimit {
    rlim_cur: 0,
    rlim_max: 0,
};
static mut coredump_disabled: bool = false;
static mut nproclimit: rlimit = rlimit {
    rlim_cur: 0,
    rlim_max: 0,
};
static mut dumpflag: libc::c_int = 0;

//包含的函数为被调用函数，如在其他声明文件中的
extern "C" {
    fn getrlimit(__resource: libc::c_int, __rlimit: *mut rlimit) -> libc::c_int;
    fn setrlimit(__resource: libc::c_int, __rlimits: *const rlimit) -> libc::c_int;
    //c(,...) == rust(_:...) 可表多参数
    fn prctl(__option: libc::c_int, _: ...) -> libc::c_int;
    fn sudo_warn_nodebug_v1(fmt: *const libc::c_char, _: ...);
}

use utsudo_util::debug_decl;
use utsudo_util::debug_decl_vars;
use utsudo_util::sudo_debug_macro::sudo_debug_subsys;
pub const SUDO_DEBUG_UTIL: libc::c_int = 13 << 6;
use utsudo_util::sudo_debug::sudo_debug_enter_v1;
use utsudo_util::sudo_debug_macro::SUDO_DEBUG_ERRNO;
use utsudo_util::sudo_debug_macro::SUDO_DEBUG_LINENO;
//use utsudo_util::sudo_debug_macro::SUDO_DEBUG_INFO;
//use utsudo_util::sudo_debug_macro::SUDO_DEBUG_ERROR;
use crate::sudo_debug_printf2_v1;
use utsudo_util::debug_return;
use utsudo_util::sudo_debug_macro::SUDO_DEBUG_WARN;
use utsudo_util::sudo_debug_printf;
//use utsudo_util::debug_return_int;
use utsudo_util::sudo_debug::sudo_debug_exit_v1;

//该属性避免编译时函数名被修改
#[no_mangle]
pub unsafe extern "C" fn disable_coredump() {
    let mut rl: rlimit = {
        let mut init = rlimit {
            rlim_cur: 0,
            rlim_max: 0,
        };
        init
    };

    //define debug_decl(disable_coredump,SUDO_DEBUG_UTIL)
    debug_decl!(disable_coredump, SUDO_DEBUG_UTIL);
    //end of define

    //c(&) == rust(&mut)?
    if getrlimit(RLIMIT_CORE, &mut corelimit) == -1 {
        //define sudo_warn("getrlimit(RLIMIT_CORE)");
        sudo_debug_printf!(
            SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
            b"getrlimit(RLIMIT_CORE)\0" as *const u8 as *const libc::c_char
        );
        sudo_warn_nodebug_v1(b"getrlimit(RLIMIT_CORE)\0" as *const u8 as *const libc::c_char);
        //end of define;
    }

    if setrlimit(RLIMIT_CORE, &mut rl) == -1 {
        //define sudo_warn("setrlimit(RLIMIT_CORE)");
        sudo_debug_printf!(
            SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
            b"setrlimit(RLIMIT_CORE)\0" as *const u8 as *const libc::c_char
        );
        sudo_warn_nodebug_v1(b"setrlimit(RLIMIT_CORE)\0" as *const u8 as *const libc::c_char);
        //end of define
    }

    dumpflag = prctl(PR_GET_DUMPABLE, 0, 0, 0, 0);
    if dumpflag == -1 {
        dumpflag = 0;
    }
    prctl(PR_GET_DUMPABLE, 0, 0, 0, 0);

    //c:coredump_disabled = true; 为何!=0?
    coredump_disabled = 1 as libc::c_int != 0;
    //coredump_disabled = true;

    //define debug_return;
    debug_return!();
    //end of define;
}

//c(static) rust(没有pub),如何解释？
unsafe extern "C" fn restore_coredump() {
    //define debug_decl(restore_coredump,SUDO_DEBUG_UTIL);
    debug_decl!(restore_coredump, SUDO_DEBUG_UTIL);
    //end of define

    if coredump_disabled {
        if setrlimit(RLIMIT_CORE, &mut corelimit) == -1 {
            //define sudo_warn("setrlimit(RLIMIT_CORE)");
            sudo_debug_printf!(
                SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                b"setrlimit(RLIMIT_CORE)\0" as *const u8 as *const libc::c_char
            );
            sudo_warn_nodebug_v1(b"setrlimit(RLIMIT_CORE)\0" as *const u8 as *const libc::c_char);
            //end of define;
        }
        prctl(PR_SET_DUMPABLE, dumpflag, 0, 0, 0);
    }
    //define debug_return;
    debug_return!();
    //end of define;
}

#[no_mangle]
unsafe extern "C" fn unlimit_nproc() {
    //结构体初始化init返回问题
    let mut rl: rlimit = {
        let mut init = rlimit {
            rlim_cur: RLIM_INFINITY as libc::c_ulong,
            rlim_max: RLIM_INFINITY as libc::c_ulong,
        };
        init
    };
    //define debug_decl(unlimit_nproc,SUDO_DEBUG_UTIL)
    debug_decl!(unlimit_nproc, SUDO_DEBUG_UTIL);
    //end of define

    if getrlimit(RLIMIT_NPROC, &mut nproclimit) != 0 {
        //define sudo_warn("getrlimit(RLIMIT_NPROC)");
        sudo_debug_printf!(
            SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
            b"getrlimit(RLIMIT_NPROC)\0" as *const u8 as *const libc::c_char
        );
        sudo_warn_nodebug_v1(b"getrlimit(RLIMIT_NPROC)\0" as *const u8 as *const libc::c_char);
        //end of define;
    }
    if setrlimit(RLIMIT_NPROC, &mut rl) == -1 {
        //两种方式有何区别？　rl.rlim_cur = rl.rlim_max = nproclimit.rlim_max;
        rl.rlim_max = nproclimit.rlim_max;
        rl.rlim_cur = rl.rlim_max;

        if setrlimit(RLIMIT_NPROC, &mut rl) != 0 {
            //define sudo_warn("setrlimit(RLIMIT_NPROC)");
            sudo_debug_printf!(
                SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
                b"setrlimit(RLIMIT_NPROC)\0" as *const u8 as *const libc::c_char
            );
            sudo_warn_nodebug_v1(b"setrlimit(RLIMIT_NPROC)\0" as *const u8 as *const libc::c_char);
            //end of define
        }
    }
    //define debug_return;
    debug_return!();
    //end of define;
}

#[no_mangle]
unsafe extern "C" fn restore_nproc() {
    //define debug_decl(restore_nproc,SUDO_DEBUG_UTIL)
    debug_decl!(restore_nproc, SUDO_DEBUG_UTIL);
    //end of define
    if setrlimit(RLIMIT_NPROC, &mut nproclimit) != 0 {
        //define sudo_warn("setrlimit(RLIMIT_NPROC)");
        sudo_debug_printf!(
            SUDO_DEBUG_WARN | SUDO_DEBUG_LINENO | SUDO_DEBUG_ERRNO,
            b"setrlimit(RLIMIT_NPROC)\0" as *const u8 as *const libc::c_char
        );
        sudo_warn_nodebug_v1(b"setrlimit(RLIMIT_NPROC)\0" as *const u8 as *const libc::c_char);
        //end of define
    }
    //define debug_return;
    debug_return!();
    //end of define;
}

#[no_mangle]
unsafe extern "C" fn unlimit_sudo() {
    let mut inf: rlimit = {
        let mut init = rlimit {
            rlim_cur: RLIM_INFINITY as libc::c_ulong,
            rlim_max: RLIM_INFINITY as libc::c_ulong,
        };
        init
    };
    let mut idx: libc::c_uint = 0;
    //define debug_decl(unlimit_sudo,SUDO_DEBUG_UTIL)
    debug_decl!(unlimit_sudo, SUDO_DEBUG_UTIL);
    //end of define

    //div,(*lim),continue反着来的代码流程，sizeof函数。
    while (idx as libc::c_ulong)
        < (::std::mem::size_of::<[saved_limit; 8]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<saved_limit>() as libc::c_ulong)
    {
        let mut lim: *mut saved_limit = &mut *saved_limits.as_mut_ptr().offset(idx as isize);
        if !getrlimit((*lim).resource, &mut (*lim).limit) == -1 {
            (*lim).saved = true;
            if setrlimit((*lim).resource, &mut inf) == -1 {
                let mut rl: rlimit = (*lim).limit;
                rl.rlim_cur = rl.rlim_max;
                if setrlimit((*lim).resource, &mut rl) == -1 {
                    //define sudo_warn("setrlimit(%d)",lim->resource);
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
        }
        idx += 1;
    }

    //debug_return;
    debug_return!();
    //end of define;
}

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
