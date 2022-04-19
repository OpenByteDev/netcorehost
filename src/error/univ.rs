use thiserror::Error;

/// A universal error type encompassing all possible errors from the [`netcorehost`](crate) crate.
#[derive(Debug, Error)]
pub enum Error {
    /// An error from the native hosting components.
    #[error(transparent)]
    Hosting(#[from] crate::error::HostingError),
    /// An error while loading a function pointer to a managed method.
    #[error(transparent)]
    #[cfg(feature = "netcore3_0")]
    #[cfg_attr(
        all(nightly, feature = "doc-cfg"),
        attr(doc(cfg(feature = "netcore3_0")))
    )]
    GetFunctionPointer(#[from] crate::hostfxr::GetManagedFunctionError),
    /// An error while loading the hostfxr library.
    #[error(transparent)]
    LoadHostfxr(#[from] crate::nethost::LoadHostfxrError),
}

impl From<crate::dlopen::Error> for Error {
    fn from(err: crate::dlopen::Error) -> Self {
        Self::LoadHostfxr(crate::nethost::LoadHostfxrError::DlOpen(err))
    }
}
