use std::convert::TryFrom;
#[cfg(feature = "nightly")]
use std::ops::{ControlFlow, FromResidual, Try};

use crate::bindings;
use derive_more::{Deref, Display, From};

/// Result of a hosting API operation of `hostfxr`, `hostpolicy` and `nethost`.
///
/// Source: [https://github.com/dotnet/runtime/blob/main/docs/design/features/host-error-codes.md](https://github.com/dotnet/runtime/blob/main/docs/design/features/host-error-codes.md)
#[must_use]
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash, Deref, From)]
#[repr(transparent)]
pub struct HostingResult(pub Result<HostingSuccess, HostingError>);

impl HostingResult {
    /// Creates a new [`HostingResult`] from the raw status code.
    #[allow(clippy::cast_possible_wrap)]
    pub fn from_status_code(code: u32) -> Self {
        if code as i32 >= 0 {
            Self::from_success(HostingSuccess::from_status_code(code))
        } else {
            Self::from_error(HostingError::from_status_code(code))
        }
    }

    /// Creates a new successful [`HostingResult`] from the give [`HostingSuccess`].
    pub fn from_success(success: HostingSuccess) -> Self {
        Self(Ok(success))
    }

    /// Creates a new erroneous [`HostingResult`] from the give [`HostingError`].
    pub fn from_error(error: HostingError) -> Self {
        Self(Err(error))
    }

    /// Tries to create a new [`HostingResult`] from the raw status code if it is known.
    /// Otherwise returns the given value as an [`Err`].
    pub fn known_from_status_code(code: u32) -> Result<Self, u32> {
        if let Ok(success) = HostingSuccess::known_from_status_code(code) {
            Ok(Self::from_success(success))
        } else if let Ok(error) = HostingError::known_from_status_code(code) {
            Ok(Self::from_error(error))
        } else {
            Err(code)
        }
    }

    /// Returns the underlying status code value.
    #[must_use]
    pub fn value(&self) -> u32 {
        match self.0 {
            Ok(success) => success.value(),
            Err(error) => error.value(),
        }
    }

    /// Returns whether the status code of this result has a known meaning.
    #[must_use]
    pub fn is_known(&self) -> bool {
        match self.0 {
            Ok(success) => success.is_known(),
            Err(error) => error.is_known(),
        }
    }

    /// Returns whether the status code of this result has a unknown meaning.
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        match self.0 {
            Ok(success) => success.is_unknown(),
            Err(error) => error.is_unknown(),
        }
    }

    /// Transforms the result into a [`Result<HostingSuccess, HostingError>`].
    pub fn into_result(&self) -> Result<HostingSuccess, HostingError> {
        self.0
    }
}

impl From<u32> for HostingResult {
    fn from(code: u32) -> Self {
        Self::from_status_code(code)
    }
}

impl From<i32> for HostingResult {
    fn from(code: i32) -> Self {
        Self::from(code as u32)
    }
}

impl From<HostingResult> for u32 {
    fn from(code: HostingResult) -> Self {
        code.value()
    }
}

impl From<HostingResult> for i32 {
    #[allow(clippy::cast_possible_wrap)]
    fn from(code: HostingResult) -> Self {
        code.value() as i32
    }
}

impl From<HostingSuccess> for HostingResult {
    fn from(success: HostingSuccess) -> Self {
        Self::from_success(success)
    }
}

impl From<HostingError> for HostingResult {
    fn from(error: HostingError) -> Self {
        Self::from_error(error)
    }
}

#[cfg(feature = "nightly")]
impl Try for HostingResult {
    type Output = HostingSuccess;
    type Residual = HostingError;
    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self.into_result().branch() {
            ControlFlow::Continue(code) => ControlFlow::Continue(code),
            ControlFlow::Break(r) => ControlFlow::Break(r.unwrap_err()),
        }
    }
    fn from_output(code: HostingSuccess) -> Self {
        HostingResult(Ok(code))
    }
}

#[cfg(feature = "nightly")]
impl FromResidual for HostingResult {
    fn from_residual(r: HostingError) -> Self {
        HostingResult(Err(r))
    }
}

