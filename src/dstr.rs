//! Module for borrowed nul-terminated UTF-8 strings.

use core::{
    ffi::{c_char, CStr},
    num::NonZeroUsize,
    slice::{from_raw_parts, from_raw_parts_mut},
    str::{from_utf8_unchecked, from_utf8_unchecked_mut},
};

mod error;
pub use error::*;

/// A nul-terminated UTF-8 string.
///
/// # Representation
///
/// References and raw pointers to a [`DStr`] are **fat pointers**.
///
/// Rather than computing `strlen` everywhere where a [`str`], [`[u8]`], etc.,
/// is required, it makes more sense to just store the length with the pointer.
///
/// Since [`DStr`] is guaranteed to be nul-terminated, it is trivial to get a
/// thin pointer from a [`DStr`].
///
/// # Safety
///
/// - The internal representation must always be valid UTF-8.
///
/// - There must be no internal nuls.
///   In practice this means:
///
///    - There must not be any nul (zero) bytes anywhere before the last byte
///      within the underlying [`str`] / [`[u8]`].
///
///    - The location of the nul is **always the last byte** in the
///      underlying [`str`] / [`[u8]`].
///
/// - The string is nul-terminated.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DStr {
    raw: str,
}

impl DStr {
    // pub const EMPTY: &'static DStr = DStr::from_c_str(c"");
}

impl DStr {
    /// Returns the length of this [`DStr`] in bytes, including the
    /// nul terminator.
    ///
    /// This is equivalent to `self.as_bytes_with_nul().len()` or
    /// `strlen(self.as_c_ptr()) + 1`.
    #[inline]
    #[must_use]
    pub const fn len_with_nul(&self) -> NonZeroUsize {
        debug_assert!(
            !self.raw.is_empty(),
            "a nul-terminated string must have at least a nul"
        );

        // SAFETY: A nul-terminated string always has at least one byte, the nul.
        //         if the length of `raw` is zero, something horrible has gone wrong in
        //         the construction of a [`DStr`].
        unsafe { NonZeroUsize::new_unchecked(self.raw.len()) }
    }

    /// Returns the length of this [`DStr`] in bytes, excluding the
    /// nul terminator.
    ///
    /// This is equivalent to `self.as_bytes().len()`, or `strlen(self.as_c_ptr())`.
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize {
        // We want to preserve our fancy error message :3
        self.len_with_nul().get() - 1
    }

    /// Returns whether this [`DStr`] is empty.
    ///
    /// **Note**: This returns whether `self.as_bytes()` has a length of zero.
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        debug_assert!(
            !self.raw.is_empty(),
            "a nul-terminated string must have at least a nul"
        );

