use hostfxr_sys::hostfxr_dotnet_environment_info;

use crate::{
    error::{HostingError, HostingResult},
    hostfxr::Hostfxr,
    pdcstring::PdCStr,
};
use std::{ffi::c_void, mem::MaybeUninit, path::PathBuf, ptr, slice};

use super::UNSUPPORTED_HOST_VERSION_ERROR_CODE;

/// Information about the current dotnet environment loaded using [Hostfxr::get_dotnet_environment_info].
#[derive(Debug, Clone)]
pub struct EnvironmentInfo {
    /// Version of hostfxr used to load this info.
    pub hostfxr_version: String,
    /// Commit hash of hostfxr used to load this info.
    pub hostfxr_commit_hash: String,
    /// Currently installed sdks, ordered by version ascending.
    pub sdks: Vec<SdkInfo>,
    /// Currently installed frameworks, ordered by name and then version ascending.
    pub frameworks: Vec<FrameworkInfo>,
}

impl PartialEq for EnvironmentInfo {
    fn eq(&self, other: &Self) -> bool {
        self.hostfxr_version == other.hostfxr_version
            && (self
                .hostfxr_commit_hash
                .starts_with(&other.hostfxr_commit_hash)
                || other
                    .hostfxr_commit_hash
                    .starts_with(&self.hostfxr_commit_hash))
            && self.sdks == other.sdks
            && self.frameworks == other.frameworks
    }
}

impl Eq for EnvironmentInfo {}

impl PartialOrd for EnvironmentInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.hostfxr_version.cmp(&other.hostfxr_version))
    }
}

/// A struct representing an installed sdk.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkInfo {
    /// The version of the sdk.
    pub version: String,
    /// The directory containing the sdk.
    pub path: PathBuf,
}

impl PartialOrd for SdkInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SdkInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.version.cmp(&other.version)
    }
}

/// A struct representing an installed framework.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameworkInfo {
    /// The name of the framework.
    pub name: String,
    /// The version of the framework.
    pub version: String,
    /// The directory containing the framework.
    pub path: PathBuf,
}

impl PartialOrd for FrameworkInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FrameworkInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name
            .cmp(&other.name)
            .then_with(|| self.version.cmp(&other.version))
    }
}

impl Hostfxr {
    /// Loads info about the dotnet environemnt, including the version of hostfxr and installed sdks and frameworks.
    ///
    /// # Ordering
    /// SDks are ordered by version ascending and multi-level lookup locations are put before private locations - items later in the list have priority over items earlier in the list.
    /// Frameworks are ordered by name ascending followed by version ascending. Multi-level lookup locations are put before private locations.
    ///
    /// # Note
    /// This is equivalent to the info retrieved using `dotnet --info`.
    /// Which means it enumerates SDKs and frameworks from the dotnet root directory (either explicitly specified or using global install location per design).
    /// If `DOTNET_MULTILEVEL_LOOKUP` is enabled (Windows-only), and the dotnet root is specified and it's not the global install location,
    /// then it will also enumerate SDKs and frameworks from the global install location.
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "net6_0")))]
    pub fn get_dotnet_environment_info(&self) -> Result<EnvironmentInfo, HostingError> {
        let mut info = MaybeUninit::<EnvironmentInfo>::uninit();
        let result = unsafe {
            self.lib.hostfxr_get_dotnet_environment_info(
                ptr::null(),
                ptr::null_mut(),
                get_dotnet_environment_info_callback,
                info.as_mut_ptr().cast(),
            )
        }
        .unwrap_or(UNSUPPORTED_HOST_VERSION_ERROR_CODE);
        HostingResult::from(result).into_result()?;
        let info = unsafe { MaybeUninit::assume_init(info) };
        Ok(info)
    }
}

extern "C" fn get_dotnet_environment_info_callback(
    info: *const hostfxr_dotnet_environment_info,
    result_context: *mut c_void,
) {
    let result = result_context.cast::<EnvironmentInfo>();

    let raw_info = unsafe { &*info };
    let hostfxr_version =
        unsafe { PdCStr::from_str_ptr(raw_info.hostfxr_version) }.to_string_lossy();
    let hostfxr_commit_hash =
        unsafe { PdCStr::from_str_ptr(raw_info.hostfxr_commit_hash) }.to_string_lossy();

    let raw_sdks = unsafe { slice::from_raw_parts(raw_info.sdks, raw_info.sdk_count) };
    let sdks = raw_sdks
        .iter()
        .map(|raw_sdk| {
            let version = unsafe { PdCStr::from_str_ptr(raw_sdk.version) }.to_string_lossy();
            let path = unsafe { PdCStr::from_str_ptr(raw_sdk.path) }
                .to_os_string()
                .into();
            SdkInfo { version, path }
        })
        .collect::<Vec<_>>();

    let raw_frameworks =
        unsafe { slice::from_raw_parts(raw_info.frameworks, raw_info.framework_count) };
    let frameworks = raw_frameworks
        .iter()
        .map(|raw_framework| {
            let name = unsafe { PdCStr::from_str_ptr(raw_framework.name) }.to_string_lossy();
            let version = unsafe { PdCStr::from_str_ptr(raw_framework.version) }.to_string_lossy();
            let path = unsafe { PdCStr::from_str_ptr(raw_framework.path) }
                .to_os_string()
                .into();
            FrameworkInfo {
                name,
                version,
                path,
            }
        })
        .collect::<Vec<_>>();

    let info = EnvironmentInfo {
        hostfxr_version,
        hostfxr_commit_hash,
        sdks,
        frameworks,
    };

    unsafe { result.write(info) };
}
