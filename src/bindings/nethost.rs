use std::{mem::size_of, ptr};

use dlopen::wrapper::WrapperApi;

use super::type_aliases::{char_t, size_t};

// #[link(name = "nethost")]
extern "C" {
    /// Get the path to the hostfxr library
    ///
    /// # Arguments
    ///  * `buffer`:
    ///     Buffer that will be populated with the hostfxr path, including a null terminator.
    ///  * `buffer_size`:
    ///     * \[in\] Size of buffer in [`char_t`] units.
    ///     * \[out\] Size of buffer used in [`char_t`] units. If the input value is too small
    ///           or `buffer` is [`null`](ptr::null()), this is populated with the minimum required size
    ///           in [`char_t`] units for a buffer to hold the hostfxr path
    ///
    /// * `get_hostfxr_parameters`:
    ///     Optional. Parameters that modify the behaviour for locating the hostfxr library.
    ///     If [`null`](ptr::null()), hostfxr is located using the enviroment variable or global registration
    ///
    /// # Return value
    ///  * 0 on success, otherwise failure
    ///  * 0x80008098 - `buffer` is too small ([`HostApiBufferTooSmall`])
    ///
    /// # Remarks
    /// The full search for the hostfxr library is done on every call. To minimize the need
    /// to call this function multiple times, pass a large buffer (e.g. [`PATH_MAX`]).
    ///
    /// [HostApiBufferTooSmall](super::error::KnownHostExitCode::HostApiBufferTooSmall)
    pub fn get_hostfxr_path(
        buffer: *mut char_t,
        buffer_size: *mut size_t,
        parameters: *const get_hostfxr_parameters,
    ) -> i32;
}

#[derive(WrapperApi)]
pub struct NethostLib {
    /// Get the path to the hostfxr library
    ///
    /// # Arguments
    ///  * `buffer`:
    ///     Buffer that will be populated with the hostfxr path, including a null terminator.
    ///  * `buffer_size`:
    ///     * \[in\] Size of buffer in [`char_t`] units.
    ///     * \[out\] Size of buffer used in [`char_t`] units. If the input value is too small
    ///           or `buffer` is [`null`](ptr::null()), this is populated with the minimum required size
    ///           in [`char_t`] units for a buffer to hold the hostfxr path
    ///
    /// * `get_hostfxr_parameters`:
    ///     Optional. Parameters that modify the behaviour for locating the hostfxr library.
    ///     If [`null`](ptr::null()), hostfxr is located using the enviroment variable or global registration
    ///
    /// # Return value
    ///  * 0 on success, otherwise failure
    ///  * 0x80008098 - `buffer` is too small ([`HostApiBufferTooSmall`])
    ///
    /// # Remarks
    /// The full search for the hostfxr library is done on every call. To minimize the need
    /// to call this function multiple times, pass a large buffer (e.g. [`PATH_MAX`]).
    ///
    /// [HostApiBufferTooSmall](super::error::KnownHostExitCode::HostApiBufferTooSmall)
    get_hostfxr_path: unsafe extern "C" fn(
        buffer: *mut char_t,
        buffer_size: *mut size_t,
        parameters: *const get_hostfxr_parameters,
    ) -> i32,
}

/// Parameters for `get_hostfxr_path`
#[repr(C)]
#[allow(non_camel_case_types, dead_code)]
pub struct get_hostfxr_parameters {
    /// Size of the struct. This is used for versioning.
    pub size: size_t,
    /// Path to the compenent's assembly.
    /// If specified, hostfxr is located as if the `assembly_path` is the apphost
    pub assembly_path: *const char_t,
    /// Path to directory containing the dotnet executable.
    /// If specified, hostfxr is located as if an application is started using
    /// 'dotnet app.dll', which means it will be searched for under the `dotnet_root`
    /// path and the `assembly_path` is ignored.
    pub dotnet_root: *const char_t,
}

impl get_hostfxr_parameters {
    pub fn with_dotnet_root(dotnet_root: *const char_t) -> get_hostfxr_parameters {
        get_hostfxr_parameters {
            size: size_of::<get_hostfxr_parameters>(),
            assembly_path: ptr::null(),
            dotnet_root,
        }
    }
    pub fn with_assembly_path(assembly_path: *const char_t) -> get_hostfxr_parameters {
        get_hostfxr_parameters {
            size: size_of::<get_hostfxr_parameters>(),
            assembly_path,
            dotnet_root: ptr::null(),
        }
    }
}
