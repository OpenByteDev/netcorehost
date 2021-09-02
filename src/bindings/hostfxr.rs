use std::{ffi::c_void, mem, ptr};

use dlopen::wrapper::WrapperApi;

use super::type_aliases::{char_t, size_t};

#[repr(i32)]
pub enum hostfxr_delegate_type {
    hdt_com_activation = 0,
    hdt_load_in_memory_assembly = 1,
    hdt_winrt_activation = 2,
    hdt_com_register = 3,
    hdt_com_unregister = 4,
    hdt_load_assembly_and_get_function_pointer = 5,
    hdt_get_function_pointer = 6,
}

pub type hostfxr_error_writer_fn = unsafe extern "C" fn(message: *const char_t);

pub type hostfxr_handle = *const c_void;

/// A structure that stores parameters which are common to all forms of initialization.
#[repr(C)]
pub struct hostfxr_initialize_parameters {
    /// The size of the structure.
    /// This is used for versioning.
    /// Should be set to `mem::size_of::<hostfxr_initialize_parameters>()`.
    pub size: size_t,
    /// Path to the native host (typically the `.exe`).
    /// This value is not used for anything by the hosting components.
    /// It's just passed to the CoreCLR as the path to the executable.
    /// It can point to a file which is not executable itself, if such file doesn't exist
    /// (for example in COM activation scenarios this points to the `comhost.dll`).
    /// This is used by PAL (Platform Abstraction Layer) to initialize internal command line structures, process name and so on.
    pub host_path: *const char_t,
    /// Path to the root of the .NET Core installation in use.
    /// This typically points to the install location from which the hostfxr has been loaded.
    /// For example on Windows this would typically point to `C:\Program Files\dotnet`.
    /// The path is used to search for shared frameworks and potentially SDKs.
    pub dotnet_root: *const char_t,
}

impl hostfxr_initialize_parameters {
    /// Creates a new instance of [`hostfxr_initialize_parameters`] with the given `host_path`.
    /// The `size` field is set accordingly to the size of the struct and `dotnet_root` to [`ptr::null()`].
    pub fn with_host_path(host_path: *const char_t) -> hostfxr_initialize_parameters {
        hostfxr_initialize_parameters {
            size: mem::size_of::<hostfxr_initialize_parameters>(),
            host_path,
            dotnet_root: ptr::null(),
        }
    }
    /// Creates a new instance of [`hostfxr_initialize_parameters`] with the given `dotnet_root`.
    /// The `size` field is set accordingly to the size of the struct and `host_path` to [`ptr::null()`].
    pub fn with_dotnet_root(dotnet_root: *const char_t) -> hostfxr_initialize_parameters {
        hostfxr_initialize_parameters {
            size: mem::size_of::<hostfxr_initialize_parameters>(),
            host_path: ptr::null(),
            dotnet_root,
        }
    }
}

#[derive(WrapperApi)]
pub struct HostfxrLib {
    /// Run an application.
    ///
    /// # Arguments
    ///  * `argv` - command-line arguments
    ///
    /// This function does not return until the application completes execution.
    /// It will shutdown CoreCLR after the application executes.
    /// If the application is successfully executed, this value will return the exit code of the application. Otherwise, it will return an error code indicating the failure.
    hostfxr_main: unsafe extern "C" fn(argc: i32, argv: *const *const char_t) -> i32,

    /// Run an application.
    ///
    /// # Arguments
    ///  * `argv`
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
    /// If the application is successfully executed, this value will return the exit code of the application. Otherwise, it will return an error code indicating the failure.
    hostfxr_main_startupinfo: unsafe extern "C" fn(
        argc: i32,
        argv: *const *const char_t,
        host_path: *const char_t,
        dotnet_root: *const char_t,
        app_path: *const char_t,
    ) -> i32,

    hostfxr_main_bundle_startupinfo: unsafe extern "C" fn(
        argc: i32,
        argv: *const *const char_t,
        host_path: *const char_t,
        dotnet_root: *const char_t,
        app_path: *const char_t,
        bundle_header_offset: i64,
    ) -> i32,

    /// Sets a callback which is to be used to write errors to.
    ///
    /// # Arguments
    ///  * `error_writer`:
    ///     A callback function which will be invoked every time an error is to be reported.
    ///     Or [`null`](ptr::null()) to unregister previously registered callback and return to the default behavior.
    ///
    /// # Return value
    /// The previously registered callback (which is now unregistered), or [`null`](ptr::null()) if no previous callback
    /// was registered
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
    hostfxr_set_error_writer:
        unsafe extern "C" fn(error_writer: hostfxr_error_writer_fn) -> hostfxr_error_writer_fn,

