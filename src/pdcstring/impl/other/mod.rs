pub(crate) type PdCStringInnerImpl = std::ffi::CString;
pub(crate) type PdCStrInnerImpl = std::ffi::CStr;
pub(crate) type ToStringErrorInnerImpl = std::str::Utf8Error;
pub(crate) type MissingNulTerminatorInnerImpl = std::ffi::FromBytesWithNulError;

mod pdcstr;
pub use pdcstr::*;

mod pdcstring;
pub use pdcstring::*;

mod error;
pub use error::*;

mod ext;
pub use ext::*;
