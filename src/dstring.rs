use core::ops::{Deref, DerefMut};

use alloc::{borrow::ToOwned, string::String};

use crate::DStr;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DString {
    inner: String,
}

impl DString {
    #[inline]
    #[must_use]
    pub fn as_dstr(&self) -> &DStr {
        // unsafe { DStr::from_str_with_nul_unchecked(&self.inner) }
        todo!()
    }

    #[inline]
    #[must_use]
    pub fn as_dstr_mut(&mut self) -> &mut DStr {
        // unsafe { DStr::from_str_with_nul_unchecked_mut(&mut self.inner) }
        todo!()
    }
}

impl Deref for DString {
    type Target = DStr;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_dstr()
    }
}

impl DerefMut for DString {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_dstr_mut()
    }
}

impl From<&DStr> for DString {
    #[inline]
    fn from(value: &DStr) -> Self {
        DString {
            inner: value.as_str_with_nul().to_owned(),
        }
    }
}

impl From<&mut DStr> for DString {
    #[inline]
    fn from(value: &mut DStr) -> Self {
        DString {
            inner: value.as_str_with_nul().to_owned(),
        }
    }
}