    /// Initializes the hosting components for a dotnet command line running an application
    ///
    /// # Arguments
    ///  * `argc`:
    ///     Number of argv arguments
    ///  * `argv`:
    ///     Command-line arguments for running an application (as if through the dotnet executable).
    ///  * `parameters`:
    ///     Optional. Additional parameters for initialization
    ///  * `host_context_handle`:
    ///     On success, this will be populated with an opaque value representing the initialized host context
    ///
    /// # Return value
    ///  * [`Success`]:
    ///     Hosting components were successfully initialized
    ///  * [`HostInvalidState`]:
    ///     Hosting components are already initialized
    ///
    /// # Remarks
    /// This function parses the specified command-line arguments to determine the application to run. It will
    /// then find the corresponding `.runtimeconfig.json` and `.deps.json` with which to resolve frameworks and
    /// dependencies and prepare everything needed to load the runtime.
    ///
    /// This function only supports arguments for running an application. It does not support SDK commands.
    ///
    /// This function does not load the runtime.
    ///
    /// [`Success`]: crate::bindings::StatusCode::Success
    /// [`HostInvalidState`]: crate::bindings::StatusCode::HostInvalidState
    hostfxr_initialize_for_dotnet_command_line: unsafe extern "C" fn(
        argc: i32,
        argv: *const *const char_t,
        parameters: *const hostfxr_initialize_parameters,
        /*out*/ host_context_handle: *mut hostfxr_handle,
    ) -> i32,

    /// Initializes the hosting components using a `.runtimeconfig.json` file
    ///
    /// # Arguments
    ///  * `runtime_config_path`:
    ///     Path to the `.runtimeconfig.json` file
    ///  * `parameters`:
    ///     Optional. Additional parameters for initialization
    ///  * `host_context_handle`:
    ///     On success, this will be populated with an opaque value representing the initialized host context
    ///
    /// # Return value
    /// * [`Success`]:
    ///      Hosting components were successfully initialized
    /// * [`Success_HostAlreadyInitialized`]:
    ///      Config is compatible with already initialized hosting components
    /// * [`Success_DifferentRuntimeProperties`]:
    ///      Config has runtime properties that differ from already initialized hosting components
    /// * [`CoreHostIncompatibleConfig`]:
    ///      Config is incompatible with already initialized hosting components
    ///
    /// # Remarks
    /// This function will process the `.runtimeconfig.json` to resolve frameworks and prepare everything needed
    /// to load the runtime. It will only process the `.deps.json` from frameworks (not any app/component that
    /// may be next to the `.runtimeconfig.json`).
    ///
    /// This function does not load the runtime.
    ///
    /// If called when the runtime has already been loaded, this function will check if the specified runtime
    /// config is compatible with the existing runtime.
    ///
    /// Both [`Success_HostAlreadyInitialized`] and [`Success_DifferentRuntimeProperties`] codes are considered successful
    /// initializations. In the case of [`Success_DifferentRuntimeProperties`], it is left to the consumer to verify that
    /// the difference in properties is acceptable.
    ///
    /// [`Success`]: crate::bindings::StatusCode::Success
    /// [`Success_HostAlreadyInitialized`]: crate::bindings::StatusCode::Success_HostAlreadyInitialized
    /// [`Success_DifferentRuntimeProperties`]: crate::bindings::StatusCode::Success_DifferentRuntimeProperties
    /// [`CoreHostIncompatibleConfig`]: crate::bindings::StatusCode::CoreHostIncompatibleConfig
    hostfxr_initialize_for_runtime_config: unsafe extern "C" fn(
        runtime_config_path: *const char_t,
        parameters: *const hostfxr_initialize_parameters,
        /*out*/ host_context_handle: *mut hostfxr_handle,
    ) -> i32,

    /// Gets the runtime property value for an initialized host context
    ///
    /// # Arguments
    ///  * `host_context_handle`:
    ///     Handle to the initialized host context
    ///  * `name`:
    ///     Runtime property name
    ///  * `value`:
    ///     Out parameter. Pointer to a buffer with the property value.
    ///
    /// # Return value
    /// The error code result.
    ///
    /// # Remarks
    /// The buffer pointed to by value is owned by the host context. The lifetime of the buffer is only
    /// guaranteed until any of the below occur:
    ///  * a 'run' method is called for the host context
    ///  * properties are changed via [`hostfxr_set_runtime_property_value`]
    ///  * the host context is closed via [`hostfxr_close`]
    ///
    /// If `host_context_handle` is [`null`](ptr::null()) and an active host context exists, this function will get the
    /// property value for the active host context.
    ///
    /// [`hostfxr_set_runtime_property_value`]: struct.HostfxrLib.html#method.hostfxr_set_runtime_property_value
    /// [`hostfxr_close`]: struct.HostfxrLib.html#method.hostfxr_close
    hostfxr_get_runtime_property_value: unsafe extern "C" fn(
        host_context_handle: hostfxr_handle,
        name: *const char_t,
        /*out*/ value: *mut *const char_t,
    ) -> i32,

