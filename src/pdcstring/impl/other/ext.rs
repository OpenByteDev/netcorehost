use std::ffi::{CStr, CString};

use crate::pdcstring::{PdCStr, PdCString};

pub trait PdCStringExt
where
    Self: Sized,
{
    fn from_c_string(s: CString) -> Self;
    fn into_c_string(self) -> CString;
}

impl PdCStringExt for PdCString {
    fn from_c_string(s: CString) -> Self {
        Self::from_inner(s)
    }

    fn into_c_string(self) -> CString {
        self.into_inner()
    }
}

pub trait PdCStrExt {
    fn from_c_str(s: &CStr) -> &Self;
    fn as_c_str(&self) -> &CStr;
}

impl PdCStrExt for PdCStr {
    fn from_c_str(s: &CStr) -> &Self {
        Self::from_inner(s)
    }

    fn as_c_str(&self) -> &CStr {
        self.as_inner()
    }
}
