use crate::{
    bindings::{nethost::get_hostfxr_parameters, MAX_PATH},
    error::{HostingError, HostingResult, HostingSuccess},
    hostfxr::Hostfxr,
    pdcstring::{self, PdCStr},
};
use std::{ffi::OsString, ptr};
use thiserror::Error;

/// Gets the path to the hostfxr library.
pub fn get_hostfxr_path() -> Result<OsString, HostingError> {
    unsafe { get_hostfxr_path_with_parameters(ptr::null()) }
}

/// Gets the path to the hostfxr library.
/// Hostfxr is located as if the `assembly_path` is the apphost.
pub fn get_hostfxr_path_with_assembly_path<P: AsRef<PdCStr>>(
    assembly_path: P,
) -> Result<OsString, HostingError> {
    let parameters = get_hostfxr_parameters::with_assembly_path(assembly_path.as_ref().as_ptr());
    unsafe { get_hostfxr_path_with_parameters(&parameters) }
}

/// Gets the path to the hostfxr library.
/// Hostfxr is located as if an application is started using `dotnet app.dll`, which means it will be
/// searched for under the `dotnet_root` path.
pub fn get_hostfxr_path_with_dotnet_root<P: AsRef<PdCStr>>(
    dotnet_root: P,
) -> Result<OsString, HostingError> {
    let parameters = get_hostfxr_parameters::with_dotnet_root(dotnet_root.as_ref().as_ptr());
    unsafe { get_hostfxr_path_with_parameters(&parameters) }
}

unsafe fn get_hostfxr_path_with_parameters(
    parameters: *const get_hostfxr_parameters,
) -> Result<OsString, HostingError> {
    let mut path_buffer = [0; MAX_PATH];
    let mut path_length = path_buffer.len();

    let result = unsafe {
        crate::bindings::nethost::get_hostfxr_path(
            path_buffer.as_mut_ptr().cast(),
            &mut path_length,
            parameters,
        )
    };

    match HostingResult::from(result).into_result() {
        Ok(_) => {
            let path_slice = &path_buffer[..path_length];
            Ok(unsafe { PdCStr::from_slice_with_nul_unchecked(path_slice) }.to_os_string())
        }
        Err(HostingError::HostApiBufferTooSmall) => {
            let mut path_vec: Vec<pdcstring::PdUChar> = Vec::new();
            path_vec.resize(path_length, 0);

            let result = unsafe {
                crate::bindings::nethost::get_hostfxr_path(
                    &mut path_vec[0],
                    &mut path_length,
                    parameters,
                )
            };
            assert_eq!(result as u32, HostingSuccess::Success.value());

            let path_slice = &path_vec[..path_length];
            Ok(unsafe { PdCStr::from_slice_with_nul_unchecked(path_slice) }.to_os_string())
        }
        Err(err) => Err(err),
    }
}

/// Retrieves the path to the hostfxr library and loads it.
pub fn load_hostfxr() -> Result<Hostfxr, LoadHostfxrError> {
    let hostfxr_path = get_hostfxr_path()?;
    let hostfxr = Hostfxr::load_from_path(hostfxr_path)?;
    Ok(hostfxr)
}

/// Retrieves the path to the hostfxr library and loads it.
/// Hostfxr is located as if the `assembly_path` is the apphost.
pub fn load_hostfxr_with_assembly_path<P: AsRef<PdCStr>>(
    assembly_path: P,
) -> Result<Hostfxr, LoadHostfxrError> {
    let hostfxr_path = get_hostfxr_path_with_assembly_path(assembly_path)?;
    let hostfxr = Hostfxr::load_from_path(hostfxr_path)?;
    Ok(hostfxr)
}

/// Retrieves the path to the hostfxr library and loads it.
/// Hostfxr is located as if an application is started using `dotnet app.dll`, which means it will be
/// searched for under the `dotnet_root` path.
pub fn load_hostfxr_with_dotnet_root<P: AsRef<PdCStr>>(
    dotnet_root: P,
) -> Result<Hostfxr, LoadHostfxrError> {
    let hostfxr_path = get_hostfxr_path_with_dotnet_root(dotnet_root)?;
    let hostfxr = Hostfxr::load_from_path(hostfxr_path)?;
    Ok(hostfxr)
}

/// Enum for errors that can occur while locating and loading the hostfxr library.
#[derive(Debug, Error)]
pub enum LoadHostfxrError {
    /// An error occured inside the hosting components.
    #[error(transparent)]
    Hosting(#[from] HostingError),
    /// An error occured while loading the hostfxr library.
    #[error(transparent)]
    DlOpen(#[from] crate::dlopen::Error),
}