    /// Sets the value of a runtime property for an initialized host context
    ///
    /// # Arguments
    ///  * `host_context_handle`:
    ///     Handle to the initialized host context
    ///  * `name`:
    ///     Runtime property name
    ///  * `value`:
    ///     Value to set
    ///
    /// # Return value
    /// The error code result.
    ///
    /// # Remarks
    /// Setting properties is only supported for the first host context, before the runtime has been loaded.
    ///
    /// If the property already exists in the host context, it will be overwritten. If value is [`null`](ptr::null()), the
    /// property will be removed.
    hostfxr_set_runtime_property_value: unsafe extern "C" fn(
        host_context_handle: hostfxr_handle,
        name: *const char_t,
        value: *const char_t,
    ) -> i32,

    /// Gets all the runtime properties for an initialized host context
    ///
    /// # Arguments
    ///  * `host_context_handle`:
    ///     Handle to the initialized host context
    ///  * `count`:
    ///     \[in\] Size of the keys and values buffers
    ///     \[out\] Number of properties returned (size of keys/values buffers used). If the input value is too
    ///             small or keys/values is [`null`](ptr::null()), this is populated with the number of available properties
    ///  * `keys`:
    ///     \[out\] Array of pointers to buffers with runtime property keys
    ///  * `values`:
    ///     \[out\] Array of pointers to buffers with runtime property values
    ///
    /// # Return value
    /// The error code result.
    ///
    /// # Remarks
    /// The buffers pointed to by keys and values are owned by the host context. The lifetime of the buffers is only
    /// guaranteed until any of the below occur:
    ///  * a 'run' method is called for the host context
    ///  * properties are changed via [`hostfxr_set_runtime_property_value`]
    ///  * the host context is closed via [`hostfxr_close`]
    ///
    /// If host_context_handle is [`null`](ptr::null()) and an active host context exists, this function will get the
    /// properties for the active host context.
    ///
    /// [`hostfxr_set_runtime_property_value`]: struct.HostfxrLib.html#hostfxr_set_runtime_property_value
    /// [`hostfxr_close`]: struct.HostfxrLib.html#method.hostfxr_closee
    hostfxr_get_runtime_properties: unsafe extern "C" fn(
        host_context_handle: hostfxr_handle,
        /*inout*/ count: *mut size_t,
        /*out*/ keys: *mut *const char_t,
        /*out*/ values: *mut *const char_t,
    ) -> i32,

    /// Load CoreCLR and run the application for an initialized host context
    ///
    /// # Arguments
    ///  * `host_context_handle`:
    ///     Handle to the initialized host context
    ///
    /// # Return value
    /// If the app was successfully run, the exit code of the application. Otherwise, the error code result.
    ///
    /// # Remarks
    /// The `host_context_handle` must have been initialized using [`hostfxr_initialize_for_dotnet_command_line`].
    ///
    /// This function will not return until the managed application exits.
    ///
    /// [`hostfxr_initialize_for_runtime_config`]: struct.HostfxrLib.html#method.hostfxr_initialize_for_runtime_config
    /// [`hostfxr_initialize_for_dotnet_command_line`]: struct.HostfxrLib.html#method.hostfxr_initialize_for_dotnet_command_line
    hostfxr_run_app: unsafe extern "C" fn(host_context_handle: hostfxr_handle) -> i32,

    /// Gets a typed delegate from the currently loaded CoreCLR or from a newly created one.
    ///
    /// # Arguments
    ///  * `host_context_handle`:
    ///     Handle to the initialized host context
    ///  * `type`:
    ///     Type of runtime delegate requested
    ///  * `delegate`:
    ///     An out parameter that will be assigned the delegate.
    ///
    /// # Return value
    /// The error code result.
    ///
    /// # Remarks
    /// If the `host_context_handle` was initialized using [`hostfxr_initialize_for_runtime_config`],
    /// then all delegate types are supported.
    /// If the host_context_handle was initialized using [`hostfxr_initialize_for_dotnet_command_line`],
    /// then only the following delegate types are currently supported:
    ///  * [`hdt_load_assembly_and_get_function_pointer`]
    ///  * [`hdt_get_function_pointer`]
    ///
    /// [`hdt_load_assembly_and_get_function_pointer`]: hostfxr_delegate_type::hdt_load_assembly_and_get_function_pointer
    /// [`hdt_get_function_pointer`]: hostfxr_delegate_type::hdt_get_function_pointer
    /// [`hostfxr_initialize_for_runtime_config`]: struct.HostfxrLib.html#method.hostfxr_initialize_for_runtime_config
    /// [`hostfxr_initialize_for_dotnet_command_line`]: struct.HostfxrLib.html#method.hostfxr_initialize_for_dotnet_command_line
    hostfxr_get_runtime_delegate: unsafe extern "C" fn(
        host_context_handle: hostfxr_handle,
        r#type: hostfxr_delegate_type,
        /*out*/ delegate: *mut *const (),
    ) -> i32,