/// Success codes returned by the hosting APIs from `hostfxr`, `hostpolicy` and `nethost`.
///
/// Source: [https://github.com/dotnet/runtime/blob/main/docs/design/features/host-error-codes.md](https://github.com/dotnet/runtime/blob/main/docs/design/features/host-error-codes.md)
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash, Display)]
pub enum HostingSuccess {
    /// Operation was successful.
    #[display(fmt = "Operation was successful.")]
    Success,

    /// Initialization was successful, but another host context is already initialized, so the returned context is "secondary".
    /// The requested context was otherwise fully compatible with the already initialized context.
    /// This is returned by `hostfxr_initialize_for_runtime_config` if it's called when the host is already initialized in the process.
    /// Comes from `corehost_initialize` in `hostpolicy`.
    #[display(
        fmt = "Initialization was successful, but another host context is already initialized, so the returned context is 'secondary'"
    )]
    HostAlreadyInitialized,

    /// Initialization was successful, but another host context is already initialized and the requested context specified some runtime properties which are not the same (either in value or in presence) to the already initialized context.
    /// This is returned by `hostfxr_initialize_for_runtime_config` if it's called when the host is already initialized in the process.
    /// Comes from `corehost_initialize` in `hostpolicy`.
    #[display(
        fmt = "Initialization was successful, but another host context is already initialized and the requested context specified some runtime properties which are not the same (either in value or in presence) to the already initialized context."
    )]
    DifferentRuntimeProperties,

    /// Unknown success status code.
    #[display(fmt = "Unknown success status code: {:#08X}", _0)]
    Unknown(u32),
}

impl HostingSuccess {
    /// Creates a new [`HostingSuccess`] from the raw status code.
    #[must_use]
    pub fn from_status_code(code: u32) -> Self {
        match Self::known_from_status_code(code) {
            Ok(s) => s,
            Err(code) => Self::Unknown(code),
        }
    }

    /// Tries to create a new [`HostingSuccess`] from the raw status code if it is known.
    /// Otherwise returns the given value as an [`Err`].
    pub fn known_from_status_code(code: u32) -> Result<Self, u32> {
        match code {
            c if c == bindings::StatusCode::Success as u32 => Ok(Self::Success),
            c if c == bindings::StatusCode::Success_DifferentRuntimeProperties as u32 => {
                Ok(Self::DifferentRuntimeProperties)
            }
            c if c == bindings::StatusCode::Success_HostAlreadyInitialized as u32 => {
                Ok(Self::HostAlreadyInitialized)
            }
            _ => Err(code),
        }
    }

    /// Returns the underlying status code value.
    #[must_use]
    pub fn value(&self) -> u32 {
        match self {
            Self::Success => bindings::StatusCode::Success as u32,
            Self::DifferentRuntimeProperties => {
                bindings::StatusCode::Success_DifferentRuntimeProperties as u32
            }
            Self::HostAlreadyInitialized => {
                bindings::StatusCode::Success_HostAlreadyInitialized as u32
            }
            Self::Unknown(code) => *code,
        }
    }

    /// Returns whether the status code of this success has a known meaning.
    #[must_use]
    pub fn is_known(&self) -> bool {
        !self.is_unknown()
    }

    /// Returns whether the status code of this success has a unknown meaning.
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown(_))
    }
}

impl TryFrom<u32> for HostingSuccess {
    type Error = u32;

    fn try_from(code: u32) -> Result<Self, Self::Error> {
        Self::known_from_status_code(code)
    }
}

impl From<HostingSuccess> for u32 {
    fn from(code: HostingSuccess) -> Self {
        code.value()
    }
}

/// Error codes returned by the hosting APIs from `hostfxr`, `hostpolicy` and `nethost`.
///
/// Source: [https://github.com/dotnet/runtime/blob/main/docs/design/features/host-error-codes.md](https://github.com/dotnet/runtime/blob/main/docs/design/features/host-error-codes.md)
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash, Display)]
#[must_use]
pub enum HostingError {
    /// One of the specified arguments for the operation is invalid.
    #[display(fmt = "One of the specified arguments for the operation is invalid.")]
    InvalidArgFailure,

