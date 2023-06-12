use crate::{
    bindings::hostfxr::{
        hostfxr_delegate_type, hostfxr_handle,
        load_assembly_and_get_function_pointer_fn,
    },
    error::{HostingError, HostingResult, HostingSuccess},
    hostfxr::{
        AppOrHostingResult, AssemblyDelegateLoader, DelegateLoader, Hostfxr, HostfxrLibrary,
        RawFunctionPtr, SharedHostfxrLibrary,
    },
    pdcstring::PdCString,
};

#[cfg(feature = "net5_0")]
use crate::bindings::hostfxr::get_function_pointer_fn;

use std::{
    ffi::c_void,
    marker::PhantomData,
    mem::{self, ManuallyDrop, MaybeUninit},
    ptr::NonNull,
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
        Self(unsafe { NonNull::new_unchecked(ptr.cast_mut()) })
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

/// State which hostfxr creates and maintains and represents a logical operation on the hosting components.
#[derive(DestructDrop)]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "netcore3_0")))]
pub struct HostfxrContext<I> {
    handle: HostfxrHandle,
    hostfxr: SharedHostfxrLibrary,
    context_type: PhantomData<I>,
    is_primary: bool,
    not_thread_safe: PhantomData<Rc<HostfxrLibrary>>,
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
            not_thread_safe: PhantomData,
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

    #[must_use]
    pub(crate) fn library(&self) -> &SharedHostfxrLibrary {
        &self.hostfxr
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
    #[cfg(feature = "net5_0")]
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
            #[cfg(feature = "net5_0")]
            get_function_pointer: self.get_get_function_pointer_delegate()?,
            hostfxr: self.hostfxr.clone(),
        })
    }

    /// Gets a delegate loader for loading function pointers of the assembly with the given path.
    /// The assembly will be loaded lazily when the first function pointer is loaded.
    pub fn get_delegate_loader_for_assembly(
        &self,
        assembly_path: impl Into<PdCString>,
    ) -> Result<AssemblyDelegateLoader, HostingError> {
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
