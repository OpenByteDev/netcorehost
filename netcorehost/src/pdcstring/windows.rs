use std::{
    borrow::Borrow,
    ffi::{OsStr, OsString},
};

use widestring::{U16CStr, U16CString};

use crate::NulError;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
#[repr(transparent)]
pub struct PdCString(U16CString);

impl PdCString {
    pub fn from_u16_c_string(s: U16CString) -> Self {
        PdCString(s)
    }
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

impl Borrow<PdCStr> for PdCString {
    fn borrow(&self) -> &PdCStr {
        PdCStr::from_u16_c_str(self.0.borrow())
    }
}

impl AsRef<PdCStr> for PdCString {
    fn as_ref(&self) -> &PdCStr {
        self.borrow()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct PdCStr(U16CStr);

impl PdCStr {
    pub fn from_u16_c_str(s: &U16CStr) -> &Self {
        unsafe { &*(s as *const U16CStr as *const PdCStr) }
    }
    pub fn as_ptr(&self) -> *const u16 {
        self.0.as_ptr()
    }
    pub unsafe fn from_str_ptr<'a>(ptr: *const u16) -> &'a Self {
        let inner = U16CStr::from_ptr_str(ptr);
        PdCStr::from_u16_c_str(inner)
    }
    pub unsafe fn from_slice_with_nul_unchecked(slice: &[u16]) -> &Self {
        let inner = U16CStr::from_slice_with_nul_unchecked(slice);
        PdCStr::from_u16_c_str(inner)
    }
    pub fn to_os_string(&self) -> OsString {
        self.0.to_os_string()
    }
}

impl AsRef<PdCStr> for PdCStr {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl ToOwned for PdCStr {
    type Owned = PdCString;
    fn to_owned(&self) -> Self::Owned {
        PdCString::from_u16_c_string(self.0.to_owned())
    }
}
