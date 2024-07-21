use core::{fmt, str::Utf8Error};

// A workaround so that I don't need to pull in `const-panic` or similar crates.
macro_rules! msg {
    (NotNulTerminated) => {
        "input was not nul-terminated"
    };
    (InteriorNul) => {
        "input contains an interior nul"
    };
    (MissingNul) => {
        "input does not contain a nul"
    };
    (InvalidUtf8) => {
        "input contains invalid utf-8"
    };
    ($($tt:tt)*) => {
        compile_error!("unrecognized error message")
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FromStrError {
    NotNulTerminated,
    InteriorNul(usize),
    MissingNul,
}

impl FromStrError {
    #[inline]
    #[must_use]
    pub const fn message(self) -> &'static str {
        match self {
            FromStrError::NotNulTerminated => msg!(NotNulTerminated),
            FromStrError::InteriorNul(_) => msg!(InteriorNul),
            FromStrError::MissingNul => msg!(MissingNul),
        }
    }

    #[inline(always)]
    pub(crate) const fn _panic<T>(self) -> T {
        panic!("{}", self.message())
    }

    #[inline]
    #[must_use]
    pub(crate) const fn to_from_bytes_error(self) -> FromBytesError {
        match self {
            FromStrError::NotNulTerminated => FromBytesError::NotNulTerminated,
            FromStrError::InteriorNul(nul_pos) => FromBytesError::InteriorNul(nul_pos),
            FromStrError::MissingNul => FromBytesError::MissingNul,
        }
    }
}

impl fmt::Display for FromStrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.message())?;

        if let &Self::InteriorNul(pos) = self {
            core::write!(f, " at pos {pos}")?;
        }

        Ok(())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for FromStrError {
    #[allow(deprecated)]
    #[inline]
    fn description(&self) -> &str {
        self.message()
    }
}

impl PartialEq<FromBytesError> for FromStrError {
    fn eq(&self, other: &FromBytesError) -> bool {
        match Self::try_from(*other) {
            Ok(other) => self.eq(&other),
            Err(_) => false,
        }
    }
}

impl TryFrom<FromBytesError> for FromStrError {
    type Error = ();

    #[inline]
    fn try_from(value: FromBytesError) -> Result<Self, Self::Error> {
        match value {
            FromBytesError::NotNulTerminated => Ok(Self::NotNulTerminated),
            FromBytesError::InteriorNul(nul_pos) => Ok(Self::InteriorNul(nul_pos)),
            FromBytesError::MissingNul => Ok(Self::MissingNul),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FromBytesError {
    NotNulTerminated,
    InteriorNul(usize),
    MissingNul,
    InvalidUtf8(Utf8Error),
}

impl FromBytesError {
    #[inline]
    #[must_use]
    pub const fn message(self) -> &'static str {
        match self {
            FromBytesError::NotNulTerminated => msg!(NotNulTerminated),
            FromBytesError::InteriorNul(_) => msg!(InteriorNul),
            FromBytesError::MissingNul => msg!(MissingNul),
            FromBytesError::InvalidUtf8(_) => msg!(InvalidUtf8),
        }
    }

    #[inline(always)]
    pub(crate) const fn _panic<T>(self) -> T {
        panic!("{}", self.message())
    }
}

impl fmt::Display for FromBytesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.message())?;

        if let &Self::InteriorNul(pos) = self {
            core::write!(f, " at pos {pos}")?;
        }

        Ok(())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for FromBytesError {
    #[inline]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FromBytesError::InvalidUtf8(err) => Some(err),
            _ => None,
        }
    }

    #[allow(deprecated)]
    #[inline]
    fn description(&self) -> &str {
        self.message()
    }
}

impl From<FromStrError> for FromBytesError {
    #[inline]
    fn from(value: FromStrError) -> Self {
        value.to_from_bytes_error()
    }
}

impl PartialEq<FromStrError> for FromBytesError {
    fn eq(&self, other: &FromStrError) -> bool {
        self.eq(&Self::from(*other))
    }
}
