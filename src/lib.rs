//! # DStr
//!
//! A small, dumb set of tools for UTF-8 null terminated strings.

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub(crate) mod dstr;
pub(crate) mod util;

#[doc(inline)]
pub use dstr::DStr;
