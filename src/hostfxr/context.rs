use crate::{
    bindings::hostfxr::{
        get_function_pointer_fn, hostfxr_delegate_type, hostfxr_handle,
        load_assembly_and_get_function_pointer_fn, wrapper,
    },
    dlopen::wrapper::Container,
    error::{HostingError, HostingResult, HostingSuccess},
    hostfxr::{
        AppOrHostingResult, AssemblyDelegateLoader, DelegateLoader, Hostfxr, RawFunctionPtr,
    },
    pdcstring::{PdCStr, PdCString},
};

use std::{
    collections::HashMap,
    ffi::c_void,
    marker::PhantomData,
    mem::{self, ManuallyDrop, MaybeUninit},
    ptr::{self, NonNull},
    rc::Rc,
};

use destruct_drop::DestructDrop;

/// A marker struct indicating that the context was initialized with a runtime config.
/// This means that it is not possible to run the application associated with the context.
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "netcore3_0")))]
pub struct InitializedForRuntimeConfig;

/// A marker struct indicating that the context was initialized for the dotnet command line.
/// This means that it is possible to run the application associated with the context.
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "netcore3_0")))]
pub struct InitializedForCommandLine;

/// Handle of a loaded [`HostfxrContext`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "netcore3_0")))]
pub struct HostfxrHandle(NonNull<c_void>);

impl HostfxrHandle {
    /// Creates a new hostfxr handle from the given raw handle.
    ///
    /// # Safety
    /// - The given raw handle has to be non-null.
    /// - The given handle has to be valid and has to represent a hostfxr context.
    #[must_use]
    pub unsafe fn new_unchecked(ptr: hostfxr_handle) -> Self {
        Self(unsafe { NonNull::new_unchecked(ptr as *mut _) })
    }

    /// Returns the raw underlying handle.
    #[must_use]
    pub fn as_raw(&self) -> hostfxr_handle {
        self.0.as_ptr()
    }
}

impl From<HostfxrHandle> for hostfxr_handle {
    fn from(handle: HostfxrHandle) -> Self {
        handle.as_raw()
    }
}

pub(crate) type HostfxrLibrary = Container<wrapper::Hostfxr>;

/// State which hostfxr creates and maintains and represents a logical operation on the hosting components.
#[derive(DestructDrop)]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "netcore3_0")))]
pub struct HostfxrContext<I> {
    handle: HostfxrHandle,
    hostfxr: Rc<HostfxrLibrary>,
    context_type: PhantomData<I>,
    is_primary: bool,
}

impl<I> HostfxrContext<I> {
    /// Creates a new context from the given handle.
    ///
    /// # Safety
    /// The context handle  has to be match the context type `I`.
    /// If the context was initialized using [`initialize_for_dotnet_command_line`] `I` has to be [`InitializedForCommandLine`].
    /// If the context was initialized using [`initialize_for_runtime_config`] `I` has to be [`InitializedForRuntimeConfig`].
    ///
    /// [`initialize_for_dotnet_command_line`]: crate::hostfxr::Hostfxr::initialize_for_dotnet_command_line
    /// [`initialize_for_runtime_config`]: crate::hostfxr::Hostfxr::initialize_for_runtime_config
    #[must_use]
    pub unsafe fn from_handle(handle: HostfxrHandle, hostfxr: Hostfxr, is_primary: bool) -> Self {
        Self {
            handle,
            hostfxr: hostfxr.lib,
            is_primary,
            context_type: PhantomData,
        }
    }

    /// Gets the underlying handle to the hostfxr context.
    #[must_use]
    pub fn handle(&self) -> HostfxrHandle {
        self.handle
    }

    /// Gets the underlying handle to the hostfxr context and consume this context.
    #[must_use]
    pub fn into_handle(self) -> HostfxrHandle {
        let this = ManuallyDrop::new(self);
        this.handle
    }

    /// Gets whether the context is the primary hostfxr context.
    /// There can only be a single primary context in a process.
    ///
    /// # Note
    /// <https://github.com/dotnet/core-setup/blob/master/Documentation/design-docs/native-hosting.md#synchronization>
    #[must_use]
    pub fn is_primary(&self) -> bool {
        self.is_primary
    }

