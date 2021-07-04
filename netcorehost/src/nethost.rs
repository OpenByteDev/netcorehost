use crate::pdcstring::PdCStr;

use crate::{
    bindings::{char_t, consts::PATH_MAX, nethost::get_hostfxr_parameters},
    error::Error,
    hostfxr::Hostfxr,
    HostExitCode,
};
use std::{ffi::OsString, mem::MaybeUninit, ptr};

/// Gets the path to the hostfxr library.
pub fn get_hostfxr_path() -> Result<OsString, Error> {
    unsafe { get_hostfxr_path_with_parameters(ptr::null()) }
}

/// Gets the path to the hostfxr library.
/// Hostfxr is located as if the `assembly_path` is the apphost.
pub fn get_hostfxr_path_with_assembly_path<P: AsRef<PdCStr>>(
    assembly_path: P,
) -> Result<OsString, Error> {
    let parameters = get_hostfxr_parameters::with_assembly_path(assembly_path.as_ref().as_ptr());
    unsafe { get_hostfxr_path_with_parameters(&parameters) }
}

/// Gets the path to the hostfxr library.
/// Hostfxr is located as if an application is started using 'dotnet app.dll', which means it will be
/// searched for under the `dotnet_root` path.
pub fn get_hostfxr_path_with_dotnet_root<P: AsRef<PdCStr>>(
    dotnet_root: P,
) -> Result<OsString, Error> {
    let parameters = get_hostfxr_parameters::with_dotnet_root(dotnet_root.as_ref().as_ptr());
    unsafe { get_hostfxr_path_with_parameters(&parameters) }
}

unsafe fn get_hostfxr_path_with_parameters(
    parameters: *const get_hostfxr_parameters,
) -> Result<OsString, Error> {
    let mut path_buffer = MaybeUninit::uninit_array::<PATH_MAX>();
    let mut path_length = path_buffer.len();

    let result = crate::bindings::nethost::get_hostfxr_path(
        path_buffer.as_mut_ptr() as *mut char_t,
        &mut path_length,
        parameters,
    );
    HostExitCode::from(result).to_result()?;

    let path_slice = MaybeUninit::slice_assume_init_ref(&path_buffer[..path_length]);
    Ok(PdCStr::from_slice_with_nul_unchecked(path_slice).to_os_string())
}

/// Retrieves the path to the hostfxr library and loads it.
pub fn load_hostfxr() -> Result<Hostfxr, Error> {
    let hostfxr_path = get_hostfxr_path()?;
    Hostfxr::load_from_path(hostfxr_path)
}

/// Retrieves the path to the hostfxr library and loads it.
/// Hostfxr is located as if the `assembly_path` is the apphost.
pub fn load_hostfxr_with_assembly_path<P: AsRef<PdCStr>>(
    assembly_path: P,
) -> Result<Hostfxr, Error> {
    let hostfxr_path = get_hostfxr_path_with_assembly_path(assembly_path)?;
    Hostfxr::load_from_path(hostfxr_path)
}

/// Retrieves the path to the hostfxr library and loads it.
/// Hostfxr is located as if an application is started using 'dotnet app.dll', which means it will be
/// searched for under the `dotnet_root` path.
pub fn load_hostfxr_with_dotnet_root<P: AsRef<PdCStr>>(dotnet_root: P) -> Result<Hostfxr, Error> {
    let hostfxr_path = get_hostfxr_path_with_dotnet_root(dotnet_root)?;
    Hostfxr::load_from_path(hostfxr_path)
}