        // SAFETY: A nul-terminated string always contains at least one byte, the nul.
        //         So it is always safe to read from `raw[0]`.
        unsafe { *self.raw.as_ptr() == 0 }
    }

    /// Converts this [`DStr`] to a `&[u8]` containing the nul
    /// terminator.
    #[inline]
    #[must_use]
    pub const fn as_bytes_with_nul(&self) -> &[u8] {
        self.raw.as_bytes()
    }

    /// Converts this [`DStr`] to a `&mut [u8]` containing the nul
    /// terminator.
    ///
    /// # Safety
    ///
    /// Before the borrow ends the caller must ensure the following:
    ///
    /// - The content of the slice is valid UTF-8.
    ///
    /// - The slice contains no interior nuls.
    ///
    /// - The slice is nul-terminated.
    #[inline]
    #[must_use]
    pub unsafe fn as_bytes_with_nul_mut(&mut self) -> &mut [u8] {
        unsafe { self.raw.as_bytes_mut() }
    }

    /// Converts this [`DStr`] to a `&[u8]` excluding the nul
    /// terminator.
    #[inline]
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
        let len = self.len();
        let ptr = self.raw.as_ptr();

        unsafe { from_raw_parts(ptr, len) }
    }

    /// Converts this [`DStr`] to a `&mut [u8]` excluding the nul
    /// terminator.
    ///
    /// # Safety
    ///
    /// Before the borrow ends the caller must ensure the following:
    ///
    /// - The content of the slice is valid UTF-8.
    ///
    /// - The slice contains no nuls.
    #[inline]
    #[must_use]
    pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        let len = self.len();
        let ptr = self.raw.as_mut_ptr();

        unsafe { from_raw_parts_mut(ptr, len) }
    }

    /// Converts this [`DStr`] to a `&str` containing the nul
    /// terminator.
    #[inline]
    #[must_use]
    pub const fn as_str_with_nul(&self) -> &str {
        &self.raw
    }

    /// Converts this [`DStr`] to a `&mut str` containing the nul
    /// terminator.
    ///
    /// # Safety
    ///
    /// Before the borrow ends the caller must ensure the following:
    ///
    /// - The string contains no interior nuls.
    ///
    /// - The string is nul-terminated.
    #[inline]
    #[must_use]
    pub unsafe fn as_str_with_nul_mut(&mut self) -> &mut str {
        #[allow(unused_unsafe)]
        unsafe {
            &mut self.raw
        }
    }

    /// Converts this [`DStr`] to a `&str` excluding the nul
    /// terminator.
    #[inline]
    #[must_use]
    pub const fn as_str(&self) -> &str {
        unsafe { from_utf8_unchecked(self.as_bytes()) }
    }

    /// Converts this [`DStr`] to a `&mut str` excluding the nul
    /// terminator.
    ///
    /// # Safety
    ///    
    /// Before the borrow ends the caller must ensure the following:
    ///
    /// - The string contains no nuls.
    #[inline]
    #[must_use]
    pub unsafe fn as_str_mut(&mut self) -> &mut str {
        unsafe { from_utf8_unchecked_mut(self.as_bytes_mut()) }
    }

    /// Converts this [`DStr`] to a `&CStr`.
    #[inline]
    #[must_use]
    pub const fn as_cstr(&self) -> &CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(self.as_bytes_with_nul()) }
    }

    /// Converts this [`DStr`] to a raw pointer.
    ///
    /// The returned pointer points to the first byte in the string.
    #[inline]
    #[must_use]
    pub const fn as_ptr(&self) -> *const u8 {
        self.raw.as_ptr()
    }

    /// Converts this [`DStr`] to a raw mutable pointer.
    ///
    /// The returned pointer points to the first byte in the string.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the invariants of [`DStr`] are
    /// not violated.
    ///
    /// See [`DStr`] for more info.
    #[inline]
    #[must_use]
    pub fn as_ptr_mut(&mut self) -> *mut u8 {
        self.raw.as_mut_ptr()
    }

    /// Converts this [`DStr`] to a raw C string pointer.
    ///
    /// The returned pointer points to the first byte in the string.
    #[inline]
    #[must_use]
    pub const fn as_c_ptr(&self) -> *const c_char {
        self.as_ptr() as *const c_char
    }

    /// Converts this [`DStr`] to a raw mutable C string pointer.
    ///
    /// The returned pointer points to the first byte in the string.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the invariants of [`DStr`] are
    /// not violated.
    ///
    /// See [`DStr`] for more info.
    #[inline]
    #[must_use]
    pub fn as_c_ptr_mut(&mut self) -> *mut c_char {
        self.as_ptr() as *mut c_char
    }

    /// Create a [`DStr`] from a nul-terminated string without
    /// doing any checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the input is a nul-terminated string
    /// with no interior nuls.
    #[inline]
    #[must_use]
    pub const unsafe fn from_str_with_nul_unchecked(string: &str) -> &DStr {
        unsafe { &*(string as *const str as *const DStr) }
    }

    /// Create a [`DStr`] from a mutable nul-terminated string without
    /// doing any checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the input is a nul-terminated string
    /// with no interior nuls.
    #[inline]
    #[must_use]
    pub unsafe fn from_str_with_nul_unchecked_mut(string: &mut str) -> &mut DStr {
        unsafe { &mut *(string as *mut str as *mut DStr) }
    }

    /// Create a [`DStr`] from a nul-terminated UTF-8 byte slice without
    /// doing any checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the input is a nul-terminated UTF-8
    /// byte slice with no interior nuls.
    #[inline]
    #[must_use]
    pub const unsafe fn from_bytes_with_nul_unchecked(bytes: &[u8]) -> &DStr {
        unsafe { &*(bytes as *const [u8] as *const DStr) }
    }

    /// Create a [`DStr`] from a mutable nul-terminated UTF-8 byte slice without
    /// doing any checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the input is a nul-terminated UTF-8
    /// byte slice with no interior nuls.
    #[inline]
    #[must_use]
    pub unsafe fn from_bytes_with_nul_unchecked_mut(bytes: &mut [u8]) -> &mut DStr {
        unsafe { &mut *(bytes as *mut [u8] as *mut DStr) }
    }
}
