use widestring::{U16CStr, U16CString};

use crate::pdcstring::{PdCStr, PdCString};

pub trait PdCStringExt
where
    Self: Sized,
{
    fn from_u16_c_string(s: U16CString) -> Self;
    fn into_u16_c_string(self) -> U16CString;
}

impl PdCStringExt for PdCString {
    fn from_u16_c_string(s: U16CString) -> Self {
        Self::from_inner(s)
    }

    fn into_u16_c_string(self) -> U16CString {
        self.into_inner()
    }
}

pub trait PdCStrExt {
    fn from_u16_c_str(s: &U16CStr) -> &Self;
    fn as_u16_c_str(&self) -> &U16CStr;
}

impl PdCStrExt for PdCStr {
    fn from_u16_c_str(s: &U16CStr) -> &Self {
        Self::from_inner(s)
    }

    fn as_u16_c_str(&self) -> &U16CStr {
        self.as_inner()
    }
}
