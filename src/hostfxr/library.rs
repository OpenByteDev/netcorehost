use crate::{
    bindings::hostfxr::wrapper::Hostfxr as HostfxrLib, dlopen::wrapper::Container,
    error::HostingResult, nethost::LoadHostfxrError,
};
use derive_more::From;
use std::{ffi::OsStr, rc::Rc};

/// A struct representing a loaded hostfxr library.
#[derive(Clone, From)]
pub struct Hostfxr(pub Rc<Container<HostfxrLib>>);

impl Hostfxr {
    /// Loads the hostfxr library from the given path.
    pub fn load_from_path(path: impl AsRef<OsStr>) -> Result<Self, crate::dlopen::Error> {
        unsafe { Container::load(path) }.map(Rc::new).map(Self)
    }

    /// Locates the hostfxr library using [`nethost`](crate::nethost) and loads it.
    #[cfg(feature = "nethost")]
    pub fn load_with_nethost() -> Result<Self, LoadHostfxrError> {
        crate::nethost::load_hostfxr()
    }
}

/// Either the exit code of the app if it ran successful, otherwise the error from the hosting components.
#[repr(transparent)]
#[cfg_attr(
    all(nightly, feature = "doc-cfg"),
    attr(doc(cfg(feature = "netcore3_0")))
)]
pub struct AppOrHostingResult(i32);

impl AppOrHostingResult {
    /// Gets the raw value of the result.
    #[must_use]
    pub fn value(self) -> i32 {
        self.0
    }
    /// Converts the result to an hosting exit code.
    pub fn as_hosting_exit_code(self) -> HostingResult {
        HostingResult::from(self.0)
    }
}

impl From<AppOrHostingResult> for i32 {
    fn from(code: AppOrHostingResult) -> Self {
        code.value()
    }
}

impl From<i32> for AppOrHostingResult {
    fn from(code: i32) -> Self {
        Self(code)
    }
}
