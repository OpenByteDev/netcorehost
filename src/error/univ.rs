use crate::{error::HostingError, hostfxr::GetManagedFunctionError, nethost::LoadHostfxrError};
use thiserror::Error;

/// A universal error type encompassing all possible errors from the [`netcorehost`](crate) crate.
#[derive(Debug, Error)]
pub enum Error {
    /// An error from the native hosting components.
    #[error(transparent)]
    Hosting(#[from] HostingError),
    /// An error while loading a function pointer to a managed method.
    #[error(transparent)]
    GetFunctionPointer(#[from] GetManagedFunctionError),
    /// An error while loading the hostfxr library.
    #[error(transparent)]
    LoadHostfxr(#[from] LoadHostfxrError),
}

impl From<crate::dlopen::Error> for Error {
    fn from(err: crate::dlopen::Error) -> Self {
        Self::LoadHostfxr(LoadHostfxrError::DlOpen(err))
    }
}
