#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unused_imports,
    unused_macros,
    unused_variables
)]


use crate::macro_struct::*;

// use stdext::function_name;
use crate::sudo_debug::*;

/*
 * The priority and subsystem are encoded in a single 32-bit value.
 * The lower 4 bits are the priority and the top 26 bits are the subsystem.
 * This allows for 16 priorities and a very large number of subsystems.
 * Bit 5 is used as a flag to specify whether to log the errno value.
 * Bit 6 specifies whether to log the function, file and line number data.
 */
/*
 * Sudo debug priorities, ordered least to most verbose,
 * in other words, highest to lowest priority.  Max pri is 15.
 * Note: order must match sudo_debug_priorities[]
 */

pub static mut sudo_debug_subsys: libc::c_int = 0 as libc::c_int;


/*constants*/
pub const SUDO_DEBUG_CRIT: libc::c_int = 1; /* critical errors */
pub const SUDO_DEBUG_ERROR: libc::c_int = 2; /* non-critical errors */
pub const SUDO_DEBUG_WARN: libc::c_int = 3; /* non-fatal warnings */
pub const SUDO_DEBUG_NOTICE: libc::c_int = 4; /* non-error condition notices */
pub const SUDO_DEBUG_DIAG: libc::c_int = 5; /* diagnostic messages */
pub const SUDO_DEBUG_INFO: libc::c_int = 6; /* informational message */
pub const SUDO_DEBUG_TRACE: libc::c_int = 7; /* log function enter/exit */
pub const SUDO_DEBUG_DEBUG: libc::c_int = 8; /* very verbose debugging */
// /* Flag to include string version of errno in debug info. */
pub const SUDO_DEBUG_ERRNO: libc::c_int = 1 << 4;
// /* Flag to include function, file and line number in debug info. */
pub const SUDO_DEBUG_LINENO: libc::c_int = 1 << 5;
// /*
//  * Sudo debug subsystems.
//  * This includes subsystems in the sudoers plugin.
//  * Note: order must match sudo_debug_subsystems[]
//  */
pub const SUDO_DEBUG_ARGS: libc::c_int = 1 << 6; /* command line argument handling */
pub const SUDO_DEBUG_CONV: libc::c_int = 2 << 6; /* user conversation */
pub const SUDO_DEBUG_EDIT: libc::c_int = 3 << 6; /* sudoedit */
pub const SUDO_DEBUG_EVENT: libc::c_int = 4 << 6; /* event handling */
pub const SUDO_DEBUG_EXEC: libc::c_int = 5 << 6; /* command execution */
pub const SUDO_DEBUG_HOOKS: libc::c_int = 6 << 6; /* hook functions */
pub const SUDO_DEBUG_MAIN: libc::c_int = 7 << 6; /* sudo main() */
pub const SUDO_DEBUG_NETIF: libc::c_int = 8 << 6; /* network interface functions */
pub const SUDO_DEBUG_PCOMM: libc::c_int = 9 << 6; /* plugin communications */
pub const SUDO_DEBUG_PLUGIN: libc::c_int = 10 << 6; /* main plugin functions */
pub const SUDO_DEBUG_PTY: libc::c_int = 11 << 6; /* pseudo-tty */
pub const SUDO_DEBUG_SELINUX: libc::c_int = 12 << 6; /* selinux */
pub const SUDO_DEBUG_UTIL: libc::c_int = 13 << 6; /* utility functions */
pub const SUDO_DEBUG_UTMP: libc::c_int = 14 << 6; /* utmp file ops */

macro_rules! SUDO_DEBUG_ALL {
    /* all subsystems */
    () => {
        0xffff0000
    };
}

// /* Error return for sudo_debug_register().  */
pub const SUDO_DEBUG_INSTANCE_ERROR: libc::c_int = -2;
// /* Initializer for instance index to indicate that debugging is not setup. */
// #define 	-1
pub const SUDO_DEBUG_INSTANCE_INITIALIZER: libc::c_int = -1;

#[macro_export]
macro_rules! debug_decl_func {
    ($funcname:expr) => {};
}

#[macro_export]
macro_rules! debug_decl_vars {
    ($funcname:expr, $subsys:expr) => {
        sudo_debug_subsys = $subsys
    };
}

#[macro_export]
macro_rules! debug_decl {
    ($funcname:expr, $subsys:expr) => {
        debug_decl_vars!($funcname, $subsys);
        sudo_debug_enter_v1(
            $funcname as *const libc::c_char,
            file!().as_ptr() as *const libc::c_char,
            line!() as libc::c_int,
            sudo_debug_subsys as libc::c_int,
        );
    };
}

// 完成
#[macro_export]
macro_rules! debug_return_int {
    ($ret:expr) => {{
        sudo_debug_exit_int_v1(
            stdext::function_name!().as_ptr() as *const libc::c_char,
            file!().as_ptr() as *const libc::c_char,
            line!() as libc::c_int,
            sudo_debug_subsys as libc::c_int,
            $ret,
        );
        return $ret;
    }};
}


// 完成
#[macro_export]
macro_rules! debug_return_id_t {
    ($ret:expr) => {{
        sudo_debug_exit_id_t_v1(
            stdext::function_name!().as_ptr() as *const libc::c_char,
            file!().as_ptr() as *const libc::c_char,
            line!() as libc::c_int,
            sudo_debug_subsys as libc::c_int,
            $ret,
        );
        return $ret;
    }};
}

#[macro_export]
macro_rules! debug_return_size_t {
    ($ret:expr) => {
        sudo_debug_exit_size_t(function_name!(), file!(), line!(), sudo_debug_subsys, $ret);
        return $ret;
    };
}

#[macro_export]
macro_rules! debug_return_ssize_t {
    ($ret:expr) => {{
        sudo_debug_exit_ssize_t_v1(
            stdext::function_name!().as_ptr() as *const libc::c_char,
            file!().as_ptr() as *const libc::c_char,
            line!() as libc::c_int,
            sudo_debug_subsys as libc::c_int,
            $ret,
        );
        return $ret;
    }};
}

#[macro_export]
macro_rules! debug_return_time_t {
    ($ret:expr) => {
        sudo_debug_exit_time_t(function_name!(), file!(), line!(), sudo_debug_subsys, $ret);
        return $ret;
    };
}


#[macro_export]
macro_rules! debug_return_long {
    ($ret:expr) => {
        sudo_debug_exit_long(function_name!(), file!(), line!(), sudo_debug_subsys, $ret);
        return $ret;
    };
}

// 完成
#[macro_export]
macro_rules! debug_return_bool {
    ($ret:expr) => {{
        sudo_debug_exit_bool_v1(
            stdext::function_name!().as_ptr() as *const libc::c_char,
            file!().as_ptr() as *const libc::c_char,
            line!() as libc::c_int,
            sudo_debug_subsys as libc::c_int,
            $ret,
        );
        return $ret;
    }};
}










