use std::{error::Error, ffi, fmt};

#[derive(Debug)]
pub struct NulError {}

impl From<ffi::NulError> for NulError {
    fn from(err: ffi::NulError) -> Self {
        NulError {}
    }
}

#[cfg(windows)]
impl From<widestring::NulError<u16>> for NulError {
    fn from(err: widestring::NulError<u16>) -> Self {
        NulError {}
    }
}

impl Error for NulError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl fmt::Display for NulError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "") // TODO
    }
}