    /// There was a failure loading a dependent library.
    /// If any of the hosting components calls `LoadLibrary`/`dlopen` on a dependent library and the call fails, this error code is returned.
    /// The most common case for this failure is if the dependent library is missing some of its dependencies (for example the necessary CRT is missing on the machine), likely corrupt or incomplete install.
    /// This error code is also returned from `corehost_resolve_component_dependencies` if it's called on a `hostpolicy` which has not been initialized via the hosting layer.
    /// This would typically happen if `coreclr` is loaded directly without the hosting layer and then `AssemblyDependencyResolver` is used (which is an unsupported scenario).
    #[display(fmt = "There was a failure loading a dependent library.")]
    CoreHostLibLoadFailure,

    /// One of the dependent libraries is missing.
    /// Typically when the `hostfxr`, `hostpolicy` or `coreclr` dynamic libraries are not present in the expected locations.
    /// Probably means corrupted or incomplete installation.
    #[display(fmt = "One of the dependent libraries is missing.")]
    CoreHostLibMissingFailure,

    /// One of the dependent libraries is missing a required entry point.
    #[display(fmt = "One of the dependent libraries is missing a required entry point.")]
    CoreHostEntryPointFailure,

    /// If the hosting component is trying to use the path to the current module (the hosting component itself) and from it deduce the location of the installation.
    /// Either the location of the current module could not be determined (some weird OS call failure) or the location is not in the right place relative to other expected components.
    /// For example the `hostfxr` may look at its location and try to deduce the location of the `shared` folder with the framework from it.
    /// It assumes the typical install layout on disk. If this doesn't work, this error will be returned.
    #[display(
        fmt = "Either the location of the current module could not be determined (some weird OS call failure) or the location is not in the right place relative to other expected components."
    )]
    CoreHostCurHostFindFailure,

    /// If the `coreclr` library could not be found.
    /// The hosting layer (`hostpolicy`) looks for `coreclr` library either next to the app itself (for self-contained) or in the root framework (for framework-dependent).
    /// This search can be done purely by looking at disk or more commonly by looking into the respective `.deps.json`.
    /// If the `coreclr` library is missing in `.deps.json` or it's there but doesn't exist on disk, this error is returned.
    #[display(fmt = "The coreclr library could not be found.")]
    CoreClrResolveFailure,

    /// The loaded `coreclr` library doesn't have one of the required entry points.
    #[display(fmt = "The loaded coreclr library doesn't have one of the required entry points.")]
    CoreClrBindFailure,

    /// The call to `coreclr_initialize` failed.
    /// The actual error returned by `coreclr` is reported in the error message.
    #[display(fmt = "The call to coreclr_initialize failed.")]
    CoreClrInitFailure,

    /// The call to `coreclr_execute_assembly` failed.
    /// Note that this does not mean anything about the app's exit code, this failure occurs if `coreclr` failed to run the app itself.
    #[display(fmt = "The call to coreclr_execute_assembly failed.")]
    CoreClrExeFailure,

    /// Initialization of the `hostpolicy` dependency resolver failed.
    /// This can be:
    ///  - One of the frameworks or the app is missing a required `.deps.json` file.
    ///  - One of the `.deps.json` files is invalid (invalid JSON, or missing required properties and so on).
    #[display(fmt = "Initialization of the hostpolicy dependency resolver failed.")]
    ResolverInitFailure,

    /// Resolution of dependencies in `hostpolicy` failed.
    /// This can mean many different things, but in general one of the processed `.deps.json` contains entry for a file which could not found, or its resolution failed for some other reason (conflict for example).
    #[display(fmt = "Resolution of dependencies in `hostpolicy` failed.")]
    ResolverResolveFailure,

    /// Failure to determine the location of the current executable.
    /// The hosting layer uses the current executable path to deduce the install location in some cases.
    /// If this path can't be obtained (OS call fails, or the returned path doesn't exist), this error is returned.
    #[display(fmt = "Failure to determine the location of the current executable.")]
    LibHostCurExeFindFailure,

    /// Initialization of the `hostpolicy` library failed.
    /// The `corehost_load` method takes a structure with lot of initialization parameters.
    /// If the version of this structure doesn't match the expected value, this error code is returned.
    /// This would in general mean incompatibility between the `hostfxr` and `hostpolicy`, which should really only happen if somehow a newer `hostpolicy` is used by older `hostfxr`.
    /// This typically means corrupted installation.
    #[display(fmt = "Initialization of the `hostpolicy` library failed.")]
    LibHostInitFailure,

    // Error only present in `error_codes.h` not in `host-error-codes.md`
    #[doc(hidden)]
    #[display(fmt = "LibHostExecModeFailure")]
    LibHostExecModeFailure,

    /// Failure to find the requested SDK.
    /// This happens in the `hostfxr` when an SDK (also called CLI) command is used with `dotnet`.
    /// In this case the hosting layer tries to find an installed .NET SDK to run the command on.
    /// The search is based on deduced install location and on the requested version from potential `global.json` file.
    /// If either no matching SDK version can be found, or that version exists, but it's missing the `dotnet.dll` file, this error code is returned.
    #[display(fmt = "Failure to find the requested SDK.")]
    LibHostSdkFindFailure,

    /// Arguments to `hostpolicy` are invalid.
    /// This is used in three unrelated places in the `hostpolicy`, but in all cases it means the component calling `hostpolicy` did something wrong:
    ///  - Command line arguments for the app - the failure would typically mean that wrong argument was passed or such.
    ///    For example if the application main assembly is not specified on the command line.
    ///    On its own this should not happen as `hostfxr` should have parsed and validated all command line arguments.
    ///  - `hostpolicy` context's `get_delegate` - if the requested delegate enum value is not recognized.
    ///    Again this would mean `hostfxr` passed the wrong value.
    ///  - `corehost_resolve_component_dependencies` - if something went wrong initializing `hostpolicy` internal structures.
    ///    Would happen for example when the `component_main_assembly_path` argument is wrong.
    #[display(fmt = "Arguments to hostpolicy are invalid.")]
    LibHostInvalidArgs,

    /// The `.runtimeconfig.json` file is invalid.
    /// The reasons for this failure can be among these:
    ///  - Failure to read from the file
    ///  - Invalid JSON
    ///  - Invalid value for a property (for example number for property which requires a string)
    ///  - Missing required property
    ///  - Other inconsistencies (for example `rollForward` and `applyPatches` are not allowed to be specified in the same config file)
    ///  - Any of the above failures reading the `.runtimecofig.dev.json` file
    ///  - Self-contained `.runtimeconfig.json` used in `hostfxr_initialize_for_runtime_config`.
    ///    Note that missing `.runtimconfig.json` is not an error (means self-contained app).
    ///    This error code is also used when there is a problem reading the CLSID map file in `comhost`.
    #[display(fmt = "Arguments to hostpolicy are invalid.")]
    InvalidConfigFile,

    /// Used internally when the command line for `dotnet.exe` doesn't contain path to the application to run.
    /// In such case the command line is considered to be a CLI/SDK command.
    /// This error code should never be returned to external caller.
    #[doc(hidden)]
    #[display(
        fmt = "[Internal error] The command line for dotnet.exe doesn't contain the path to the application to run."
    )]
    AppArgNotRunnable,

    /// `apphost` failed to determine which application to run.
    /// This can mean:
    ///  - The `apphost` binary has not been imprinted with the path to the app to run (so freshly built `apphost.exe` from the branch will fail to run like this)
    ///  - The `apphost` is a bundle (single-file exe) and it failed to extract correctly.
    #[display(fmt = "apphost failed to determine which application to run.")]
    AppHostExeNotBoundFailure,

    /// It was not possible to find a compatible framework version.
    /// This originates in `hostfxr` (`resolve_framework_reference`) and means that the app specified a reference to a framework in its `.runtimeconfig.json` which could not be resolved.
    /// The failure to resolve can mean that no such framework is available on the disk, or that the available frameworks don't match the minimum version specified or that the roll forward options specified excluded all available frameworks.
    /// Typically this would be used if a 3.0 app is trying to run on a machine which has no 3.0 installed.
    /// It would also be used for example if a 32bit 3.0 app is running on a machine which has 3.0 installed but only for 64bit.
    #[display(fmt = "It was not possible to find a compatible framework version.")]
    FrameworkMissingFailure,

    /// Returned by `hostfxr_get_native_search_directories` if the `hostpolicy` could not calculate the `NATIVE_DLL_SEARCH_DIRECTORIES`.
    #[display(fmt = "hostpolicy could not calculate the NATIVE_DLL_SEARCH_DIRECTORIES.")]
    HostApiFailed,

    /// Returned when the buffer specified to an API is not big enough to fit the requested value.
    /// Can be returned from:
    ///  - `hostfxr_get_runtime_properties`
    ///  - `hostfxr_get_native_search_directories`
    ///  - `get_hostfxr_path`
    #[display(
        fmt = "Returned when the buffer specified to an API is not big enough to fit the requested value."
    )]
    HostApiBufferTooSmall,

    /// Returned by `hostpolicy` if the `corehost_main_with_output_buffer` is called with unsupported host command.
    /// This error code means there is incompatibility between the `hostfxr` and `hostpolicy`.
    /// In reality this should pretty much never happen.
    #[display(
        fmt = "corehost_main_with_output_buffer was called with an unsupported host command."
    )]
    LibHostUnknownCommand,

    /// Returned by `apphost` if the imprinted application path doesn't exist.
    /// This would happen if the app is built with an executable (the `apphost`) and the main `app.dll` is missing.
    #[display(fmt = "The imprinted application path doesn't exist.")]
    LibHostAppRootFindFailure,

    /// Returned from `hostfxr_resolve_sdk2` when it fails to find matching SDK.
    /// Similar to `LibHostSdkFindFailure` but only used in the `hostfxr_resolve_sdk2`.
    #[display(fmt = "hostfxr_resolve_sdk2 failed to find a matching SDK.")]
    SdkResolverResolveFailure,

    /// During processing of `.runtimeconfig.json` there were two framework references to the same framework which were not compatible.
    /// This can happen if the app specified a framework reference to a lower-level framework which is also specified by a higher-level framework which is also used by the app.
    /// For example, this would happen if the app referenced `Microsoft.AspNet.App` version 2.0 and `Microsoft.NETCore.App` version 3.0. In such case the `Microsoft.AspNet.App` has `.runtimeconfig.json` which also references `Microsoft.NETCore.App` but it only allows versions 2.0 up to 2.9 (via roll forward options).
    /// So the version 3.0 requested by the app is incompatible.
    #[display(
        fmt = "During processing of `.runtimeconfig.json` there were two framework references to the same framework which were not compatible."
    )]
    FrameworkCompatFailure,

    /// Error used internally if the processing of framework references from `.runtimeconfig.json` reached a point where it needs to reprocess another already processed framework reference.
    /// If this error is returned to the external caller, it would mean there's a bug in the framework resolution algorithm.
    #[doc(hidden)]
    #[display(
        fmt = "[Internal error] The processing of framework references from .runtimeconfig.json reached a point where it needs to reprocess another already processed framework reference."
    )]
    FrameworkCompatRetry,

    /// Error reading the bundle footer metadata from a single-file `apphost`.
    /// This would mean a corrupted `apphost`.
    #[display(fmt = "Error reading the bundle footer metadata from a single-file apphost.")]
    AppHostExeNotBundle,

    /// Error extracting single-file `apphost` bundle.
    /// This is used in case of any error related to the bundle itself.
    /// Typically would mean a corrupted bundle.
    #[display(fmt = "Error extracting single-file apphost bundle.")]
    BundleExtractionFailure,

    /// Error reading or writing files during single-file `apphost` bundle extraction.
    #[display(
        fmt = "Error reading or writing files during single-file apphost bundle extraction."
    )]
    BundleExtractionIOError,

    /// The `.runtimeconfig.json` specified by the app contains a runtime property which is also produced by the hosting layer.
    /// For example if the `.runtimeconfig.json` would specify a property `TRUSTED_PLATFORM_ROOTS`, this error code would be returned.
    /// It is not allowed to specify properties which are otherwise populated by the hosting layer (`hostpolicy`) as there is not good way to resolve such conflicts.
    #[display(
        fmt = "The .runtimeconfig.json specified by the app contains a runtime property which is also produced by the hosting layer."
    )]
    LibHostDuplicateProperty,

    /// Feature which requires certain version of the hosting layer binaries was used on a version which doesn't support it.
    /// For example if COM component specified to run on 2.0 `Microsoft.NETCore.App` - as that contains older version of `hostpolicy` which doesn't support the necessary features to provide COM services.
    #[display(
        fmt = "Feature which requires certain version of the hosting layer binaries was used on a version which doesn't support it."
    )]
    HostApiUnsupportedVersion,

    /// Error code returned by the hosting APIs in `hostfxr` if the current state is incompatible with the requested operation.
    /// There are many such cases, please refer to the documentation of the hosting APIs for details.
    /// For example if `hostfxr_get_runtime_property_value` is called with the `host_context_handle` `nullptr` (meaning get property from the active runtime) but there's no active runtime in the process.
    #[display(fmt = "The current state is incompatible with the requested operation.")]
    HostInvalidState,

    /// Property requested by `hostfxr_get_runtime_property_value` doesn't exist.
    #[display(fmt = "Property requested by hostfxr_get_runtime_property_value doesn't exist.")]
    HostPropertyNotFound,

    /// Error returned by `hostfxr_initialize_for_runtime_config` if the component being initialized requires framework which is not available or incompatible with the frameworks loaded by the runtime already in the process.
    /// For example trying to load a component which requires 3.0 into a process which is already running a 2.0 runtime.
    #[display(
        fmt = "Error returned by hostfxr_initialize_for_runtime_config if the component being initialized requires framework which is not available or incompatible with the frameworks loaded by the runtime already in the process."
    )]
    CoreHostIncompatibleConfig,

    /// Error returned by `hostfxr_get_runtime_delegate` when `hostfxr` doesn't currently support requesting the given delegate type using the given context.
    #[display(
        fmt = "Requesting the given delegate type using the given context is currently not supported."
    )]
    HostApiUnsupportedScenario,

    /// Error returned by `hostfxr_get_runtime_delegate` when managed feature support for native host is disabled.
    #[display(fmt = "Managed feature support for native hosting is disabled")]
    HostFeatureDisabled,

    /// Unknown error status code.
    #[display(fmt = "Unknown error status code: {:#08X}", _0)]
    Unknown(u32),
}

