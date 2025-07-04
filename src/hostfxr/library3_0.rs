use crate::{
    bindings::hostfxr::{hostfxr_handle, hostfxr_initialize_parameters},
    error::{HostingError, HostingResult, HostingSuccess},
    hostfxr::{
        Hostfxr, HostfxrContext, HostfxrHandle, InitializedForCommandLine,
        InitializedForRuntimeConfig,
    },
    pdcstring::{PdCStr, PdChar},
};
use std::{iter, mem::MaybeUninit, ptr};

use super::UNSUPPORTED_HOST_VERSION_ERROR_CODE;

impl Hostfxr {
    /// Initializes the hosting components for a dotnet command line running an application
    ///
    /// Like all the other `initialize` functions, this function will
    /// * Process the `.runtimeconfig.json`
    /// * Resolve framework references and find actual frameworks
    /// * Find the root framework (`Microsoft.NETCore.App`) and load the hostpolicy from it
    /// * The hostpolicy will then process all relevant `.deps.json` files and produce the list of assemblies, native search paths and other artifacts needed to initialize the runtime.
    ///
    /// The functions will **NOT** load the `CoreCLR` runtime. They just prepare everything to the point where it can be loaded.
    ///
    /// # Arguments
    ///  * `app_path`:
    ///    The path to the target application.
    ///
    /// # Remarks
    /// This function parses the specified command-line arguments to determine the application to run. It will
    /// then find the corresponding `.runtimeconfig.json` and `.deps.json` with which to resolve frameworks and
    /// dependencies and prepare everything needed to load the runtime.
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "netcore3_0")))]
    pub fn initialize_for_dotnet_command_line(
        &self,
        app_path: impl AsRef<PdCStr>,
    ) -> Result<HostfxrContext<InitializedForCommandLine>, HostingError> {
        self.initialize_for_dotnet_command_line_with_args(app_path, iter::empty::<&PdCStr>())
    }

    /// Initializes the hosting components for a dotnet command line running an application
    ///
    /// Like all the other `initialize` functions, this function will
    /// * Process the `.runtimeconfig.json`
    /// * Resolve framework references and find actual frameworks
    /// * Find the root framework (`Microsoft.NETCore.App`) and load the hostpolicy from it
    /// * The hostpolicy will then process all relevant `.deps.json` files and produce the list of assemblies, native search paths and other artifacts needed to initialize the runtime.
    ///
    /// The functions will **NOT** load the `CoreCLR` runtime. They just prepare everything to the point where it can be loaded.
    ///
    /// # Arguments
    ///  * `app_path`:
    ///    The path to the target application.
    ///  * `host_path`:
    ///    Path to the native host (typically the `.exe`).
    ///    This value is not used for anything by the hosting components.
    ///    It's just passed to the `CoreCLR` as the path to the executable.
    ///    It can point to a file which is not executable itself, if such file doesn't exist (for example in COM activation scenarios this points to the `comhost.dll`).
    ///    This is used by PAL to initialize internal command line structures, process name and so on.
    ///
    /// # Remarks
    /// This function parses the specified command-line arguments to determine the application to run. It will
    /// then find the corresponding `.runtimeconfig.json` and `.deps.json` with which to resolve frameworks and
    /// dependencies and prepare everything needed to load the runtime.
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "netcore3_0")))]
    pub fn initialize_for_dotnet_command_line_with_host_path(
        &self,
        app_path: impl AsRef<PdCStr>,
        host_path: impl AsRef<PdCStr>,
    ) -> Result<HostfxrContext<InitializedForCommandLine>, HostingError> {
        self.initialize_for_dotnet_command_line_with_args_and_host_path(
            app_path,
            iter::empty::<&PdCStr>(),
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
    /// The functions will **NOT** load the `CoreCLR` runtime. They just prepare everything to the point where it can be loaded.
    ///
    /// # Arguments
    ///  * `app_path`:
    ///    The path to the target application.
    ///  * `dotnet_root`:
    ///    Path to the root of the .NET Core installation in use.
    ///    This typically points to the install location from which the hostfxr has been loaded.
    ///    For example on Windows this would typically point to `C:\Program Files\dotnet`.
    ///    The path is used to search for shared frameworks and potentially SDKs.
    ///
    /// # Remarks
    /// This function parses the specified command-line arguments to determine the application to run. It will
    /// then find the corresponding `.runtimeconfig.json` and `.deps.json` with which to resolve frameworks and
    /// dependencies and prepare everything needed to load the runtime.
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "netcore3_0")))]
    pub fn initialize_for_dotnet_command_line_with_dotnet_root(
        &self,
        app_path: impl AsRef<PdCStr>,
        dotnet_root: impl AsRef<PdCStr>,
    ) -> Result<HostfxrContext<InitializedForCommandLine>, HostingError> {
        self.initialize_for_dotnet_command_line_with_args_and_dotnet_root(
            app_path,
            iter::empty::<&PdCStr>(),
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
    /// The functions will **NOT** load the `CoreCLR` runtime. They just prepare everything to the point where it can be loaded.
    ///
    /// # Arguments
    ///  * `app_path`:
    ///    The path to the target application.
    ///  * `args`:
    ///    The command line arguments for the managed application.
    ///
    /// # Remarks
    /// This function parses the specified command-line arguments to determine the application to run. It will
    /// then find the corresponding `.runtimeconfig.json` and `.deps.json` with which to resolve frameworks and
    /// dependencies and prepare everything needed to load the runtime.
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "netcore3_0")))]
    pub fn initialize_for_dotnet_command_line_with_args(
        &self,
        app_path: impl AsRef<PdCStr>,
        args: impl Iterator<Item = impl AsRef<PdCStr>>,
    ) -> Result<HostfxrContext<InitializedForCommandLine>, HostingError> {
        unsafe {
            self.initialize_for_dotnet_command_line_with_parameters(app_path, args, ptr::null())
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
    /// The functions will **NOT** load the `CoreCLR` runtime. They just prepare everything to the point where it can be loaded.
    ///
    /// # Arguments
    ///  * `app_path`:
    ///    The path to the target application.
    ///  * `args`:
    ///    The command line arguments for the managed application.
    ///  * `host_path`:
    ///    Path to the native host (typically the `.exe`).
    ///    This value is not used for anything by the hosting components.
    ///    It's just passed to the `CoreCLR` as the path to the executable.
    ///    It can point to a file which is not executable itself, if such file doesn't exist (for example in COM activation scenarios this points to the `comhost.dll`).
    ///    This is used by PAL to initialize internal command line structures, process name and so on.
    ///
    /// # Remarks
    /// This function parses the specified command-line arguments to determine the application to run. It will
    /// then find the corresponding `.runtimeconfig.json` and `.deps.json` with which to resolve frameworks and
    /// dependencies and prepare everything needed to load the runtime.
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "netcore3_0")))]
    pub fn initialize_for_dotnet_command_line_with_args_and_host_path(
        &self,
        app_path: impl AsRef<PdCStr>,
        args: impl Iterator<Item = impl AsRef<PdCStr>>,
        host_path: impl AsRef<PdCStr>,
    ) -> Result<HostfxrContext<InitializedForCommandLine>, HostingError> {
        let parameters = hostfxr_initialize_parameters::with_host_path(host_path.as_ref().as_ptr());
        unsafe {
            self.initialize_for_dotnet_command_line_with_parameters(
                app_path,
                args,
                &raw const parameters,
            )
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
    /// The functions will **NOT** load the `CoreCLR` runtime. They just prepare everything to the point where it can be loaded.
    ///
    /// # Arguments
    ///  * `app_path`:
    ///    The path to the target application.
    ///  * `args`:
    ///    The command line arguments for the managed application.
    ///  * `dotnet_root`:
    ///    Path to the root of the .NET Core installation in use.
    ///    This typically points to the install location from which the hostfxr has been loaded.
    ///    For example on Windows this would typically point to `C:\Program Files\dotnet`.
    ///    The path is used to search for shared frameworks and potentially SDKs.
    ///
    /// # Remarks
    /// This function parses the specified command-line arguments to determine the application to run. It will
    /// then find the corresponding `.runtimeconfig.json` and `.deps.json` with which to resolve frameworks and
    /// dependencies and prepare everything needed to load the runtime.
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "netcore3_0")))]
    pub fn initialize_for_dotnet_command_line_with_args_and_dotnet_root(
        &self,
        app_path: impl AsRef<PdCStr>,
        args: impl Iterator<Item = impl AsRef<PdCStr>>,
        dotnet_root: impl AsRef<PdCStr>,
    ) -> Result<HostfxrContext<InitializedForCommandLine>, HostingError> {
        let parameters =
            hostfxr_initialize_parameters::with_dotnet_root(dotnet_root.as_ref().as_ptr());
        unsafe {
            self.initialize_for_dotnet_command_line_with_parameters(
                app_path,
                args,
                &raw const parameters,
            )
        }
    }

    unsafe fn initialize_for_dotnet_command_line_with_parameters(
        &self,
        app_path: impl AsRef<PdCStr>,
        args: impl Iterator<Item = impl AsRef<PdCStr>>,
        parameters: *const hostfxr_initialize_parameters,
    ) -> Result<HostfxrContext<InitializedForCommandLine>, HostingError> {
        let mut hostfxr_handle = MaybeUninit::<hostfxr_handle>::uninit();

        let app_path = app_path.as_ref().as_ptr();
        let args = args.map(|arg| arg.as_ref().as_ptr());
        let app_path_and_args = iter::once(app_path).chain(args).collect::<Vec<_>>();
        let result = unsafe {
            self.lib.hostfxr_initialize_for_dotnet_command_line(
                app_path_and_args.len().try_into().unwrap(),
                app_path_and_args.as_ptr(),
                parameters,
                hostfxr_handle.as_mut_ptr(),
            )
        }
        .unwrap_or(UNSUPPORTED_HOST_VERSION_ERROR_CODE);

        let success_code = HostingResult::from(result).into_result()?;

        let is_primary = matches!(success_code, HostingSuccess::Success);

        Ok(unsafe {
            HostfxrContext::from_handle(
                HostfxrHandle::new_unchecked(hostfxr_handle.assume_init()),
                self.clone(),
                is_primary,
            )
        })
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
    /// The functions will **NOT** load the `CoreCLR` runtime. They just prepare everything to the point where it can be loaded.
    ///
    /// # Arguments
    ///  * `runtime_config_path`:
    ///    Path to the `.runtimeconfig.json` file to process.
    ///    Unlike with [`initialize_for_dotnet_command_line`], any `.deps.json` from the app/component will not be processed by the hosting layers.
    ///
    /// [`initialize_for_dotnet_command_line`]: Hostfxr::initialize_for_dotnet_command_line
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "netcore3_0")))]
    pub fn initialize_for_runtime_config(
        &self,
        runtime_config_path: impl AsRef<PdCStr>,
    ) -> Result<HostfxrContext<InitializedForRuntimeConfig>, HostingError> {
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
    /// The functions will **NOT** load the `CoreCLR` runtime. They just prepare everything to the point where it can be loaded.
    ///
    /// # Arguments
    ///  * `runtime_config_path`:
    ///    Path to the `.runtimeconfig.json` file to process.
    ///    Unlike with [`initialize_for_dotnet_command_line`], any `.deps.json` from the app/component will not be processed by the hosting layers.
    ///  * `host_path`:
    ///    Path to the native host (typically the `.exe`).
    ///    This value is not used for anything by the hosting components.
    ///    It's just passed to the `CoreCLR` as the path to the executable.
    ///    It can point to a file which is not executable itself, if such file doesn't exist (for example in COM activation scenarios this points to the `comhost.dll`).
    ///    This is used by PAL to initialize internal command line structures, process name and so on.
    ///
    /// [`initialize_for_dotnet_command_line`]: Hostfxr::initialize_for_dotnet_command_line
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "netcore3_0")))]
    pub fn initialize_for_runtime_config_with_host_path(
        &self,
        runtime_config_path: impl AsRef<PdCStr>,
        host_path: impl AsRef<PdCStr>,
    ) -> Result<HostfxrContext<InitializedForRuntimeConfig>, HostingError> {
        let parameters = hostfxr_initialize_parameters::with_host_path(host_path.as_ref().as_ptr());
        unsafe {
            self.initialize_for_runtime_config_with_parameters(
                runtime_config_path,
                &raw const parameters,
            )
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
    /// The functions will **NOT** load the `CoreCLR` runtime. They just prepare everything to the point where it can be loaded.
    ///
    /// # Arguments
    ///  * `runtime_config_path`:
    ///    Path to the `.runtimeconfig.json` file to process.
    ///    Unlike with [`initialize_for_dotnet_command_line`], any `.deps.json` from the app/component will not be processed by the hosting layers.
    ///  * `dotnet_root`:
    ///    Path to the root of the .NET Core installation in use.
    ///    This typically points to the install location from which the hostfxr has been loaded.
    ///    For example on Windows this would typically point to `C:\Program Files\dotnet`.
    ///    The path is used to search for shared frameworks and potentially SDKs.
    ///
    /// [`initialize_for_dotnet_command_line`]: Hostfxr::initialize_for_dotnet_command_line
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "netcore3_0")))]
    pub fn initialize_for_runtime_config_with_dotnet_root(
        &self,
        runtime_config_path: impl AsRef<PdCStr>,
        dotnet_root: impl AsRef<PdCStr>,
    ) -> Result<HostfxrContext<InitializedForRuntimeConfig>, HostingError> {
        let parameters =
            hostfxr_initialize_parameters::with_dotnet_root(dotnet_root.as_ref().as_ptr());
        unsafe {
            self.initialize_for_runtime_config_with_parameters(
                runtime_config_path,
                &raw const parameters,
            )
        }
    }

    unsafe fn initialize_for_runtime_config_with_parameters(
        &self,
        runtime_config_path: impl AsRef<PdCStr>,
        parameters: *const hostfxr_initialize_parameters,
    ) -> Result<HostfxrContext<InitializedForRuntimeConfig>, HostingError> {
        let mut hostfxr_handle = MaybeUninit::uninit();

        let result = unsafe {
            self.lib.hostfxr_initialize_for_runtime_config(
                runtime_config_path.as_ref().as_ptr(),
                parameters,
                hostfxr_handle.as_mut_ptr(),
            )
        }
        .unwrap_or(UNSUPPORTED_HOST_VERSION_ERROR_CODE);

        let success_code = HostingResult::from(result).into_result()?;

        let is_primary = matches!(success_code, HostingSuccess::Success);

        Ok(unsafe {
            HostfxrContext::from_handle(
                HostfxrHandle::new_unchecked(hostfxr_handle.assume_init()),
                self.clone(),
                is_primary,
            )
        })
    }

    /// Sets a callback which is to be used to write errors to.
    ///
    /// # Arguments
    ///  * `error_writer`
    ///    A callback function which will be invoked every time an error is to be reported.
    ///    Or [`None`] to unregister previously registered callbacks and return to the default behavior.
    ///
    /// # Remarks
    /// The error writer is registered per-thread, so the registration is thread-local. On each thread
    /// only one callback can be registered. Subsequent registrations overwrite the previous ones.
    ///
    /// By default no callback is registered in which case the errors are written to stderr.
    ///
    /// Each call to the error writer is sort of like writing a single line (the EOL character is omitted).
    /// Multiple calls to the error writer may occure for one failure.
    ///
    /// If the hostfxr invokes functions in hostpolicy as part of its operation, the error writer
    /// will be propagated to hostpolicy for the duration of the call. This means that errors from
    /// both hostfxr and hostpolicy will be reporter through the same error writer.
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "netcore3_0")))]
    pub fn set_error_writer(&self, error_writer: Option<ErrorWriter>) {
        let new_raw_error_writer = error_writer
            .as_ref()
            .map(|_| error_writer_trampoline as hostfxr_sys::hostfxr_error_writer_fn);
        unsafe { self.lib.hostfxr_set_error_writer(new_raw_error_writer) };

        CURRENT_ERROR_WRITER.with(|current_writer| {
            *current_writer.borrow_mut() = error_writer;
        });
    }
}

type ErrorWriter = Box<dyn FnMut(&PdCStr)>;

thread_local! {
    static CURRENT_ERROR_WRITER: std::cell::RefCell<Option<ErrorWriter>> = std::cell::RefCell::new(None);
}

extern "C" fn error_writer_trampoline(raw_error: *const PdChar) {
    CURRENT_ERROR_WRITER.with(|writer_holder| {
        if let Some(writer) = writer_holder.borrow_mut().as_mut() {
            let error_message = unsafe { PdCStr::from_str_ptr(raw_error) };
            writer(error_message);
        }
    });
}
