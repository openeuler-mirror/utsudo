
#![warn(missing_docs, unreachable_pub)]
pub mod duration;
#[macro_use]
pub mod macros;
pub mod num;
pub mod option;
pub mod result;
pub mod str;
pub mod sync;
pub mod vec;
/// A "prelude" module which re-exports all the extension traits for the simple library usage.
pub mod prelude {
    pub use crate::{
        duration::*,
        num::{float_convert::*, integer::*},
        option::*,
        result::*,
        str::*,
        sync::{mutex::*, rw_lock::*},
        vec::*,
    };
}