impl std::error::Error for HostingError {}

impl HostingError {
    /// Creates a new [`HostingError`] from the raw status code.
    pub fn from_status_code(code: u32) -> Self {
        match Self::known_from_status_code(code) {
            Ok(s) => s,
            Err(code) => Self::Unknown(code),
        }
    }

    /// Tries to create a new [`HostingError`] from the raw status code if it is known.
    /// Otherwise returns the given value as an [`Err`].
    pub fn known_from_status_code(code: u32) -> Result<Self, u32> {
        match code {
            c if c == bindings::StatusCode::InvalidArgFailure as u32 => Ok(Self::InvalidArgFailure),
            c if c == bindings::StatusCode::CoreHostLibLoadFailure as u32 => {
                Ok(Self::CoreHostLibLoadFailure)
            }
            c if c == bindings::StatusCode::CoreHostLibMissingFailure as u32 => {
                Ok(Self::CoreHostLibMissingFailure)
            }
            c if c == bindings::StatusCode::CoreHostEntryPointFailure as u32 => {
                Ok(Self::CoreHostEntryPointFailure)
            }
            c if c == bindings::StatusCode::CoreHostCurHostFindFailure as u32 => {
                Ok(Self::CoreHostCurHostFindFailure)
            }
            c if c == bindings::StatusCode::CoreClrResolveFailure as u32 => {
                Ok(Self::CoreClrResolveFailure)
            }
            c if c == bindings::StatusCode::CoreClrBindFailure as u32 => {
                Ok(Self::CoreClrBindFailure)
            }
            c if c == bindings::StatusCode::CoreClrInitFailure as u32 => {
                Ok(Self::CoreClrInitFailure)
            }
            c if c == bindings::StatusCode::CoreClrExeFailure as u32 => Ok(Self::CoreClrExeFailure),
            c if c == bindings::StatusCode::ResolverInitFailure as u32 => {
                Ok(Self::ResolverInitFailure)
            }
            c if c == bindings::StatusCode::ResolverResolveFailure as u32 => {
                Ok(Self::ResolverResolveFailure)
            }
            c if c == bindings::StatusCode::LibHostCurExeFindFailure as u32 => {
                Ok(Self::LibHostCurExeFindFailure)
            }
            c if c == bindings::StatusCode::LibHostInitFailure as u32 => {
                Ok(Self::LibHostInitFailure)
            }
            c if c == bindings::StatusCode::LibHostExecModeFailure as u32 => {
                Ok(Self::LibHostExecModeFailure)
            }
            c if c == bindings::StatusCode::LibHostSdkFindFailure as u32 => {
                Ok(Self::LibHostSdkFindFailure)
            }
            c if c == bindings::StatusCode::LibHostInvalidArgs as u32 => {
                Ok(Self::LibHostInvalidArgs)
            }
            c if c == bindings::StatusCode::InvalidConfigFile as u32 => Ok(Self::InvalidConfigFile),
            c if c == bindings::StatusCode::AppArgNotRunnable as u32 => Ok(Self::AppArgNotRunnable),
            c if c == bindings::StatusCode::AppHostExeNotBoundFailure as u32 => {
                Ok(Self::AppHostExeNotBoundFailure)
            }
            c if c == bindings::StatusCode::FrameworkMissingFailure as u32 => {
                Ok(Self::FrameworkMissingFailure)
            }
            c if c == bindings::StatusCode::HostApiFailed as u32 => Ok(Self::HostApiFailed),
            c if c == bindings::StatusCode::HostApiBufferTooSmall as u32 => {
                Ok(Self::HostApiBufferTooSmall)
            }
            c if c == bindings::StatusCode::LibHostUnknownCommand as u32 => {
                Ok(Self::LibHostUnknownCommand)
            }
            c if c == bindings::StatusCode::LibHostAppRootFindFailure as u32 => {
                Ok(Self::LibHostAppRootFindFailure)
            }
            c if c == bindings::StatusCode::SdkResolverResolveFailure as u32 => {
                Ok(Self::SdkResolverResolveFailure)
            }
            c if c == bindings::StatusCode::FrameworkCompatFailure as u32 => {
                Ok(Self::FrameworkCompatFailure)
            }
            c if c == bindings::StatusCode::FrameworkCompatRetry as u32 => {
                Ok(Self::FrameworkCompatRetry)
            }
            c if c == bindings::StatusCode::BundleExtractionFailure as u32 => {
                Ok(Self::BundleExtractionFailure)
            }
            c if c == bindings::StatusCode::BundleExtractionIOError as u32 => {
                Ok(Self::BundleExtractionIOError)
            }
            c if c == bindings::StatusCode::LibHostDuplicateProperty as u32 => {
                Ok(Self::LibHostDuplicateProperty)
            }
            c if c == bindings::StatusCode::HostApiUnsupportedVersion as u32 => {
                Ok(Self::HostApiUnsupportedVersion)
            }
            c if c == bindings::StatusCode::HostInvalidState as u32 => Ok(Self::HostInvalidState),
            c if c == bindings::StatusCode::HostPropertyNotFound as u32 => {
                Ok(Self::HostPropertyNotFound)
            }
            c if c == bindings::StatusCode::CoreHostIncompatibleConfig as u32 => {
                Ok(Self::CoreHostIncompatibleConfig)
            }
            c if c == bindings::StatusCode::HostApiUnsupportedScenario as u32 => {
                Ok(Self::HostApiUnsupportedScenario)
            }
            c if c == bindings::StatusCode::HostFeatureDisabled as u32 => {
                Ok(Self::HostFeatureDisabled)
            }
            _ => Err(code),
        }
    }

