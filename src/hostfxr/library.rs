use crate::{
    bindings::{
        char_t,
        hostfxr::{hostfxr_handle, hostfxr_initialize_parameters, HostfxrLib},
    },
    nethost,
    pdcstring::PdCStr,
    Error,
};
use dlopen::wrapper::Container;
use std::{ffi::OsStr, mem::MaybeUninit, ptr};

use super::{
    HostExitCode, HostfxrContext, HostfxrHandle, InitializedForCommandLine,
    InitializedForRuntimeConfig,
};

/// A struct representing a loaded hostfxr library.
pub struct Hostfxr {
    pub(crate) lib: Container<HostfxrLib>,
}

impl !Sync for Hostfxr {}
impl !Send for Hostfxr {}

impl Hostfxr {
    /// Loads the hostfxr library from the given path.
    pub fn load_from_path(path: impl AsRef<OsStr>) -> Result<Self, Error> {
        Ok(Self {
            lib: unsafe { Container::load(path)? },
        })
    }

    /// Locates the hostfxr library using [`nethost`] and loads it.
    pub fn load_with_nethost() -> Result<Self, Error> {
        nethost::load_hostfxr()
    }

    /// Initializes the hosting components for a dotnet command line running an application
    ///
    /// # Remarks
    /// This function parses the specified command-line arguments to determine the application to run. It will
    /// then find the corresponding `.runtimeconfig.json` and `.deps.json` with which to resolve frameworks and
    /// dependencies and prepare everything needed to load the runtime.
    ///
    /// This function only supports arguments for running an application. It does not support SDK commands.
    ///
    /// This function does not load the runtime.
    pub fn initialize_for_dotnet_command_line(
        &self,
        app_path: impl AsRef<PdCStr>,
    ) -> Result<HostfxrContext<InitializedForCommandLine>, Error> {
        self.initialize_for_dotnet_command_line_with_args(&[app_path.as_ref()])
    }

    /// Initializes the hosting components for a dotnet command line running an application
    ///
    /// # Remarks
    /// This function parses the specified command-line arguments to determine the application to run. It will
    /// then find the corresponding `.runtimeconfig.json` and `.deps.json` with which to resolve frameworks and
    /// dependencies and prepare everything needed to load the runtime.
    ///
    /// This function only supports arguments for running an application. It does not support SDK commands.
    ///
    /// This function does not load the runtime.
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
