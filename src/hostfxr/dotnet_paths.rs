use std::{path::PathBuf, str::FromStr};

use crate::{pdcstr, pdcstring::PdCString};

#[cfg(not(nightly))]
use once_cell::sync::Lazy as SyncLazy;
#[cfg(nightly)]
use std::lazy::SyncLazy;

pub(crate) static DOTNET_BIN: SyncLazy<PathBuf> = SyncLazy::new(|| {
    which::which("dotnet").unwrap_or_else(|_| PathBuf::from_str("dotnet").unwrap())
});
pub(crate) static DOTNET_BIN_PDC: SyncLazy<PdCString> = SyncLazy::new(|| {
    PdCString::from_os_str(DOTNET_BIN.as_os_str()).unwrap_or_else(|_| pdcstr!("dotnet").to_owned())
});
