use std::{path::PathBuf, str::FromStr};

use once_cell::sync::Lazy;

use crate::{pdcstr, pdcstring::PdCString};

pub(crate) static DOTNET_BIN: Lazy<PathBuf> =
    Lazy::new(|| which::which("dotnet").unwrap_or_else(|_| PathBuf::from_str("dotnet").unwrap()));
pub(crate) static DOTNET_BIN_PDC: Lazy<PdCString> = Lazy::new(|| {
    PdCString::from_os_str(DOTNET_BIN.as_os_str()).unwrap_or_else(|_| pdcstr!("dotnet").to_owned())
});
