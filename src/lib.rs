//! # Dumstr
//!
//! A small, dumb set of tools for UTF-8 null terminated strings.

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub(crate) mod mem;

pub mod dstr;

#[doc(inline)]
pub use dstr::DStr;

#[cfg(feature = "alloc")]
pub mod dstring;

#[cfg(feature = "alloc")]
#[doc(inline)]
pub use dstring::DString;