    /// Gets the runtime property value for the given key of this host context.
    pub fn get_runtime_property_value(
        &self,
        name: impl AsRef<PdCStr>,
    ) -> Result<PdCString, HostingError> {
        unsafe { self.get_runtime_property_value_ref(name) }.map(PdCStr::to_owned)
    }

    /// Gets the runtime property value for the given key of this host context.
    ///
    /// # Safety
    /// The value string is owned by the host context. The lifetime of the buffer is only
    /// guaranteed until any of the below occur:
    ///  * [`run_app`] is called for this host context
    ///  * properties are changed via [`set_runtime_property_value`] or [`remove_runtime_property_value`]
    ///
    /// [`run_app`]: HostfxrContext::run_app
    /// [`set_runtime_property_value`]: HostfxrContext::set_runtime_property_value
    /// [`remove_runtime_property_value`]: HostfxrContext::remove_runtime_property_value
    pub unsafe fn get_runtime_property_value_ref(
        &self,
        name: impl AsRef<PdCStr>,
    ) -> Result<&PdCStr, HostingError> {
        let mut value = MaybeUninit::uninit();

        let result = unsafe {
            self.hostfxr.hostfxr_get_runtime_property_value(
                self.handle.as_raw(),
                name.as_ref().as_ptr(),
                value.as_mut_ptr(),
            )
        };
        HostingResult::from(result).into_result()?;

        Ok(unsafe { PdCStr::from_str_ptr(value.assume_init()) })
    }

    /// Sets the value of a runtime property for this host context.
    pub fn set_runtime_property_value(
        &self,
        name: impl AsRef<PdCStr>,
        value: impl AsRef<PdCStr>,
    ) -> Result<HostingSuccess, HostingError> {
        let result = unsafe {
            self.hostfxr.hostfxr_set_runtime_property_value(
                self.handle.as_raw(),
                name.as_ref().as_ptr(),
                value.as_ref().as_ptr(),
            )
        };
        HostingResult::from(result).into_result()
    }

    /// Remove a runtime property for this host context.
    pub fn remove_runtime_property_value(
        &self,
        name: impl AsRef<PdCStr>,
    ) -> Result<HostingSuccess, HostingError> {
        let result = unsafe {
            self.hostfxr.hostfxr_set_runtime_property_value(
                self.handle.as_raw(),
                name.as_ref().as_ptr(),
                ptr::null(),
            )
        };
        HostingResult::from(result).into_result()
    }

