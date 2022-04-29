use crate::{pdcstr, pdcstring::PdCStr};

#[cfg(windows)]
pub(crate) fn get_dotnet_bin_path() -> &'static PdCStr {
    pdcstr!("dotnet")
}

#[cfg(not(windows))]
pub(crate) fn get_dotnet_bin_path() -> &'static PdCStr {
    #[cfg(not(nightly))]
    use once_cell::sync::Lazy as SyncLazy;
    #[cfg(nightly)]
    use std::lazy::SyncLazy;

    use crate::pdcstring::PdCString;

    static DOTNET_BIN: SyncLazy<Option<PdCString>> = SyncLazy::new(|| {
        which::which("dotnet")
            .ok()
            .and_then(|p| PdCString::from_os_str(p.as_os_str()).ok())
    });

    match *DOTNET_BIN {
        Some(ref path) => path,
        None => pdcstr!("dotnet"),
    }
}
