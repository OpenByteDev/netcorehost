use crate::pdcstring::{MissingNulTerminatorInner, ToStringErrorInner};

impl ToStringErrorInner for widestring::error::Utf16Error {
    fn index(&self) -> Option<usize> {
        Some(self.index())
    }
}

impl MissingNulTerminatorInner for widestring::error::MissingNulTerminator {}