    /// Closes an initialized host context
    ///
    /// # Arguments
    ///  * `host_context_handle`:
    ///     Handle to the initialized host context
    ///
    /// # Return value:
    /// The error code result.
    hostfxr_close: unsafe extern "C" fn(host_context_handle: hostfxr_handle) -> i32,
}

/// Signature of delegate returned by [`hostfxr_get_runtime_delegate`] for type [`hdt_load_assembly_and_get_function_pointer`]
///
/// # Arguments
///  * `assembly_path`:
///    Fully qualified path to assembly
///  * `type_name`:
///     Assembly qualified type name
///  * `method_name`:
///     Public static method name compatible with delegateType
///  * `delegate_type_name`:
///     Assembly qualified delegate type name or [`null`](ptr::null()), or [`UNMANAGED_CALLERS_ONLY_METHOD`] if the method is marked with the [`UnmanagedCallersOnlyAttribute`].
///  * `load_context`:
///     Extensibility parameter (currently unused and must be 0)
///  * `reserved`:
///     Extensibility parameter (currently unused and must be 0)
///  * `delegate`:
///     Pointer where to store the function pointer result
///
/// [`hostfxr_get_runtime_delegate`]: struct.HostfxrLib.html#method.hostfxr_get_runtime_delegate
/// [`hdt_load_assembly_and_get_function_pointer`]: hostfxr_delegate_type::`hdt_load_assembly_and_get_function_pointer
/// [`UNMANAGED_CALLERS_ONLY_METHOD`]: crate::bindings::consts::UNMANAGED_CALLERS_ONLY_METHOD
/// [`UnmanagedCallersOnlyAttribute`]: https://docs.microsoft.com/en-us/dotnet/api/system.runtime.interopservices.unmanagedcallersonlyattribute
pub type load_assembly_and_get_function_pointer_fn = unsafe extern "system" fn(
    assembly_path: *const char_t,
    type_name: *const char_t,
    method_name: *const char_t,
    delegate_type_name: *const char_t,
    reserved: *const c_void,
    /*out*/ delegate: *mut *const c_void,
) -> i32;

/// Signature of delegate returned by [`hostfxr_get_runtime_delegate`] for type [`hdt_get_function_pointer`]
///
/// # Arguments
///  * `type_name`:
///     Assembly qualified type name
///  * `method_name`:
///     Public static method name compatible with delegateType
///  * `delegate_type_name`:
///     Assembly qualified delegate type name or [`null`](ptr::null()), or [`UNMANAGED_CALLERS_ONLY_METHOD`] if the method is marked with the [`UnmanagedCallersOnlyAttribute`].
///  * `load_context`:
///     Extensibility parameter (currently unused and must be 0)
///  * `reserved`:
///     Extensibility parameter (currently unused and must be 0)
///  * `delegate`:
///     Pointer where to store the function pointer result
///
/// [`hdt_get_function_pointer`]: hostfxr_delegate_type::hdt_get_function_pointer
/// [`hostfxr_get_runtime_delegate`]: struct.HostfxrLib.html#method.hostfxr_get_runtime_delegate
/// [`UNMANAGED_CALLERS_ONLY_METHOD`]: crate::bindings::consts::UNMANAGED_CALLERS_ONLY_METHOD
/// [`UnmanagedCallersOnlyAttribute`]: https://docs.microsoft.com/en-us/dotnet/api/system.runtime.interopservices.unmanagedcallersonlyattribute
pub type get_function_pointer_fn = unsafe extern "system" fn(
    type_name: *const char_t,
    method_name: *const char_t,
    delegate_type_name: *const char_t,
    load_context: *const c_void,
    reserved: *const c_void,
    /*out*/ delegate: *mut *const c_void,
) -> i32;

/// Signature of delegate returned by [`load_assembly_and_get_function_pointer_fn`] when `delegate_type_name == null` (default)
pub type component_entry_point_fn = unsafe extern "system" fn(*const c_void, size_t) -> i32;
