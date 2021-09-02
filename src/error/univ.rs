use crate::{error::HostingError, hostfxr::GetFunctionPointerError, nethost::LoadHostfxrError};
use thiserror::Error;

/// A universal error type encompassing all possible errors from the [`netcorehost`](crate) crate.
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Hosting(#[from] HostingError),
    #[error(transparent)]
    GetFunctionPointer(#[from] GetFunctionPointerError),
    #[error(transparent)]
    LoadHostfxr(#[from] LoadHostfxrError)
}

impl From<dlopen::Error> for Error {
    fn from(err: dlopen::Error) -> Self {
        Self::LoadHostfxr(LoadHostfxrError::DlOpen(err))
    }
}

