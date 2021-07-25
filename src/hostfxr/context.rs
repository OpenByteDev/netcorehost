use crate::{
    bindings::hostfxr::{
        get_function_pointer_fn, hostfxr_delegate_type, hostfxr_handle,
        load_assembly_and_get_function_pointer_fn,
    },
    pdcstring::{PdCStr, PdCString},
    Error,
};

use std::{
    collections::HashMap,
    marker::PhantomData,
    mem::{self, MaybeUninit},
    ptr::{self, NonNull},
};

use super::{
    AssemblyDelegateLoader, DelegateLoader, HostExitCode, Hostfxr, KnownHostExitCode,
    MethodWithUnknownSignature,
};

/// A marker struct indicating that the context was initialized with a runtime config.
/// This means that it is not possible to run the application associated with the context.
pub struct InitializedForRuntimeConfig;

/// A marker struct indicating that the context was initialized for the dotnet command line.
/// This means that it is possible to run the application associated with the context.
pub struct InitializedForCommandLine;

/// Handle of a loaded [`HostfxrContext`].
#[derive(Debug, Clone, Copy)]
pub(crate) struct HostfxrHandle(NonNull<()>);

impl HostfxrHandle {
    // pub(crate) fn new(ptr: hostfxr_handle) -> Option<Self> {
    //     NonNull::new(ptr as *mut _).map(Self)
    // }
    pub(crate) unsafe fn new_unchecked(ptr: hostfxr_handle) -> Self {
        Self(NonNull::new_unchecked(ptr as *mut _))
    }
    pub(crate) fn as_raw(&self) -> hostfxr_handle {
        self.0.as_ptr()
    }
}

impl From<HostfxrHandle> for hostfxr_handle {
    fn from(handle: HostfxrHandle) -> Self {
        handle.as_raw()
    }
}

/// State which hostfxr creates and maintains and represents a logical operation on the hosting components.
#[derive(Clone)]
pub struct HostfxrContext<'a, I> {
    handle: HostfxrHandle,
    hostfxr: &'a Hostfxr,
    context_type: PhantomData<&'a I>,
}

impl<'a, I> HostfxrContext<'a, I> {
    pub(crate) fn new(handle: HostfxrHandle, hostfxr: &'a Hostfxr) -> Self {
        Self {
            handle,
            hostfxr,
            context_type: PhantomData,
        }
    }

    /// Gets the runtime property value for the given key of this host context.
    pub fn get_runtime_property_value_owned(
        &self,
        name: impl AsRef<PdCStr>,
    ) -> Result<PdCString, Error> {
        unsafe { self.get_runtime_property_value_borrowed(name) }.map(|str| str.to_owned())
    }

    /// Gets the runtime property value for the given key of this host context.
    ///
    /// # Safety
    /// The value string is owned by the host context. The lifetime of the buffer is only
    /// guaranteed until any of the below occur:
    ///  * [`run_app`] is called for this host context
    ///  * properties are changed via [`set_runtime_property_value`] or [`remove_runtime_property_value`]
    ///  * the host context is dropped
    ///
    /// [`run_app`]: HostfxrContext::run_app
    /// [`set_runtime_property_value`]: HostfxrContext::set_runtime_property_value
    /// [`remove_runtime_property_value`]: HostfxrContext::remove_runtime_property_value
    pub unsafe fn get_runtime_property_value_borrowed(
        &self,
        name: impl AsRef<PdCStr>,
    ) -> Result<&'a PdCStr, Error> {
        let mut value = MaybeUninit::uninit();

        let result = self.hostfxr.lib.hostfxr_get_runtime_property_value(
            self.handle.as_raw(),
            name.as_ref().as_ptr(),
            value.as_mut_ptr(),
        );
        HostExitCode::from(result).to_result()?;

