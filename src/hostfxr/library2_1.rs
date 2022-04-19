use crate::{
    bindings::hostfxr::{
        hostfxr_resolve_sdk2_flags_t, hostfxr_resolve_sdk2_result_key_t, PATH_SEPARATOR,
    },
    error::{HostingError, HostingResult},
    hostfxr::{AppOrHostingResult, Hostfxr},
    pdcstring::{PdCStr, PdUChar},
};
use coreclr_hosting_shared::char_t;
#[cfg(feature = "sdk-resolver")]
use once_cell::sync::Lazy;
#[cfg(feature = "sdk-resolver")]
use parking_lot::ReentrantMutex;
use std::{
    cell::RefCell,
    mem::MaybeUninit,
    path::{Path, PathBuf},
    slice,
};

impl Hostfxr {
    /// Run an application.
    ///
    /// # Arguments
    ///  * `args`
    ///     command-line arguments
    ///  * `host_path`
    ///     path to the host application
    ///  * `dotnet_root`
    ///     path to the .NET Core installation root
    ///  * `app_path`
    ///     path to the application to run
    ///
    /// This function does not return until the application completes execution.
    /// It will shutdown CoreCLR after the application executes.
    /// If the application is successfully executed, this value will return the exit code of the application.
    /// Otherwise, it will return an error code indicating the failure.
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "netcore2_1")))]
    pub fn run_app_with_startup_info<A: AsRef<PdCStr>>(
        &self,
        args: &[A],
        host_path: &PdCStr,
        dotnet_root: &PdCStr,
        app_path: &PdCStr,
    ) -> AppOrHostingResult {
        let args = args.iter().map(|s| s.as_ref().as_ptr()).collect::<Vec<_>>();
        let result = unsafe {
            self.0.hostfxr_main_startupinfo(
                args.len().try_into().unwrap(),
                args.as_ptr(),
                host_path.as_ptr(),
                dotnet_root.as_ptr(),
                app_path.as_ptr(),
            )
        };
        AppOrHostingResult::from(result)
    }

    /// Determine the directory location of the SDK, accounting for `global.json` and multi-level lookup policy.
    ///
    /// # Arguments
    ///  * `exe_dir` - main directory where SDKs are located in `sdk\[version]` sub-folders.
    ///  * `working_dir` - directory where the search for `global.json` will start and proceed upwards
    ///  * `allow_prerelease` - allow resolution to return a pre-release SDK version
    #[cfg(feature = "sdk-resolver")]
    #[cfg_attr(
        feature = "doc-cfg",
        doc(cfg(all(feature = "sdk-resolver", feature = "netcore2_1")))
    )]
    pub fn resolve_sdk(
        &self,
        exe_dir: &PdCStr,
        working_dir: &PdCStr,
        allow_prerelease: bool,
    ) -> Result<ResolveSdkResult, HostingError> {
        let sdk_path = RESOLVE_SDK2_MUTEX.lock();

        let flags = if allow_prerelease {
            0
        } else {
            hostfxr_resolve_sdk2_flags_t::disallow_prerelease as i32
        };
        let result = unsafe {
            self.0.hostfxr_resolve_sdk2(
                exe_dir.as_ptr(),
                working_dir.as_ptr(),
                flags,
                resolve_sdk2_callback,
            )
        };
        HostingResult::from(result).into_result()?;

        Ok(sdk_path.take().unwrap())
    }

    /// Get the list of all available SDKs ordered by ascending version.
    ///
    /// # Arguments
    ///  * `exe_dir` - path to the dotnet executable
    #[cfg(feature = "sdk-resolver")]
    #[cfg_attr(
        feature = "doc-cfg",
        doc(cfg(all(feature = "sdk-resolver", feature = "netcore2_1")))
    )]
    pub fn get_available_sdks(&self, exe_dir: &PdCStr) -> Vec<PathBuf> {
        let sdk_dirs = GET_AVAILABLE_SDKS_MUTEX.lock();
        unsafe {
            self.0
                .hostfxr_get_available_sdks(exe_dir.as_ptr(), get_available_sdks_callback)
        };
        sdk_dirs.take().unwrap()
    }

    /// Get the native search directories of the runtime based upon the specified app.
    ///
    /// # Arguments
    ///  * `args` - command-line arguments
    pub fn get_native_search_directories<A: AsRef<PdCStr>>(
        &self,
        args: &[A],
    ) -> Result<Vec<PathBuf>, HostingError> {
        let args = args.iter().map(|s| s.as_ref().as_ptr()).collect::<Vec<_>>();
        let mut buffer = Vec::<PdUChar>::new();

        let mut required_buffer_size = MaybeUninit::uninit();
        unsafe {
            self.0.hostfxr_get_native_search_directories(
                args.len().try_into().unwrap(),
                args.as_ptr(),
                buffer.as_mut_ptr().cast(),
                0,
                required_buffer_size.as_mut_ptr(),
            )
        };
        let mut required_buffer_size = unsafe { required_buffer_size.assume_init() };

        buffer.reserve(required_buffer_size.try_into().unwrap());
        let result = unsafe {
            self.0.hostfxr_get_native_search_directories(
                args.len().try_into().unwrap(),
                args.as_ptr(),
                buffer.spare_capacity_mut().as_mut_ptr().cast(),
                buffer.spare_capacity_mut().len().try_into().unwrap(),
                &mut required_buffer_size,
            )
        };
        HostingResult::from(result).into_result()?;
        unsafe { buffer.set_len(required_buffer_size.try_into().unwrap()) };

        let directories = buffer
            .split(|&c| c == PATH_SEPARATOR as PdUChar)
            .map(|s| {
                PdCStr::from_slice_with_nul(s)
                    .unwrap()
                    .to_os_string()
                    .into()
            })
            .collect::<Vec<_>>();

        Ok(directories)
    }
}

