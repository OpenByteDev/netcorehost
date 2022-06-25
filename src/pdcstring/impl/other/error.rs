use crate::pdcstring::{MissingNulTerminatorInner, ToStringErrorInner};

impl ToStringErrorInner for std::str::Utf8Error {
    fn index(&self) -> Option<usize> {
        self.error_len()
    }
}

impl MissingNulTerminatorInner for std::ffi::FromBytesWithNulError {}
