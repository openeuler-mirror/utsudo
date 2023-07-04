#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
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