#[cfg(feature = "sdk-resolver")]
static GET_AVAILABLE_SDKS_MUTEX: Lazy<ReentrantMutex<RefCell<Option<Vec<PathBuf>>>>> =
    Lazy::new(|| ReentrantMutex::new(RefCell::new(None)));

#[cfg(feature = "sdk-resolver")]
extern "C" fn get_available_sdks_callback(sdk_count: i32, sdks_ptr: *const *const char_t) {
    let sdks_guard = GET_AVAILABLE_SDKS_MUTEX.lock();
    let mut sdks_holder = sdks_guard.borrow_mut();
    let sdks = sdks_holder.get_or_insert_with(Vec::new);

    let sdk_count = sdk_count as usize;
    sdks.reserve(sdk_count as usize);

    let raw_sdks = unsafe { slice::from_raw_parts(sdks_ptr, sdk_count) };

    for &raw_sdk in raw_sdks {
        let sdk = unsafe { PdCStr::from_str_ptr(raw_sdk) };
        sdks.push(sdk.to_os_string().into());
    }
}

#[cfg(feature = "sdk-resolver")]
static RESOLVE_SDK2_MUTEX: Lazy<ReentrantMutex<RefCell<Option<ResolveSdkResult>>>> =
    Lazy::new(|| ReentrantMutex::new(RefCell::new(None)));

#[cfg(feature = "sdk-resolver")]
extern "C" fn resolve_sdk2_callback(key: hostfxr_resolve_sdk2_result_key_t, value: *const char_t) {
    let sdk_guard = RESOLVE_SDK2_MUTEX.lock();
    let mut sdk_holder = sdk_guard.borrow_mut();
    let path = unsafe { PdCStr::from_str_ptr(value) };
    let path = PathBuf::from(path.to_os_string());
    *sdk_holder = Some(match key {
        hostfxr_resolve_sdk2_result_key_t::resolved_sdk_dir => {
            ResolveSdkResult::ResolvedSdkDirectory(path)
        }
        hostfxr_resolve_sdk2_result_key_t::global_json_path => {
            ResolveSdkResult::GlobalJsonPath(path)
        }
    });
}

/// Result of [`Hostfxr::hostfxr_resolve_sdk`](struct.Hostfxr.html#method.hostfxr_resolve_sdk)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolveSdkResult {
    /// `global.json` was not present or did not impact the resolved SDK location.
    ResolvedSdkDirectory(PathBuf),
    /// `global.json` was used during resolution.
    GlobalJsonPath(PathBuf),
}

impl ResolveSdkResult {
    /// Returns the path to the resolved SDK directory.
    #[must_use]
    pub fn into_path(self) -> PathBuf {
        match self {
            ResolveSdkResult::ResolvedSdkDirectory(path) => path,
            ResolveSdkResult::GlobalJsonPath(path) => path,
        }
    }

    /// Returns the path to the resolved SDK directory.
    #[must_use]
    pub fn path(&self) -> &Path {
        match self {
            ResolveSdkResult::ResolvedSdkDirectory(path) => path,
            ResolveSdkResult::GlobalJsonPath(path) => path,
        }
    }
}