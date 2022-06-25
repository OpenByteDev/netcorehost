use widestring::U16CString;

use crate::pdcstring::{ContainsNul, PdCStringInner, PdChar};

impl PdCStringInner for U16CString {
    fn from_str(s: &str) -> Result<Self, ContainsNul> {
        Ok(U16CString::from_str(s)?)
    }

    fn from_os_str(s: impl AsRef<std::ffi::OsStr>) -> Result<Self, ContainsNul> {
        U16CString::from_os_str(s).map_err(|e| e.into())
    }

    unsafe fn from_str_ptr(ptr: *const PdChar) -> Self {
        unsafe { U16CString::from_ptr_str(ptr) }
    }

    fn from_vec(vec: impl Into<Vec<PdChar>>) -> Result<Self, ContainsNul> {
        U16CString::from_vec(vec).map_err(|e| e.into())
    }

    fn into_vec(self) -> Vec<PdChar> {
        U16CString::into_vec(self)
    }

    fn into_vec_with_nul(self) -> Vec<PdChar> {
        U16CString::into_vec_with_nul(self)
    }
}
