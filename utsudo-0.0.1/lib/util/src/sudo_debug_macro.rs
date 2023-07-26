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

pub static mut sudo_debug_subsys: libc::c_int = 0 as libc::c_int;
// /* Error return for sudo_debug_register().  */
pub const SUDO_DEBUG_INSTANCE_ERROR: libc::c_int = -2;
// /* Initializer for instance index to indicate that debugging is not setup. */
// #define 	-1
pub const SUDO_DEBUG_INSTANCE_INITIALIZER: libc::c_int = -1;

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

#[macro_export]
macro_rules! debug_return_str {
    ($ret:expr) => {{
        sudo_debug_exit_str_v1(
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
macro_rules! debug_return_const_str {
    ($ret:expr) => {
        sudo_debug_exit_str(function_name!(), file!(), line!(), sudo_debug_subsys, $ret);
        return $ret;
    };
}

#[macro_export]
macro_rules! debug_return_str_masked {
    ($ret:expr) => {{
        sudo_debug_exit_str_masked_v1(
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
macro_rules! debug_return_ptr {
    ($ret:expr) => {{
        sudo_debug_exit_ptr_v1(
            stdext::function_name!().as_ptr() as *const libc::c_char,
            file!().as_ptr() as *const libc::c_char,
            line!() as libc::c_int,
            sudo_debug_subsys as libc::c_int,
            $ret as *const libc::c_void,
        );
        return $ret;
    }};
}

macro_rules! debug_return_const_ptr {
    ($ret:expr) => {{
        sudo_debug_exit_ptr_v1(
            stdext::function_name!().as_ptr() as *const libc::c_char,
            file!().as_ptr() as *const libc::c_char,
            line!() as libc::c_int,
            sudo_debug_subsys as libc::c_int,
            $ret as *const libc::c_void,
        );
        return $ret;
    }};
}

#[macro_export]
macro_rules! sudo_debug_execve {
    ($pri:expr, $path:expr, $argv:expr, $envp:expr) => {{
        sudo_debug_execve2_v1(
            $pri | sudo_debug_subsys as libc::c_int,
            $path as *const libc::c_char,
            $argv as *const *mut libc::c_char,
            $envp as *const *mut libc::c_char,
        );
    }};
}

macro_rules! sudo_debug_write {
    ($ret:expr) => {};
}

macro_rules! sudo_debug_write {
    ($fd:expr, $str:expr, $len:expr, $errnum:expr) => {{
        sudo_debug_write2_v1(
            $fd as libc::c_int,
            0 as *const libc::c_char,
            0 as *const libc::c_char,
            0 as libc::c_int,
            $str as *const libc::c_char,
            $len as libc::c_int,
            $errnum as libc::c_int,
        );
    }};
}

// # define sudo_debug_printf(pri, ...) \
//     sudo_debug_printf2(__func__, __FILE__, __LINE__, (pri)|sudo_debug_subsys, \
//     __VA_ARGS__)
// #endif
#[macro_export]
macro_rules! sudo_debug_printf {
    ($pri:expr, $($arg:tt)*) => {{
        sudo_debug_printf2_v1(
        stdext::function_name!().as_ptr() as *const libc::c_char,
        file!().as_ptr() as *const libc::c_char,
        line!() as libc::c_int,
        ($pri | sudo_debug_subsys) as libc::c_int,
        $($arg)*,
        );
    }};
}

#[macro_export]
macro_rules! debug_return {
    () => {{
        sudo_debug_exit_v1(
            stdext::function_name!().as_ptr() as *const libc::c_char,
            file!().as_ptr() as *const libc::c_char,
            line!() as libc::c_int,
            sudo_debug_subsys as libc::c_int,
        );
        return;
    }};
}

// #  define sudo_warn(fmt...) do {					       \
//     sudo_debug_printf2(__func__, __FILE__, __LINE__,			       \
// 	SUDO_DEBUG_ERROR|SUDO_DEBUG_LINENO|SUDO_DEBUG_ERRNO|sudo_debug_subsys, \
// 	fmt);								       \
//     sudo_warn_nodebug_v1(fmt);
// } while (0)
#[macro_export]
macro_rules! sudo_warn {
    ($fmt:expr, $($arg:tt)*) => {{
        sudo_debug_printf2_v1(
            stdext::function_name!().as_ptr() as *const libc::c_char,
            file!().as_ptr() as *const libc::c_char,
            line!() as libc::c_int,
            SUDO_DEBUG_ERROR|SUDO_DEBUG_LINENO|SUDO_DEBUG_ERRNO|sudo_debug_subsys,
            $fmt,
            $($arg)*
            );
        sudo_warn_nodebug_v1($fmt,  $($arg)*);
    }};
}

// sudo_warnx(fmt...) do {					       \
//     sudo_debug_printf2(__func__, __FILE__, __LINE__,			       \
// 	SUDO_DEBUG_ERROR|SUDO_DEBUG_LINENO|sudo_debug_subsys, fmt);	       \
//     sudo_warnx_nodebug_v1(fmt);
// } while (0)
#[macro_export]
macro_rules! sudo_warnx {
    ($fmt:expr, $($arg:tt)*) => {{
        sudo_debug_printf2_v1(
            stdext::function_name!().as_ptr() as *const libc::c_char,
            file!().as_ptr() as *const libc::c_char,
            line!() as libc::c_int,
            SUDO_DEBUG_ERROR|SUDO_DEBUG_LINENO|sudo_debug_subsys,
            $fmt,
            $($arg)*
            );
        sudo_warnx_nodebug_v1(
            $fmt,
            $($arg)*
        );
    }};
}

// #  define sudo_fatalx(fmt...) do {					       \
//     sudo_debug_printf2(__func__, __FILE__, __LINE__,			       \
// 	SUDO_DEBUG_ERROR|SUDO_DEBUG_LINENO|sudo_debug_subsys, fmt);	       \
//     sudo_fatalx_nodebug_v1(fmt);					       \
// } while (0)
#[macro_export]
macro_rules! sudo_fatalx {
    ($fmt:expr, $($arg:tt)*) => {{
        sudo_debug_printf2_v1(
            stdext::function_name!().as_ptr() as *const libc::c_char,
            file!().as_ptr() as *const libc::c_char,
            line!() as libc::c_int,
            SUDO_DEBUG_ERROR|SUDO_DEBUG_LINENO|sudo_debug_subsys,
            $fmt,
            $($arg)*
            );
        sudo_fatalx_nodebug_v1(
            $fmt,
            $($arg)*
        );
    }};
}

// #  define sudo_fatal(fmt...) do {					       \
//     sudo_debug_printf2(__func__, __FILE__, __LINE__,			       \
// 	SUDO_DEBUG_ERROR|SUDO_DEBUG_LINENO|SUDO_DEBUG_ERRNO|sudo_debug_subsys, \
// 	fmt);								       \
//     sudo_fatal_nodebug_v1(fmt);
// } while (0)
#[macro_export]
macro_rules! sudo_fatal {
    ($fmt:expr, $($arg:tt)*) => {{
        sudo_debug_printf2_v1(
            stdext::function_name!().as_ptr() as *const libc::c_char,
            file!().as_ptr() as *const libc::c_char,
            line!() as libc::c_int,
            SUDO_DEBUG_ERROR|SUDO_DEBUG_LINENO|SUDO_DEBUG_ERRNO|sudo_debug_subsys,
            $fmt,
            $($arg)*
            );
        sudo_fatal_nodebug_v1($fmt,  $($arg)*);
    }};
}
