use core::ffi::c_char;

/// `const` friendly memchr function.
#[inline(always)]
#[no_mangle]
pub const fn memchr(needle: u8, haystack: &[u8]) -> Option<usize> {
    let mut index = 0usize;

    while index < haystack.len() {
        if haystack[index] == needle {
            return Some(index);
        }

        index += 1;
    }

    None
}

/// `const` friendly strlen function.
///
/// # Safety
///
/// The caller must ensure that the input pointer is a valid
/// pointer to a null terminated string.
///
/// Additionally the null must be within [`isize::MAX`] bytes from
/// `ptr`.
#[inline(always)]
#[must_use]
pub const unsafe fn strlen(ptr: *const c_char) -> usize {
    let mut len = 0usize;

    while unsafe { *ptr.add(len) } != 0 {
        len += 1;
    }

    len
}
