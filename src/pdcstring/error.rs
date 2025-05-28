use std::{
    error::Error,
    fmt::{self, Display},
};

use super::{MissingNulTerminatorInnerImpl, PdUChar, ToStringErrorInner, ToStringErrorInnerImpl};

// same definition as ffi::NulError and widestring::error::ContainsNul<u16>
/// An error returned to indicate that an invalid nul value was found in a string.
#[must_use]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ContainsNul(usize, Vec<PdUChar>);

impl ContainsNul {
    pub(crate) fn new(nul_position: usize, data: Vec<PdUChar>) -> Self {
        Self(nul_position, data)
    }

    /// Returns the position of the nul byte in the slice.
    #[must_use]
    pub fn nul_position(&self) -> usize {
        self.0
    }

    /// Consumes this error, returning the underlying vector of bytes which
    /// generated the error in the first place.
    #[must_use]
    pub fn into_vec(self) -> Vec<PdUChar> {
        self.1
    }
}

#[cfg(not(windows))]
impl From<std::ffi::NulError> for ContainsNul {
    fn from(err: std::ffi::NulError) -> Self {
        Self::new(err.nul_position(), err.into_vec())
    }
}

#[cfg(windows)]
impl From<widestring::error::ContainsNul<PdUChar>> for ContainsNul {
    fn from(err: widestring::error::ContainsNul<PdUChar>) -> Self {
        Self::new(err.nul_position(), err.into_vec().unwrap())
    }
}

impl Error for ContainsNul {
    fn description(&self) -> &'static str {
        "nul value found in data"
    }
}

impl fmt::Display for ContainsNul {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "nul byte found in provided data at position: {}", self.0)
    }
}

impl From<ContainsNul> for Vec<PdUChar> {
    fn from(e: ContainsNul) -> Vec<PdUChar> {
        e.into_vec()
    }
}

// common definition of str::Utf8Error and widestring::error::Utf16Error
/// Errors which can occur when attempting to interpret a sequence of platform-dependent characters as a string.
#[must_use]
#[derive(Clone, Debug)]
pub struct ToStringError(pub(crate) ToStringErrorInnerImpl);

impl ToStringError {
    /// Returns [`Some`]`(index)` in the given string at which the invalid value occurred or
    /// [`None`] if the end of the input was reached unexpectedly.
    #[must_use]
    pub fn index(&self) -> Option<usize> {
        ToStringErrorInner::index(&self.0)
    }
}

impl Display for ToStringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Error for ToStringError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}

/// An error returned from to indicate that a terminating nul value was missing.
#[must_use]
#[derive(Clone, Debug)]
pub struct MissingNulTerminator(pub(crate) MissingNulTerminatorInnerImpl);

impl Display for MissingNulTerminator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Error for MissingNulTerminator {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}