    /// Returns the underlying status code value.
    #[must_use]
    pub fn value(&self) -> u32 {
        match self {
            Self::InvalidArgFailure => bindings::StatusCode::InvalidArgFailure as u32,
            Self::CoreHostLibLoadFailure => bindings::StatusCode::CoreHostLibLoadFailure as u32,
            Self::CoreHostLibMissingFailure => {
                bindings::StatusCode::CoreHostLibMissingFailure as u32
            }
            Self::CoreHostEntryPointFailure => {
                bindings::StatusCode::CoreHostEntryPointFailure as u32
            }
            Self::CoreHostCurHostFindFailure => {
                bindings::StatusCode::CoreHostCurHostFindFailure as u32
            }
            Self::CoreClrResolveFailure => bindings::StatusCode::CoreClrResolveFailure as u32,
            Self::CoreClrBindFailure => bindings::StatusCode::CoreClrBindFailure as u32,
            Self::CoreClrInitFailure => bindings::StatusCode::CoreClrInitFailure as u32,
            Self::CoreClrExeFailure => bindings::StatusCode::CoreClrExeFailure as u32,
            Self::ResolverInitFailure => bindings::StatusCode::ResolverInitFailure as u32,
            Self::ResolverResolveFailure => bindings::StatusCode::ResolverResolveFailure as u32,
            Self::LibHostCurExeFindFailure => bindings::StatusCode::LibHostCurExeFindFailure as u32,
            Self::LibHostInitFailure => bindings::StatusCode::LibHostInitFailure as u32,
            Self::LibHostExecModeFailure => bindings::StatusCode::LibHostExecModeFailure as u32,
            Self::LibHostSdkFindFailure => bindings::StatusCode::LibHostSdkFindFailure as u32,
            Self::LibHostInvalidArgs => bindings::StatusCode::LibHostInvalidArgs as u32,
            Self::InvalidConfigFile => bindings::StatusCode::InvalidConfigFile as u32,
            Self::AppArgNotRunnable => bindings::StatusCode::AppArgNotRunnable as u32,
            Self::AppHostExeNotBoundFailure => {
                bindings::StatusCode::AppHostExeNotBoundFailure as u32
            }
            Self::FrameworkMissingFailure => bindings::StatusCode::FrameworkMissingFailure as u32,
            Self::HostApiFailed => bindings::StatusCode::HostApiFailed as u32,
            Self::HostApiBufferTooSmall => bindings::StatusCode::HostApiBufferTooSmall as u32,
            Self::LibHostUnknownCommand => bindings::StatusCode::LibHostUnknownCommand as u32,
            Self::LibHostAppRootFindFailure => {
                bindings::StatusCode::LibHostAppRootFindFailure as u32
            }
            Self::SdkResolverResolveFailure => {
                bindings::StatusCode::SdkResolverResolveFailure as u32
            }
            Self::FrameworkCompatFailure => bindings::StatusCode::FrameworkCompatFailure as u32,
            Self::FrameworkCompatRetry => bindings::StatusCode::FrameworkCompatRetry as u32,
            Self::AppHostExeNotBundle => bindings::StatusCode::AppHostExeNotBundle as u32,
            Self::BundleExtractionFailure => bindings::StatusCode::BundleExtractionFailure as u32,
            Self::BundleExtractionIOError => bindings::StatusCode::BundleExtractionIOError as u32,
            Self::LibHostDuplicateProperty => bindings::StatusCode::LibHostDuplicateProperty as u32,
            Self::HostApiUnsupportedVersion => {
                bindings::StatusCode::HostApiUnsupportedVersion as u32
            }
            Self::HostInvalidState => bindings::StatusCode::HostInvalidState as u32,
            Self::HostPropertyNotFound => bindings::StatusCode::HostPropertyNotFound as u32,
            Self::CoreHostIncompatibleConfig => {
                bindings::StatusCode::CoreHostIncompatibleConfig as u32
            }
            Self::HostApiUnsupportedScenario => {
                bindings::StatusCode::HostApiUnsupportedScenario as u32
            }
            Self::HostFeatureDisabled => bindings::StatusCode::HostFeatureDisabled as u32,
            Self::Unknown(code) => *code,
        }
    }

    /// Returns whether the status code of this error has a known meaning.
    #[must_use]
    pub fn is_known(&self) -> bool {
        !self.is_unknown()
    }

    /// Returns whether the status code of this error has a unknown meaning.
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown(_))
    }
}

impl TryFrom<u32> for HostingError {
    type Error = u32;

    fn try_from(code: u32) -> Result<Self, Self::Error> {
        Self::known_from_status_code(code)
    }
}

impl From<HostingError> for u32 {
    fn from(code: HostingError) -> Self {
        code.value()
    }
}
