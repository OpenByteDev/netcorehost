use crate::{
    bindings::{
        char_t,
        hostfxr::{hostfxr_handle, hostfxr_initialize_parameters, HostfxrLib},
    },
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

    #[cfg(feature = "nethost")]
    /// Locates the hostfxr library using [`nethost`] and loads it.
    pub fn load_with_nethost() -> Result<Self, Error> {
        crate::nethost::load_hostfxr()
    }

    /// Initializes the hosting components for a dotnet command line running an application
    ///
    /// Like all the other `initialize` functions, this function will
    /// * Process the `.runtimeconfig.json`
    /// * Resolve framework references and find actual frameworks
    /// * Find the root framework (`Microsoft.NETCore.App`) and load the hostpolicy from it
    /// * The hostpolicy will then process all relevant `.deps.json` files and produce the list of assemblies, native search paths and other artifacts needed to initialize the runtime.
    ///
    /// The functions will **NOT** load the CoreCLR runtime. They just prepare everything to the point where it can be loaded.
    ///
    /// # Arguments
    ///  * `app_path`:
    ///     The path to the target application.
    ///
    /// # Remarks
    /// This function parses the specified command-line arguments to determine the application to run. It will
    /// then find the corresponding `.runtimeconfig.json` and `.deps.json` with which to resolve frameworks and
    /// dependencies and prepare everything needed to load the runtime.
    pub fn initialize_for_dotnet_command_line(
        &self,
        app_path: impl AsRef<PdCStr>,
    ) -> Result<HostfxrContext<InitializedForCommandLine>, Error> {
        self.initialize_for_dotnet_command_line_with_args(&[app_path.as_ref()])
    }

    /// Initializes the hosting components for a dotnet command line running an application
    ///
    /// Like all the other `initialize` functions, this function will
    /// * Process the `.runtimeconfig.json`
    /// * Resolve framework references and find actual frameworks
    /// * Find the root framework (`Microsoft.NETCore.App`) and load the hostpolicy from it
    /// * The hostpolicy will then process all relevant `.deps.json` files and produce the list of assemblies, native search paths and other artifacts needed to initialize the runtime.
    ///
    /// The functions will **NOT** load the CoreCLR runtime. They just prepare everything to the point where it can be loaded.
    ///
    /// # Arguments
    ///  * `app_path`:
    ///     The path to the target application.
    ///  * `host_path`:
    ///     Path to the native host (typically the `.exe`).
    ///     This value is not used for anything by the hosting components.
    ///     It's just passed to the CoreCLR as the path to the executable.
    ///     It can point to a file which is not executable itself, if such file doesn't exist (for example in COM activation scenarios this points to the `comhost.dll`).
    ///     This is used by PAL to initialize internal command line structures, process name and so on.
    ///
    /// # Remarks
    /// This function parses the specified command-line arguments to determine the application to run. It will
    /// then find the corresponding `.runtimeconfig.json` and `.deps.json` with which to resolve frameworks and
    /// dependencies and prepare everything needed to load the runtime.
    pub fn initialize_for_dotnet_command_line_with_host_path(
        &self,
        app_path: impl AsRef<PdCStr>,
        host_path: impl AsRef<PdCStr>,
    ) -> Result<HostfxrContext<InitializedForCommandLine>, Error> {
        self.initialize_for_dotnet_command_line_with_args_and_host_path(
            &[app_path.as_ref()],
            host_path,
        )
    }

    /// Initializes the hosting components for a dotnet command line running an application
    ///
    /// Like all the other `initialize` functions, this function will
    /// * Process the `.runtimeconfig.json`
    /// * Resolve framework references and find actual frameworks
    /// * Find the root framework (`Microsoft.NETCore.App`) and load the hostpolicy from it
    /// * The hostpolicy will then process all relevant `.deps.json` files and produce the list of assemblies, native search paths and other artifacts needed to initialize the runtime.
    ///
    /// The functions will **NOT** load the CoreCLR runtime. They just prepare everything to the point where it can be loaded.
    ///
    /// # Arguments
    ///  * `app_path`:
    ///     The path to the target application.
    ///  * `dotnet_root`:
    ///     Path to the root of the .NET Core installation in use.
    ///     This typically points to the install location from which the hostfxr has been loaded.
    ///     For example on Windows this would typically point to `C:\Program Files\dotnet`.
    ///     The path is used to search for shared frameworks and potentially SDKs.
    ///
    /// # Remarks
    /// This function parses the specified command-line arguments to determine the application to run. It will
    /// then find the corresponding `.runtimeconfig.json` and `.deps.json` with which to resolve frameworks and
    /// dependencies and prepare everything needed to load the runtime.
    pub fn initialize_for_dotnet_command_line_with_dotnet_root(
        &self,
        app_path: impl AsRef<PdCStr>,
        dotnet_root: impl AsRef<PdCStr>,
    ) -> Result<HostfxrContext<InitializedForCommandLine>, Error> {
        self.initialize_for_dotnet_command_line_with_args_and_dotnet_root(
            &[app_path.as_ref()],
            dotnet_root,
        )
    }

    /// Initializes the hosting components for a dotnet command line running an application
    ///
    /// Like all the other `initialize` functions, this function will
    /// * Process the `.runtimeconfig.json`
    /// * Resolve framework references and find actual frameworks
    /// * Find the root framework (`Microsoft.NETCore.App`) and load the hostpolicy from it
    /// * The hostpolicy will then process all relevant `.deps.json` files and produce the list of assemblies, native search paths and other artifacts needed to initialize the runtime.
    ///
    /// The functions will **NOT** load the CoreCLR runtime. They just prepare everything to the point where it can be loaded.
    ///
    /// # Arguments
    ///  * `args`:
    ///     The command line for running a managed application.
    ///     These represent the arguments which would have been passed to the muxer if the app was being run from the command line.
    ///     Note that the first argument has to be the path of the target app.
    ///
    /// # Remarks
    /// This function parses the specified command-line arguments to determine the application to run. It will
    /// then find the corresponding `.runtimeconfig.json` and `.deps.json` with which to resolve frameworks and
    /// dependencies and prepare everything needed to load the runtime.
    pub fn initialize_for_dotnet_command_line_with_args(
        &self,
        args: &[&PdCStr],
    ) -> Result<HostfxrContext<InitializedForCommandLine>, Error> {
        unsafe {
            self.initialize_for_dotnet_command_line_with_parameters(args.as_ref(), ptr::null())
        }
    }

    /// Initializes the hosting components for a dotnet command line running an application
    ///
    /// Like all the other `initialize` functions, this function will
    /// * Process the `.runtimeconfig.json`
    /// * Resolve framework references and find actual frameworks
    /// * Find the root framework (`Microsoft.NETCore.App`) and load the hostpolicy from it
    /// * The hostpolicy will then process all relevant `.deps.json` files and produce the list of assemblies, native search paths and other artifacts needed to initialize the runtime.
    ///
    /// The functions will **NOT** load the CoreCLR runtime. They just prepare everything to the point where it can be loaded.
    ///
    /// # Arguments
    ///  * `args`:
    ///     The command line for running a managed application.
    ///     These represent the arguments which would have been passed to the muxer if the app was being run from the command line.
    ///     Note that the first argument has to be the path of the target app.
    ///  * `host_path`:
    ///     Path to the native host (typically the `.exe`).
    ///     This value is not used for anything by the hosting components.
    ///     It's just passed to the CoreCLR as the path to the executable.
    ///     It can point to a file which is not executable itself, if such file doesn't exist (for example in COM activation scenarios this points to the `comhost.dll`).
    ///     This is used by PAL to initialize internal command line structures, process name and so on.
    ///
    /// # Remarks
    /// This function parses the specified command-line arguments to determine the application to run. It will
    /// then find the corresponding `.runtimeconfig.json` and `.deps.json` with which to resolve frameworks and
    /// dependencies and prepare everything needed to load the runtime.
    pub fn initialize_for_dotnet_command_line_with_args_and_host_path(
        &self,
        args: &[&PdCStr],
        host_path: impl AsRef<PdCStr>,
    ) -> Result<HostfxrContext<InitializedForCommandLine>, Error> {
        let parameters = hostfxr_initialize_parameters::with_host_path(host_path.as_ref().as_ptr());
        unsafe { self.initialize_for_dotnet_command_line_with_parameters(args, &parameters) }
    }

    /// Initializes the hosting components for a dotnet command line running an application
    ///
    /// Like all the other `initialize` functions, this function will
    /// * Process the `.runtimeconfig.json`
    /// * Resolve framework references and find actual frameworks
    /// * Find the root framework (`Microsoft.NETCore.App`) and load the hostpolicy from it
    /// * The hostpolicy will then process all relevant `.deps.json` files and produce the list of assemblies, native search paths and other artifacts needed to initialize the runtime.
    ///
    /// The functions will **NOT** load the CoreCLR runtime. They just prepare everything to the point where it can be loaded.
    ///
    /// # Arguments
    ///  * `args`:
    ///     The command line for running a managed application.
    ///     These represent the arguments which would have been passed to the muxer if the app was being run from the command line.
    ///     Note that the first argument has to be the path of the target app.
    ///  * `dotnet_root`:
    ///     Path to the root of the .NET Core installation in use.
    ///     This typically points to the install location from which the hostfxr has been loaded.
    ///     For example on Windows this would typically point to `C:\Program Files\dotnet`.
    ///     The path is used to search for shared frameworks and potentially SDKs.
    ///
    /// # Remarks
    /// This function parses the specified command-line arguments to determine the application to run. It will
    /// then find the corresponding `.runtimeconfig.json` and `.deps.json` with which to resolve frameworks and
    /// dependencies and prepare everything needed to load the runtime.
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

        let result = unsafe {
            self.lib.hostfxr_initialize_for_dotnet_command_line(
                args.len() as i32,
                args.as_ptr() as *const *const char_t,
                parameters,
                hostfxr_handle.as_mut_ptr(),
            )
        };

        HostExitCode::from(result).to_result()?;

        Ok(HostfxrContext::new(
            unsafe { HostfxrHandle::new_unchecked(hostfxr_handle.assume_init()) },
            self,
        ))
    }

    /// This function loads the specified `.runtimeconfig.json`, resolve all frameworks, resolve all the assets from those frameworks and
    /// then prepare runtime initialization where the TPA contains only frameworks.
    /// Note that this case does **NOT** consume any `.deps.json` from the app/component (only processes the framework's `.deps.json`).
    ///
    /// Like all the other `initialize` functions, this function will
    /// * Process the `.runtimeconfig.json`
    /// * Resolve framework references and find actual frameworks
    /// * Find the root framework (`Microsoft.NETCore.App`) and load the hostpolicy from it
    /// * The hostpolicy will then process all relevant `.deps.json` files and produce the list of assemblies, native search paths and other artifacts needed to initialize the runtime.
    ///
    /// The functions will **NOT** load the CoreCLR runtime. They just prepare everything to the point where it can be loaded.
    ///
    /// # Arguments
    ///  * `runtime_config_path`:
    ///     Path to the `.runtimeconfig.json` file to process.
    ///     Unlike with [`initialize_for_dotnet_command_line`], any `.deps.json` from the app/component will not be processed by the hosting layers.
    ///
    /// [`initialize_for_dotnet_command_line`]: Hostfxr::initialize_for_dotnet_command_line
    pub fn initialize_for_runtime_config(
        &self,
        runtime_config_path: impl AsRef<PdCStr>,
    ) -> Result<HostfxrContext<InitializedForRuntimeConfig>, Error> {
        unsafe {
            self.initialize_for_runtime_config_with_parameters(runtime_config_path, ptr::null())
        }
    }

    /// This function loads the specified `.runtimeconfig.json`, resolve all frameworks, resolve all the assets from those frameworks and
    /// then prepare runtime initialization where the TPA contains only frameworks.
    /// Note that this case does **NOT** consume any `.deps.json` from the app/component (only processes the framework's `.deps.json`).
    ///
    /// Like all the other `initialize` functions, this function will
    /// * Process the `.runtimeconfig.json`
    /// * Resolve framework references and find actual frameworks
    /// * Find the root framework (`Microsoft.NETCore.App`) and load the hostpolicy from it
    /// * The hostpolicy will then process all relevant `.deps.json` files and produce the list of assemblies, native search paths and other artifacts needed to initialize the runtime.
    ///
    /// The functions will **NOT** load the CoreCLR runtime. They just prepare everything to the point where it can be loaded.
    ///
    /// # Arguments
    ///  * `runtime_config_path`:
    ///     Path to the `.runtimeconfig.json` file to process.
    ///     Unlike with [`initialize_for_dotnet_command_line`], any `.deps.json` from the app/component will not be processed by the hosting layers.
    ///  * `host_path`:
    ///     Path to the native host (typically the `.exe`).
    ///     This value is not used for anything by the hosting components.
    ///     It's just passed to the CoreCLR as the path to the executable.
    ///     It can point to a file which is not executable itself, if such file doesn't exist (for example in COM activation scenarios this points to the `comhost.dll`).
    ///     This is used by PAL to initialize internal command line structures, process name and so on.
    ///
    /// [`initialize_for_dotnet_command_line`]: Hostfxr::initialize_for_dotnet_command_line
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
    /// This function loads the specified `.runtimeconfig.json`, resolve all frameworks, resolve all the assets from those frameworks and
    /// then prepare runtime initialization where the TPA contains only frameworks.
    /// Note that this case does **NOT** consume any `.deps.json` from the app/component (only processes the framework's `.deps.json`).
    ///
    /// Like all the other `initialize` functions, this function will
    /// * Process the `.runtimeconfig.json`
    /// * Resolve framework references and find actual frameworks
    /// * Find the root framework (`Microsoft.NETCore.App`) and load the hostpolicy from it
    /// * The hostpolicy will then process all relevant `.deps.json` files and produce the list of assemblies, native search paths and other artifacts needed to initialize the runtime.
    ///
    /// The functions will **NOT** load the CoreCLR runtime. They just prepare everything to the point where it can be loaded.
    ///
    /// # Arguments
    ///  * `runtime_config_path`:
    ///     Path to the `.runtimeconfig.json` file to process.
    ///     Unlike with [`initialize_for_dotnet_command_line`], any `.deps.json` from the app/component will not be processed by the hosting layers.
    ///  * `dotnet_root`:
    ///     Path to the root of the .NET Core installation in use.
    ///     This typically points to the install location from which the hostfxr has been loaded.
    ///     For example on Windows this would typically point to `C:\Program Files\dotnet`.
    ///     The path is used to search for shared frameworks and potentially SDKs.
    ///
    /// [`initialize_for_dotnet_command_line`]: Hostfxr::initialize_for_dotnet_command_line
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

        let result = unsafe {
            self.lib.hostfxr_initialize_for_runtime_config(
                runtime_config_path.as_ref().as_ptr(),
                parameters,
                hostfxr_handle.as_mut_ptr(),
            )
        };

        HostExitCode::from(result).to_result()?;

        Ok(HostfxrContext::new(
            unsafe { HostfxrHandle::new_unchecked(hostfxr_handle.assume_init()) },
            self,
        ))
    }
}
