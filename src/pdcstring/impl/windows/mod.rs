pub(crate) type PdCStringInnerImpl = widestring::U16CString;
pub(crate) type PdCStrInnerImpl = widestring::U16CStr;
pub(crate) type ToStringErrorInnerImpl = widestring::error::Utf16Error;
pub(crate) type MissingNulTerminatorInnerImpl = widestring::error::MissingNulTerminator;

mod pdcstr;
pub use pdcstr::*;

mod pdcstring;
pub use pdcstring::*;

mod error;
pub use error::*;

mod ext;
pub use ext::*;
