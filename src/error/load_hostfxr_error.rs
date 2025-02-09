use crate::error::HostingError;
use thiserror::Error;

/// Enum for errors that can occur while locating and loading the hostfxr library.
#[derive(Debug, Error)]
pub enum LoadHostfxrError {
    /// An error occured inside the hosting components.
    #[error(transparent)]
    Hosting(#[from] HostingError),
    /// An error occured while loading the hostfxr library.
    #[error(transparent)]
    DlOpen(#[from] crate::dlopen2::Error),
}
