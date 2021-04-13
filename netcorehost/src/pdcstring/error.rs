use crate::pdcstring::OsChar;
use std::{error::Error, fmt};

// same definition as ffi::NulError and widestring::NulError<u16>
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NulError(usize, Vec<OsChar>);

impl NulError {
    pub fn new(nul_position: usize, data: Vec<OsChar>) -> Self {
        Self(nul_position, data)
    }

    /// Returns the position of the nul byte in the slice that caused
    /// [`PdCString::new`] to fail.
    pub fn nul_position(&self) -> usize {
        self.0
    }

    /// Consumes this error, returning the underlying vector of bytes which
    /// generated the error in the first place.
    pub fn into_vec(self) -> Vec<OsChar> {
        self.1
    }
}

#[cfg(not(windows))]
impl From<std::ffi::NulError> for NulError {
    fn from(err: std::ffi::NulError) -> Self {
        Self::new(err.nul_position(), err.into_vec())
    }
}

#[cfg(windows)]
impl From<widestring::NulError<OsChar>> for NulError {
    fn from(err: widestring::NulError<OsChar>) -> Self {
        Self::new(err.nul_position(), err.into_vec())
    }
}

impl Error for NulError {
    fn description(&self) -> &str {
        "nul value found in data"
    }
}

impl fmt::Display for NulError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "nul byte found in provided data at position: {}", self.0)
    }
}

impl Into<Vec<OsChar>> for NulError {
    fn into(self) -> Vec<OsChar> {
        self.into_vec()
    }
}
