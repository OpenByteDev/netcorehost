use std::{
    error::Error,
    ffi::{OsStr, OsString},
    fmt::{Debug, Display},
};

use crate::pdcstring::{ContainsNul, MissingNulTerminator, PdChar, PdUChar, ToStringError};

pub(crate) trait PdCStringInner
where
    Self: Sized,
{
    fn from_str(s: impl AsRef<str>) -> Result<Self, ContainsNul>;
    fn from_os_str(s: impl AsRef<OsStr>) -> Result<Self, ContainsNul>;
    unsafe fn from_str_ptr(ptr: *const PdChar) -> Self;
    fn from_vec(vec: impl Into<Vec<PdUChar>>) -> Result<Self, ContainsNul>;
    fn into_vec(self) -> Vec<PdUChar>;
    fn into_vec_with_nul(self) -> Vec<PdUChar>;
}

pub(crate) trait PdCStrInner {
    fn as_ptr(&self) -> *const PdChar;
    unsafe fn from_str_ptr<'a>(ptr: *const PdChar) -> &'a Self;
    unsafe fn from_slice_with_nul_unchecked(slice: &[PdUChar]) -> &Self;
    fn to_os_string(&self) -> OsString;
    fn from_slice_with_nul(slice: &[PdUChar]) -> Result<&Self, MissingNulTerminator>;
    fn as_slice(&self) -> &[PdUChar];
    fn as_slice_with_nul(&self) -> &[PdUChar];
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn to_string(&self) -> Result<String, ToStringError>;
    fn to_string_lossy(&self) -> String;
}

pub(crate) trait ToStringErrorInner: Debug + Display + Error + Clone {
    fn index(&self) -> Option<usize>;
}

pub(crate) trait MissingNulTerminatorInner: Debug + Display + Error + Clone {}