        Ok(PdCStr::from_str_ptr(value.assume_init()))
    }

    /// Sets the value of a runtime property for this host context.
    pub fn set_runtime_property_value(
        &self,
        name: impl AsRef<PdCStr>,
        value: impl AsRef<PdCStr>,
    ) -> Result<(), Error> {
        let result = unsafe {
            self.hostfxr.lib.hostfxr_set_runtime_property_value(
                self.handle.as_raw(),
                name.as_ref().as_ptr(),
                value.as_ref().as_ptr(),
            )
        };
        HostExitCode::from(result).to_result().map(|_| ())
    }

    /// Remove a runtime property for this host context.
    pub fn remove_runtime_property_value(&self, name: impl AsRef<PdCStr>) -> Result<(), Error> {
        let result = unsafe {
            self.hostfxr.lib.hostfxr_set_runtime_property_value(
                self.handle.as_raw(),
                name.as_ref().as_ptr(),
                ptr::null(),
            )
        };
        HostExitCode::from(result).to_result().map(|_| ())
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
    pub unsafe fn get_runtime_properties_borrowed(
        &self,
    ) -> Result<(Vec<&'a PdCStr>, Vec<&'a PdCStr>), Error> {
        // get count
        let mut count = MaybeUninit::uninit();
        let result = self.hostfxr.lib.hostfxr_get_runtime_properties(
            self.handle.as_raw(),
            count.as_mut_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
        );

        // ignore buffer too small error
        match HostExitCode::from(result).to_result() {
            Err(Error::Hostfxr(HostExitCode::Known(KnownHostExitCode::HostApiBufferTooSmall))) => {
                Ok(())
            }
            res => res,
        }?;

        // get values / fill buffer
        let mut count = count.assume_init();
        let mut keys = Vec::with_capacity(count);
        let mut values = Vec::with_capacity(count);
        let result = self.hostfxr.lib.hostfxr_get_runtime_properties(
            self.handle.as_raw(),
            &mut count,
            keys.as_mut_ptr(),
            values.as_mut_ptr(),
        );
        HostExitCode::from(result).to_result()?;

        keys.set_len(count);
        values.set_len(count);

        let keys = keys.into_iter().map(|e| PdCStr::from_str_ptr(e)).collect();
        let values = values
            .into_iter()
            .map(|e| PdCStr::from_str_ptr(e))
            .collect();

        Ok((keys, values))
    }

    /// Get all runtime properties for this host context as owned strings.
    pub fn get_runtime_properties_owned(&self) -> Result<(Vec<PdCString>, Vec<PdCString>), Error> {
        unsafe { self.get_runtime_properties_borrowed() }.map(|(keys, values)| {
            let owned_keys = keys.into_iter().map(|key| key.to_owned()).collect();
            let owned_values = values.into_iter().map(|value| value.to_owned()).collect();
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
    pub unsafe fn get_runtime_properties_iter_borrowed(
        &self,
    ) -> Result<impl Iterator<Item = (&'a PdCStr, &'a PdCStr)>, Error> {
        self.get_runtime_properties_borrowed()
            .map(|(keys, values)| keys.into_iter().zip(values.into_iter()))
    }

    /// Get all runtime properties for this host context as an iterator over owned key-value pairs.
    pub fn get_runtime_properties_iter_owned(
        &self,
    ) -> Result<impl Iterator<Item = (PdCString, PdCString)>, Error> {
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
    pub unsafe fn get_runtime_properties_borrowed_as_map(
        &self,
    ) -> Result<HashMap<&'a PdCStr, &'a PdCStr>, Error> {
        self.get_runtime_properties_borrowed()
            .map(|(keys, values)| keys.into_iter().zip(values.into_iter()).collect())
    }

    /// Get all runtime properties for this host context as an hashmap of owned strings.
    pub fn get_runtime_properties_owned_as_map(
        &self,
    ) -> Result<HashMap<PdCString, PdCString>, Error> {
        self.get_runtime_properties_iter_owned()
            .map(|iter| iter.collect())
    }

    /// Gets a typed delegate from the currently loaded CoreCLR or from a newly created one.
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
    ) -> Result<MethodWithUnknownSignature, Error> {
        let mut delegate = MaybeUninit::uninit();
        let result = unsafe {
            self.hostfxr.lib.hostfxr_get_runtime_delegate(
                self.handle.as_raw(),
                r#type,
                delegate.as_mut_ptr(),
            )
        };

        HostExitCode::from(result).to_result()?;

        Ok(unsafe { delegate.assume_init() })
    }
    fn get_load_assembly_and_get_function_pointer_delegate(
        &self,
    ) -> Result<load_assembly_and_get_function_pointer_fn, Error> {
        unsafe {
            self.get_runtime_delegate(
                hostfxr_delegate_type::hdt_load_assembly_and_get_function_pointer,
            )
            .map(|ptr| mem::transmute(ptr))
        }
    }
    fn get_get_function_pointer_delegate(&self) -> Result<get_function_pointer_fn, Error> {
        unsafe {
            self.get_runtime_delegate(hostfxr_delegate_type::hdt_get_function_pointer)
                .map(|ptr| mem::transmute(ptr))
        }
    }

    /// Gets a delegate loader for loading an assembly and contained function pointers.
    pub fn get_delegate_loader(&self) -> Result<DelegateLoader, Error> {
        Ok(DelegateLoader {
            get_load_assembly_and_get_function_pointer: self
                .get_load_assembly_and_get_function_pointer_delegate()?,
            get_function_pointer: self.get_get_function_pointer_delegate()?,
        })
    }

    /// Gets a delegate loader for loading function pointers of the assembly with the given path.
    /// The assembly will be loaded lazily when the first function pointer is loaded.
    pub fn get_delegate_loader_for_assembly<A: AsRef<PdCStr>>(
        &self,
        assembly_path: A,
    ) -> Result<AssemblyDelegateLoader<A>, Error> {
        self.get_delegate_loader()
            .map(|loader| AssemblyDelegateLoader::new(loader, assembly_path))
    }

    unsafe fn close(&self) -> Result<(), Error> {
        self.hostfxr.lib.hostfxr_close(self.handle.as_raw());
        Ok(())
    }
}

impl<'a> HostfxrContext<'a, InitializedForCommandLine> {
    /// Load CoreCLR and run the application.
    ///
    /// # Return value
    /// If the app was successfully run, the exit code of the application. Otherwise, the error code result.
    pub fn run_app(&self) -> HostExitCode {
        let result = unsafe { self.hostfxr.lib.hostfxr_run_app(self.handle.as_raw()) };
        HostExitCode::from(result)
    }
}

impl<I> Drop for HostfxrContext<'_, I> {
    fn drop(&mut self) {
        let _ = unsafe { self.close() };
    }
}
