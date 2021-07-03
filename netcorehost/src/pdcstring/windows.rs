use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::OsStrExt;

use widestring::{MissingNulError, U16CStr, U16CString};

use super::{NulError, PdCStr, PdCString};

pub(crate) type PdCStringInner = U16CString;
pub(crate) type PdCStrInner = U16CStr;

// conversions to and from inner
impl PdCString {
    pub fn from_u16_c_string(s: U16CString) -> Self {
        PdCString::from_inner(s)
    }
    pub fn into_u16_c_string(self) -> U16CString {
        self.into_inner()
    }
}

// methods used by this crate
impl PdCString {
    pub fn from_os_str<T: AsRef<OsStr>>(s: T) -> Result<Self, NulError> {
        let inner = U16CString::from_os_str(s)?;
        Ok(PdCString::from_u16_c_string(inner))
    }
    pub fn from_str(s: &str) -> Result<Self, NulError> {
        let inner = U16CString::from_str(s)?;
        Ok(PdCString::from_u16_c_string(inner))
    }
    pub unsafe fn from_str_ptr(ptr: *const u16) -> Self {
        let inner = U16CString::from_ptr_str(ptr);
        PdCString::from_u16_c_string(inner)
    }
}

// methods not used by this crate
impl PdCString {
    pub fn from_vec(vec: impl Into<Vec<u16>>) -> Result<Self, NulError> {
        let inner = U16CString::new(vec)?;
        Ok(PdCString::from_inner(inner))
    }
}

// conversions to and from inner
impl PdCStr {
    pub fn from_u16_c_str(s: &U16CStr) -> &Self {
        PdCStr::from_inner(s)
    }
    pub fn to_u16_c_str(&self) -> &U16CStr {
        self.to_inner()
    }
}

// methods used by this crate
impl PdCStr {
    pub fn as_ptr(&self) -> *const u16 {
        self.0.as_ptr()
    }
    pub unsafe fn from_str_ptr<'a>(ptr: *const u16) -> &'a Self {
        let inner = U16CStr::from_ptr_str(ptr);
        PdCStr::from_inner(inner)
    }
    pub unsafe fn from_slice_with_nul_unchecked(slice: &[u16]) -> &Self {
        let inner = U16CStr::from_slice_with_nul_unchecked(slice);
        PdCStr::from_inner(inner)
    }
    pub fn to_os_string(&self) -> OsString {
        self.0.to_os_string()
    }
}

// methods not used by this crate
impl PdCStr {
    // TODO: use abstract error type
    pub fn from_slice_with_nul(slice: &[u16]) -> Result<&Self, MissingNulError<u16>> {
        U16CStr::from_slice_with_nul(slice).map(|s| PdCStr::from_inner(s))
    }
    pub fn to_slice(&self) -> &[u16] {
        self.0.as_slice()
    }
    pub fn to_slice_with_nul(&self) -> &[u16] {
        self.0.as_slice_with_nul()
    }
}
