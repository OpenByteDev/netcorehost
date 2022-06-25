use std::{
    borrow::Borrow,
    convert::TryFrom,
    ffi::{OsStr, OsString},
    fmt::{self, Debug, Display, Formatter},
    ops::Deref, str::FromStr,
};

use super::{
    ContainsNul, MissingNulTerminator, PdCStrInner, PdCStrInnerImpl, PdCStringInner,
    PdCStringInnerImpl, PdChar, PdUChar, ToStringError,
};

/// A platform-dependent c-like string type for interacting with the .NET hosting components.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Default)]
#[repr(transparent)]
pub struct PdCString(pub(crate) PdCStringInnerImpl);

impl PdCString {
    #[inline]
    pub(crate) fn from_inner(inner: PdCStringInnerImpl) -> Self {
        Self(inner)
    }
    #[inline]
    pub(crate) fn into_inner(self) -> PdCStringInnerImpl {
        self.0
    }

    /// Construct a [`PdCString`] copy from an [`OsStr`], reencoding it in a platform-dependent manner.
    #[inline]
    pub fn from_os_str(s: impl AsRef<OsStr>) -> Result<Self, ContainsNul> {
        PdCStringInner::from_os_str(s).map(Self::from_inner)
    }
    /// Constructs a new [`PdCString`] copied from a nul-terminated string pointer.
    #[inline]
    #[must_use]
    pub unsafe fn from_str_ptr(ptr: *const PdChar) -> Self {
        Self::from_inner(unsafe { PdCStringInner::from_str_ptr(ptr) })
    }
    /// Constructs a [`PdCString`] from a container of platform-dependent character data.
    #[inline]
    pub fn from_vec(vec: impl Into<Vec<PdUChar>>) -> Result<Self, ContainsNul> {
        PdCStringInner::from_vec(vec).map(Self::from_inner)
    }
    /// Converts the string into a [`Vec`] without a nul terminator, consuming the string in the process.
    #[inline]
    #[must_use]
    pub fn into_vec(self) -> Vec<PdUChar> {
        PdCStringInner::into_vec(self.into_inner())
    }
    /// Converts the string into a [`Vec`], consuming the string in the process.
    #[inline]
    #[must_use]
    pub fn into_vec_with_nul(self) -> Vec<PdUChar> {
        PdCStringInner::into_vec_with_nul(self.into_inner())
    }
}

/// A borrowed slice of a [`PdCString`].
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct PdCStr(pub(crate) PdCStrInnerImpl);

impl PdCStr {
    #[inline]
    pub(crate) fn from_inner(inner: &PdCStrInnerImpl) -> &Self {
        // Safety:
        // Safe because PdCStr has the same layout as PdCStrInnerImpl
        unsafe { &*(inner as *const PdCStrInnerImpl as *const PdCStr) }
    }
    #[inline]
    pub(crate) fn as_inner(&self) -> &PdCStrInnerImpl {
        // Safety:
        // Safe because PdCStr has the same layout as PdCStrInnerImpl
        unsafe { &*(self as *const PdCStr as *const PdCStrInnerImpl) }
    }

    /// Returns a raw pointer to the string.
    #[inline]
    #[must_use]
    pub fn as_ptr(&self) -> *const PdChar {
        PdCStrInner::as_ptr(self.as_inner())
    }
    /// Constructs a [`PdCStr`] from a nul-terminated string pointer.
    #[inline]
    #[must_use]
    pub unsafe fn from_str_ptr<'a>(ptr: *const PdChar) -> &'a Self {
        Self::from_inner(unsafe { PdCStrInner::from_str_ptr(ptr) })
    }
    /// Constructs a [`PdCStr`] from a slice of characters with a terminating nul, checking for invalid interior nul values.
    #[inline]
    pub fn from_slice_with_nul(slice: &[PdUChar]) -> Result<&Self, MissingNulTerminator> {
        PdCStrInner::from_slice_with_nul(slice).map(Self::from_inner)
    }
    /// Constructs a [`PdCStr`] from a slice of values without checking for a terminating or interior nul values.
    #[inline]
    #[must_use]
    pub unsafe fn from_slice_with_nul_unchecked(slice: &[PdUChar]) -> &Self {
        Self::from_inner(unsafe { PdCStrInner::from_slice_with_nul_unchecked(slice) })
    }
    /// Copys the string to an owned [`OsString`].
    #[inline]
    #[must_use]
    pub fn to_os_string(&self) -> OsString {
        PdCStrInner::to_os_string(self.as_inner())
    }
    /// Converts this string to a slice of the underlying elements.
    /// The slice will **not** include the nul terminator.
    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[PdUChar] {
        PdCStrInner::as_slice(self.as_inner())
    }
    /// Converts this string to a slice of the underlying elements, including the nul terminator.
    #[inline]
    #[must_use]
    pub fn as_slice_with_nul(&self) -> &[PdUChar] {
        PdCStrInner::as_slice_with_nul(self.as_inner())
    }
    /// Returns whether this string contains no data (i.e. is only the nul terminator).
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        PdCStrInner::is_empty(self.as_inner())
    }
    /// Returns the length of the string as number of elements (not number of bytes) not including the nul terminator.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        PdCStrInner::len(self.as_inner())
    }
    /// Copies the string to a [`String`] if it contains valid encoded data.
    #[inline]
    pub fn to_string(&self) -> Result<String, ToStringError> {
        PdCStrInner::to_string(self.as_inner())
    }
    /// Decodes the string to a [`String`] even if it contains invalid data.
    /// Any invalid sequences are replaced with U+FFFD REPLACEMENT CHARACTER, which looks like this: �. It will *not have a nul terminator.
    #[inline]
    #[must_use]
    pub fn to_string_lossy(&self) -> String {
        PdCStrInner::to_string_lossy(self.as_inner())
    }
}

impl Borrow<PdCStr> for PdCString {
    fn borrow(&self) -> &PdCStr {
        PdCStr::from_inner(self.0.borrow())
    }
}

impl AsRef<PdCStr> for PdCString {
    fn as_ref(&self) -> &PdCStr {
        self.borrow()
    }
}

impl Deref for PdCString {
    type Target = PdCStr;

    fn deref(&self) -> &Self::Target {
        self.borrow()
    }
}

impl Display for PdCStr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<PdCStringInnerImpl> for PdCString {
    fn from(s: PdCStringInnerImpl) -> Self {
        Self::from_inner(s)
    }
}

impl From<PdCString> for PdCStringInnerImpl {
    fn from(s: PdCString) -> Self {
        s.into_inner()
    }
}

impl<'a> From<&'a PdCStrInnerImpl> for &'a PdCStr {
    fn from(s: &'a PdCStrInnerImpl) -> Self {
        PdCStr::from_inner(s)
    }
}

impl<'a> From<&'a PdCStr> for &'a PdCStrInnerImpl {
    fn from(s: &'a PdCStr) -> Self {
        s.as_inner()
    }
}

impl FromStr for PdCString {
    type Err = ContainsNul;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PdCStringInner::from_str(s).map(Self::from_inner)
    }
}

impl<'a> TryFrom<&'a str> for PdCString {
    type Error = ContainsNul;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Self::from_str(s)
    }
}

impl TryFrom<Vec<PdUChar>> for PdCString {
    type Error = ContainsNul;

    fn try_from(vec: Vec<PdUChar>) -> Result<Self, Self::Error> {
        Self::from_vec(vec)
    }
}

impl From<PdCString> for Vec<PdUChar> {
    fn from(s: PdCString) -> Vec<PdUChar> {
        s.into_vec()
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
        PdCString::from_inner(self.0.to_owned())
    }
}
