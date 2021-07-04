use crate::{
    bindings::{
        char_t,
        hostfxr::{hostfxr_handle, hostfxr_initialize_parameters, HostfxrLib},
    },
    pdcstring::PdCStr,
    Error,
};
use dlopen::wrapper::Container;
use std::{
    ffi::OsStr,
    mem::MaybeUninit,
    ptr::{self, NonNull},
};

use super::{HostExitCode, HostfxrContext, InitializedForCommandLine, InitializedForRuntimeConfig};

pub struct Hostfxr {
    pub(crate) lib: Container<HostfxrLib>,
}

#[derive(Debug, Clone, Copy)]
pub struct HostfxrHandle(NonNull<()>);

impl HostfxrHandle {
    pub unsafe fn new(ptr: hostfxr_handle) -> Option<Self> {
        NonNull::new(ptr as *mut _).map(Self)
    }
    pub unsafe fn new_unchecked(ptr: hostfxr_handle) -> Self {
        Self(NonNull::new_unchecked(ptr as *mut _))
    }
    pub fn as_raw(&self) -> hostfxr_handle {
        self.0.as_ptr()
    }
}

impl From<HostfxrHandle> for hostfxr_handle {
    fn from(handle: HostfxrHandle) -> Self {
        handle.as_raw()
    }
}

impl Hostfxr {
    pub fn load_from_path(path: impl AsRef<OsStr>) -> Result<Self, Error> {
        Ok(Self {
            lib: unsafe { Container::load(path)? },
        })
    }

    pub fn initialize_for_dotnet_command_line(
        &self,
        app_path: impl AsRef<PdCStr>,
    ) -> Result<HostfxrContext<InitializedForCommandLine>, Error> {
        self.initialize_for_dotnet_command_line_with_args(&[app_path.as_ref()])
    }
    pub fn initialize_for_dotnet_command_line_and_host_path(
        &self,
        app_path: impl AsRef<PdCStr>,
        host_path: impl AsRef<PdCStr>,
    ) -> Result<HostfxrContext<InitializedForCommandLine>, Error> {
        self.initialize_for_dotnet_command_line_with_args_and_host_path(
            &[app_path.as_ref()],
            host_path,
        )
    }
    pub fn initialize_for_dotnet_command_line_and_dotnet_root(
        &self,
        app_path: impl AsRef<PdCStr>,
        dotnet_root: impl AsRef<PdCStr>,
    ) -> Result<HostfxrContext<InitializedForCommandLine>, Error> {
        self.initialize_for_dotnet_command_line_with_args_and_dotnet_root(
            &[app_path.as_ref()],
            dotnet_root,
        )
    }

    pub fn initialize_for_dotnet_command_line_with_args(
        &self,
        args: &[&PdCStr],
    ) -> Result<HostfxrContext<InitializedForCommandLine>, Error> {
        unsafe {
            self.initialize_for_dotnet_command_line_with_parameters(args.as_ref(), ptr::null())
        }
    }
    pub fn initialize_for_dotnet_command_line_with_args_and_host_path(
        &self,
        args: &[&PdCStr],
        host_path: impl AsRef<PdCStr>,
    ) -> Result<HostfxrContext<InitializedForCommandLine>, Error> {
        let parameters = hostfxr_initialize_parameters::with_host_path(host_path.as_ref().as_ptr());
        unsafe { self.initialize_for_dotnet_command_line_with_parameters(args, &parameters) }
    }
    pub fn initialize_for_dotnet_command_line_with_args_and_dotnet_root(
        &self,
        args: &[&PdCStr],
        dotnet_root: impl AsRef<PdCStr>,
    ) -> Result<HostfxrContext<InitializedForCommandLine>, Error> {
        let parameters =
            hostfxr_initialize_parameters::with_dotnet_root(dotnet_root.as_ref().as_ptr());
        unsafe { self.initialize_for_dotnet_command_line_with_parameters(args, &parameters) }
    }
    unsafe fn initialize_for_dotnet_command_line_with_parameters(
        &self,
        args: &[&PdCStr],
        parameters: *const hostfxr_initialize_parameters,
    ) -> Result<HostfxrContext<InitializedForCommandLine>, Error> {
        let mut hostfxr_handle = MaybeUninit::<hostfxr_handle>::uninit();

        let result = self.lib.hostfxr_initialize_for_dotnet_command_line(
            args.len() as i32,
            args.as_ptr() as *const *const char_t,
            parameters,
            hostfxr_handle.as_mut_ptr(),
        );

        HostExitCode::from(result).to_result()?;

        Ok(HostfxrContext::new(
            HostfxrHandle::new_unchecked(hostfxr_handle.assume_init()),
            self,
        ))
    }

    pub fn initialize_for_runtime_config(
        &self,
        runtime_config_path: impl AsRef<PdCStr>,
    ) -> Result<HostfxrContext<InitializedForRuntimeConfig>, Error> {
        unsafe {
            self.initialize_for_runtime_config_with_parameters(runtime_config_path, ptr::null())
        }
    }
    pub fn initialize_for_runtime_config_with_host_path(
        &self,
        runtime_config_path: impl AsRef<PdCStr>,
        host_path: impl AsRef<PdCStr>,
    ) -> Result<HostfxrContext<InitializedForRuntimeConfig>, Error> {
        let parameters = hostfxr_initialize_parameters::with_host_path(host_path.as_ref().as_ptr());
        unsafe {
            self.initialize_for_runtime_config_with_parameters(runtime_config_path, &parameters)
        }
    }
    pub fn initialize_for_runtime_config_with_dotnet_root(
        &self,
        runtime_config_path: impl AsRef<PdCStr>,
        dotnet_root: impl AsRef<PdCStr>,
    ) -> Result<HostfxrContext<InitializedForRuntimeConfig>, Error> {
        let parameters =
            hostfxr_initialize_parameters::with_dotnet_root(dotnet_root.as_ref().as_ptr());
        unsafe {
            self.initialize_for_runtime_config_with_parameters(runtime_config_path, &parameters)
        }
    }

    unsafe fn initialize_for_runtime_config_with_parameters(
        &self,
        runtime_config_path: impl AsRef<PdCStr>,
        parameters: *const hostfxr_initialize_parameters,
    ) -> Result<HostfxrContext<InitializedForRuntimeConfig>, Error> {
        let mut hostfxr_handle = MaybeUninit::uninit();

        let result = self.lib.hostfxr_initialize_for_runtime_config(
            runtime_config_path.as_ref().as_ptr(),
            parameters,
            hostfxr_handle.as_mut_ptr(),
        );

        HostExitCode::from(result).to_result()?;

        Ok(HostfxrContext::new(
            HostfxrHandle::new_unchecked(hostfxr_handle.assume_init()),
            self,
        ))
    }
}
