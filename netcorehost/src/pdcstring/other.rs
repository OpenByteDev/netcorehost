use std::os::unix::ffi::OsStrExt;
use std::{
    borrow::Borrow,
    ffi::{CStr, CString, OsStr, OsString},
};

use crate::NulError;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
#[repr(transparent)]
pub struct PdCString(CString);

impl PdCString {
    pub fn from_c_string(s: CString) -> Self {
        PdCString(s)
    }
    pub fn from_os_str<T: AsRef<OsStr>>(s: T) -> Result<Self, NulError> {
        PdCString::from_vec(s.as_ref().as_bytes().to_vec())
    }
    pub fn from_str(s: &str) -> Result<Self, NulError> {
        PdCString::from_vec(s.as_bytes().to_vec())
    }
    pub fn from_vec(mut vec: Vec<u8>) -> Result<Self, NulError> {
        if vec.ends_with(&[0]) {
            vec.push(0);
        }

        let inner = CString::new(vec)?;
        Ok(PdCString::from_c_string(inner))
    }
    pub unsafe fn from_str_ptr(ptr: *const i8) -> Self {
        PdCStr::from_str_ptr(ptr).to_owned()
    }
}

impl Borrow<PdCStr> for PdCString {
    fn borrow(&self) -> &PdCStr {
        PdCStr::from_c_str(self.0.borrow())
    }
}

impl AsRef<PdCStr> for PdCString {
    fn as_ref(&self) -> &PdCStr {
        self.borrow()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct PdCStr(CStr);

impl PdCStr {
    pub fn from_c_str(s: &CStr) -> &Self {
        unsafe { &*(s as *const CStr as *const PdCStr) }
    }
    pub fn as_ptr(&self) -> *const i8 {
        self.0.as_ptr()
    }
    pub unsafe fn from_str_ptr<'a>(ptr: *const i8) -> &'a Self {
        let inner = CStr::from_ptr(ptr);
        PdCStr::from_c_str(inner)
    }
    pub unsafe fn from_slice_with_nul_unchecked(slice: &[u8]) -> &Self {
        let inner = CStr::from_bytes_with_nul_unchecked(slice);
        PdCStr::from_c_str(inner)
    }
    pub fn to_os_str(&self) -> &OsStr {
        OsStr::from_bytes(self.0.to_bytes())
    }
    pub fn to_os_string(&self) -> OsString {
        self.to_os_str().to_owned()
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
        PdCString::from_c_string(self.0.to_owned())
    }
}