    /// Get all runtime properties for this host context.
    ///
    /// # Safety
    /// The strings returned are owned by the host context. The lifetime of the buffers is only
    /// guaranteed until any of the below occur:
    ///  * [`run_app`] is called for this host context
    ///  * properties are changed via [`set_runtime_property_value`] or [`remove_runtime_property_value`]
    ///  * the host context is dropped
    ///
    /// [`run_app`]: HostfxrContext::run_app
    /// [`set_runtime_property_value`]: HostfxrContext::set_runtime_property_value
    /// [`remove_runtime_property_value`]: HostfxrContext::remove_runtime_property_value
    pub unsafe fn get_runtime_properties_ref(
        &self,
    ) -> Result<(Vec<&PdCStr>, Vec<&PdCStr>), HostingError> {
        // get count
        let mut count = MaybeUninit::uninit();
        let mut result = unsafe {
            self.hostfxr.hostfxr_get_runtime_properties(
                self.handle.as_raw(),
                count.as_mut_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };

        // ignore buffer too small error as the first call is only to get the required buffer size.
        match HostingResult::from(result).into_result() {
            Ok(_) | Err(HostingError::HostApiBufferTooSmall) => {}
            Err(e) => return Err(e),
        };

        // get values / fill buffer
        let mut count = unsafe { count.assume_init() };
        let mut keys = Vec::with_capacity(count);
        let mut values = Vec::with_capacity(count);
        result = unsafe {
            self.hostfxr.hostfxr_get_runtime_properties(
                self.handle.as_raw(),
                &mut count,
                keys.as_mut_ptr(),
                values.as_mut_ptr(),
            )
        };
        HostingResult::from(result).into_result()?;

        unsafe { keys.set_len(count) };
        unsafe { values.set_len(count) };

        let keys = keys
            .into_iter()
            .map(|e| unsafe { PdCStr::from_str_ptr(e) })
            .collect();
        let values = values
            .into_iter()
            .map(|e| unsafe { PdCStr::from_str_ptr(e) })
            .collect();

        Ok((keys, values))
    }

    /// Get all runtime properties for this host context as owned strings.
    pub fn get_runtime_properties_owned(
        &self,
    ) -> Result<(Vec<PdCString>, Vec<PdCString>), HostingError> {
        unsafe { self.get_runtime_properties_ref() }.map(|(keys, values)| {
            let owned_keys = keys.into_iter().map(PdCStr::to_owned).collect();
            let owned_values = values.into_iter().map(PdCStr::to_owned).collect();
            (owned_keys, owned_values)
        })
    }

    /// Get all runtime properties for this host context as an iterator over borrowed key-value pairs.
    ///
    /// # Safety
    /// The strings returned are owned by the host context. The lifetime of the buffers is only
    /// guaranteed until any of the below occur:
    ///  * [`run_app`] is called for this host context
    ///  * properties are changed via [`set_runtime_property_value`] or [`remove_runtime_property_value`]
    ///  * the host context is dropped
    ///
    /// [`run_app`]: HostfxrContext::run_app
    /// [`set_runtime_property_value`]: HostfxrContext::set_runtime_property_value
    /// [`remove_runtime_property_value`]: HostfxrContext::remove_runtime_property_value
    pub unsafe fn get_runtime_properties_ref_iter(
        &self,
    ) -> Result<impl Iterator<Item = (&PdCStr, &PdCStr)>, HostingError> {
        unsafe { self.get_runtime_properties_ref() }
            .map(|(keys, values)| keys.into_iter().zip(values.into_iter()))
    }

    /// Get all runtime properties for this host context as an iterator over owned key-value pairs.
    pub fn get_runtime_properties_iter(
        &self,
    ) -> Result<impl Iterator<Item = (PdCString, PdCString)>, HostingError> {
        self.get_runtime_properties_owned()
            .map(|(keys, values)| keys.into_iter().zip(values.into_iter()))
    }

    /// Get all runtime properties for this host context as an hashmap of borrowed strings.
    ///
    /// # Safety
    /// The strings returned are owned by the host context. The lifetime of the buffers is only
    /// guaranteed until any of the below occur:
    ///  * [`run_app`] is called for this host context
    ///  * properties are changed via [`set_runtime_property_value`] or [`remove_runtime_property_value`]
    ///  * the host context is dropped
    ///
    /// [`run_app`]: HostfxrContext::run_app
    /// [`set_runtime_property_value`]: HostfxrContext::set_runtime_property_value
    /// [`remove_runtime_property_value`]: HostfxrContext::remove_runtime_property_value
    pub unsafe fn get_runtime_properties_ref_as_map(
        &self,
    ) -> Result<HashMap<&PdCStr, &PdCStr>, HostingError> {
        unsafe { self.get_runtime_properties_ref() }
            .map(|(keys, values)| keys.into_iter().zip(values.into_iter()).collect())
    }

    /// Get all runtime properties for this host context as an hashmap of owned strings.
    pub fn get_runtime_properties_as_map(
        &self,
    ) -> Result<HashMap<PdCString, PdCString>, HostingError> {
        self.get_runtime_properties_iter().map(Iterator::collect)
    }

    /// Gets a typed delegate from the currently loaded `CoreCLR` or from a newly created one.
    /// You propably want to use [`get_delegate_loader`] or [`get_delegate_loader_for_assembly`]
    /// instead of this function if you want to load function pointers.
    ///
    /// # Remarks
    /// If the context was initialized using [`initialize_for_runtime_config`], then all delegate types are supported.
    /// If it was initialized using [`initialize_for_dotnet_command_line`], then only the following
    /// delegate types are currently supported:
    ///  * [`hdt_load_assembly_and_get_function_pointer`]
    ///  * [`hdt_get_function_pointer`]
    ///
    /// [`get_delegate_loader`]: HostfxrContext::get_delegate_loader
    /// [`get_delegate_loader_for_assembly`]: HostfxrContext::get_delegate_loader_for_assembly
    /// [`hdt_load_assembly_and_get_function_pointer`]: hostfxr_delegate_type::hdt_load_assembly_and_get_function_pointer
    /// [`hdt_get_function_pointer`]: hostfxr_delegate_type::hdt_get_function_pointer
    /// [`initialize_for_runtime_config`]: Hostfxr::initialize_for_runtime_config
    /// [`initialize_for_dotnet_command_line`]: Hostfxr::initialize_for_dotnet_command_line
    pub fn get_runtime_delegate(
        &self,
        r#type: hostfxr_delegate_type,
    ) -> Result<RawFunctionPtr, HostingError> {
        let mut delegate = MaybeUninit::uninit();
        let result = unsafe {
            self.hostfxr.hostfxr_get_runtime_delegate(
                self.handle.as_raw(),
                r#type,
                delegate.as_mut_ptr(),
            )
        };

        HostingResult::from(result).into_result()?;

        Ok(unsafe { delegate.assume_init() }.cast())
    }
    fn get_load_assembly_and_get_function_pointer_delegate(
        &self,
    ) -> Result<load_assembly_and_get_function_pointer_fn, HostingError> {
        unsafe {
            self.get_runtime_delegate(
                hostfxr_delegate_type::hdt_load_assembly_and_get_function_pointer,
            )
            .map(|ptr| mem::transmute(ptr))
        }
    }
    fn get_get_function_pointer_delegate(&self) -> Result<get_function_pointer_fn, HostingError> {
        unsafe {
            self.get_runtime_delegate(hostfxr_delegate_type::hdt_get_function_pointer)
                .map(|ptr| mem::transmute(ptr))
        }
    }

    /// Gets a delegate loader for loading an assembly and contained function pointers.
    pub fn get_delegate_loader(&self) -> Result<DelegateLoader, HostingError> {
        Ok(DelegateLoader {
            get_load_assembly_and_get_function_pointer: self
                .get_load_assembly_and_get_function_pointer_delegate()?,
            get_function_pointer: self.get_get_function_pointer_delegate()?,
            hostfxr: self.hostfxr.clone(),
        })
    }

    /// Gets a delegate loader for loading function pointers of the assembly with the given path.
    /// The assembly will be loaded lazily when the first function pointer is loaded.
    pub fn get_delegate_loader_for_assembly<A: AsRef<PdCStr>>(
        &self,
        assembly_path: A,
    ) -> Result<AssemblyDelegateLoader<A>, HostingError> {
        self.get_delegate_loader()
            .map(|loader| AssemblyDelegateLoader::new(loader, assembly_path))
    }

    /// Closes an initialized host context.
    /// This method is automatically called on drop, but can be explicitely called to handle errors during closing.
    /// This should only be called once active references to the underlying hostfxr library have been dropped (e.g. through [`ManagedFunction`](crate::hostfxr::ManagedFunction)).
    pub unsafe fn close(self) -> Result<HostingSuccess, HostingError> {
        let result = unsafe { self._close() };
        self.destruct_drop();
        result
    }

    /// Internal non-consuming version of [`close`](HostfxrContext::close)
    unsafe fn _close(&self) -> Result<HostingSuccess, HostingError> {
        let result = unsafe { self.hostfxr.hostfxr_close(self.handle.as_raw()) };
        HostingResult::from(result).into_result()
    }
}

impl HostfxrContext<InitializedForCommandLine> {
    /// Load the dotnet runtime and run the application.
    ///
    /// # Return value
    /// If the app was successfully run, the exit code of the application. Otherwise, the error code result.
    #[must_use]
    pub fn run_app(self) -> AppOrHostingResult {
        let result = unsafe { self.hostfxr.hostfxr_run_app(self.handle.as_raw()) };
        AppOrHostingResult::from(result)
    }
}

impl<I> Drop for HostfxrContext<I> {
    fn drop(&mut self) {
        let _ = unsafe { self._close() };
    }
}
