use std::{
    ffi::{CStr, CString},
    os::unix::prelude::OsStrExt,
};

use crate::pdcstring::{ContainsNul, PdCStringInner, PdChar, PdUChar};

impl PdCStringInner for CString {
    fn from_str(s: impl AsRef<str>) -> Result<Self, ContainsNul> {
        Self::from_vec(s.as_ref().as_bytes().to_vec())
    }

    fn from_os_str(s: impl AsRef<std::ffi::OsStr>) -> Result<Self, ContainsNul> {
        Self::from_vec(s.as_ref().as_bytes().to_vec())
    }

    unsafe fn from_str_ptr(ptr: *const PdChar) -> Self {
        unsafe { CStr::from_ptr(ptr) }.to_owned()
    }

    fn from_vec(vec: impl Into<Vec<PdUChar>>) -> Result<Self, ContainsNul> {
        CString::new(vec).map_err(|e| e.into())
    }

    fn into_vec(self) -> Vec<PdUChar> {
        CString::into_bytes(self)
    }

    fn into_vec_with_nul(self) -> Vec<PdUChar> {
        CString::into_bytes_with_nul(self)
    }
}
