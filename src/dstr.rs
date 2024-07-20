use core::{
    ffi::{c_char, CStr},
    num::NonZeroUsize,
    slice::{from_raw_parts, from_raw_parts_mut},
    str::{from_utf8, from_utf8_unchecked, from_utf8_unchecked_mut},
};

use crate::util;

/// A nul-terminated UTF-8 string.
#[repr(transparent)]
pub struct DStr {
    bytes: [u8],
}

impl DStr {
    /// Returns the length of this [`DStr`] in bytes, including the
    /// nul terminator.
    #[inline]
    #[must_use]
    pub const fn len_with_nul(&self) -> NonZeroUsize {
        debug_assert!(
            !self.bytes.is_empty(),
            "a nul-terminated string must have at least a nul"
        );

        // SAFETY: A nul-terminated string always has at least one byte, the nul.
        //         if the length of `bytes` is zero, something horrible has gone wrong in
        //         the construction of a [`DStr`].
        unsafe { NonZeroUsize::new_unchecked(self.bytes.len()) }
    }

    /// Returns the length of this [`DStr`] in bytes, excluding the
    /// nul terminator.
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize {
        // We want to preserve our fancy error message :3
        self.len_with_nul().get() - 1
    }

    /// Returns whether this [`DStr`] is empty.
    ///
    /// **Note**: This returns whether `self.as_bytes()` as a length of zero.
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        debug_assert!(
            !self.bytes.is_empty(),
            "a nul-terminated string must have at least a nul"
        );

        // SAFETY: A nul-terminated string always contains at least one byte, the nul.
        //         So it is always safe to read from `bytes[0]`.
        unsafe { *self.bytes.as_ptr() == 0 }
    }

    /// Converts this [`DStr`] to a `&[u8]` containing the nul
    /// terminator.
    #[inline]
    #[must_use]
    pub const fn as_bytes_with_nul(&self) -> &[u8] {
        &self.bytes
    }

    /// Converts this [`DStr`] to a `&[u8]` excluding the nul
    /// terminator.
    #[inline]
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
        unsafe { from_raw_parts(self.bytes.as_ptr(), self.len()) }
    }

    /// Converts this [`DStr`] to a `&mut [u8]` containing the nul
    /// terminator.
    ///
    /// # Safety
    ///
    /// Before the borrow ends the caller must ensure the following:
    ///
    /// - The content of the slice is valid UTF-8.
    /// - The slice contains no interior nuls.
    /// - The slice is nul-terminated.
    ///
    /// Failure to uphold this contract is undefined behavior.
    #[inline]
    #[must_use]
    pub unsafe fn as_bytes_with_nul_mut(&mut self) -> &mut [u8] {
        &mut self.bytes
    }

    /// Converts this [`DStr`] to a `&mut [u8]` excluding the nul
    /// terminator.
    ///
    /// # Safety
    ///
    /// Before the borrow ends the caller must ensure the following:
    ///
    /// - The content of the slice is valid UTF-8.
    /// - The slice contains no nuls.
    ///
    /// Failure to uphold this contract is undefined behavior.
    #[inline]
    #[must_use]
    pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        let len = self.len();
        let ptr = self.bytes.as_mut_ptr();

        unsafe { from_raw_parts_mut(ptr, len) }
    }

    /// Converts this [`DStr`] to a `&str` containing the nul
    /// terminator.
    #[inline]
    #[must_use]
    pub const fn as_str_with_nul(&self) -> &str {
        debug_assert!(
            from_utf8(self.as_bytes_with_nul()).is_ok(),
            "string is not valid utf-8"
        );

        unsafe { from_utf8_unchecked(self.as_bytes_with_nul()) }
    }

    /// Converts this [`DStr`] to a `&str` excluding the nul
    /// terminator.
    #[inline]
    #[must_use]
    pub const fn as_str(&self) -> &str {
        debug_assert!(
            from_utf8(self.as_bytes()).is_ok(),
            "string is not valid utf-8"
        );

        unsafe { from_utf8_unchecked(self.as_bytes()) }
    }

    /// Converts this [`DStr`] to a `&mut str` containing the nul
    /// terminator.
    ///
    /// # Safety
    ///
    /// Before the borrow ends the caller must ensure the following:
    ///
    /// - The string contains no interior nuls.
    /// - The string is nul-terminated.
    #[inline]
    #[must_use]
    pub unsafe fn as_str_with_nul_mut(&mut self) -> &mut str {
        debug_assert!(
            from_utf8(self.as_bytes_with_nul()).is_ok(),
            "string is not valid utf-8"
        );

        unsafe { from_utf8_unchecked_mut(self.as_bytes_with_nul_mut()) }
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
        debug_assert!(
            from_utf8(self.as_bytes()).is_ok(),
            "string is not valid utf-8"
        );

        unsafe { from_utf8_unchecked_mut(self.as_bytes_mut()) }
    }

    /// Converts this [`DStr`] to a `&CStr`.
    #[inline]
    #[must_use]
    pub const fn as_cstr(&self) -> &CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(self.as_bytes_with_nul()) }
    }

    /// Converts this [`DStr`] to a raw pointer.
    #[inline]
    #[must_use]
    pub const fn as_ptr(&self) -> *const u8 {
        self.bytes.as_ptr()
    }

    /// Converts this [`DStr`] to a raw mutable pointer.
    #[inline]
    #[must_use]
    pub fn as_ptr_mut(&mut self) -> *mut u8 {
        self.bytes.as_mut_ptr()
    }

    /// Converts this [`DStr`] to a raw C string pointer.
    #[inline]
    #[must_use]
    pub const fn as_c_ptr(&self) -> *const c_char {
        self.as_ptr() as *const c_char
    }

    /// Converts this [`DStr`] to a raw mutable C string pointer.
    #[inline]
    #[must_use]
    pub fn as_c_ptr_mut(&mut self) -> *mut c_char {
        self.as_ptr() as *mut c_char
    }
}

impl DStr {
    #[inline]
    #[must_use]
    pub const unsafe fn from_bytes_with_nul_unchecked(bytes: &[u8]) -> &DStr {
        unsafe { &*(bytes as *const [u8] as *const DStr) }
    }

    #[inline]
    #[must_use]
    pub unsafe fn from_bytes_with_nul_unchecked_mut(bytes: &mut [u8]) -> &mut DStr {
        unsafe { &mut *(bytes as *mut [u8] as *mut DStr) }
    }

    #[inline]
    #[must_use]
    pub const unsafe fn from_raw<'a>(ptr: *const c_char) -> &'a DStr {
        unsafe {
            DStr::from_bytes_with_nul_unchecked(from_raw_parts(
                ptr as *const u8,
                util::strlen(ptr) + 1,
            ))
        }
    }

    #[inline]
    #[must_use]
    pub unsafe fn from_raw_mut<'a>(ptr: *mut c_char) -> &'a mut DStr {
        unsafe {
            DStr::from_bytes_with_nul_unchecked_mut(from_raw_parts_mut(
                ptr as *mut u8,
                util::strlen(ptr) + 1,
            ))
        }
    }
}
