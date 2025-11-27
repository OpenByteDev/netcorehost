use crate::{
    bindings::hostfxr::{
        hostfxr_delegate_type, hostfxr_handle, load_assembly_and_get_function_pointer_fn,
    },
    error::{HostingError, HostingResult, HostingSuccess},
    hostfxr::{
        AppOrHostingResult, AssemblyDelegateLoader, DelegateLoader, Hostfxr, HostfxrLibrary,
        RawFnPtr, SharedHostfxrLibrary,
    },
    pdcstring::PdCString,
};

#[cfg(feature = "net5_0")]
use crate::bindings::hostfxr::get_function_pointer_fn;
#[cfg(feature = "net8_0")]
use crate::{
    bindings::hostfxr::{load_assembly_bytes_fn, load_assembly_fn},
    pdcstring::PdCStr,
};

use std::{
    cell::Cell,
    ffi::c_void,
    fmt::{self, Debug},
    marker::PhantomData,
    mem::{self, ManuallyDrop, MaybeUninit},
    ptr::NonNull,
};

#[cfg(feature = "net8_0")]
use std::ptr;

use destruct_drop::DestructDrop;
use enum_map::EnumMap;
use once_cell::unsync::OnceCell;

/// A marker struct indicating that the context was initialized with a runtime config.
/// This means that it is not possible to run the application associated with the context.
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "netcore3_0")))]
#[derive(Debug, Clone, Copy)]
pub struct InitializedForRuntimeConfig;

/// A marker struct indicating that the context was initialized for the dotnet command line.
/// This means that it is possible to run the application associated with the context.
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "netcore3_0")))]
#[derive(Debug, Clone, Copy)]
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
    pub const unsafe fn new_unchecked(ptr: hostfxr_handle) -> Self {
        Self(unsafe { NonNull::new_unchecked(ptr.cast_mut()) })
    }

    /// Returns the raw underlying handle.
    #[must_use]
    pub const fn as_raw(&self) -> hostfxr_handle {
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
    is_primary: bool,
    runtime_delegates: EnumMap<hostfxr_delegate_type, OnceCell<RawFnPtr>>,
    context_type: PhantomData<I>,
    not_sync: PhantomData<Cell<HostfxrLibrary>>,
}

unsafe impl<I> Send for HostfxrContext<I> {}

impl<I> Debug for HostfxrContext<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HostfxrContext")
            .field("handle", &self.handle)
            .field("is_primary", &self.is_primary)
            .field("runtime_delegates", &self.runtime_delegates)
            .field("context_type", &self.context_type)
            .finish_non_exhaustive()
    }
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
            runtime_delegates: EnumMap::default(),
            context_type: PhantomData,
            not_sync: PhantomData,
        }
    }

    /// Gets the underlying handle to the hostfxr context.
    #[must_use]
    pub const fn handle(&self) -> HostfxrHandle {
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
    pub const fn is_primary(&self) -> bool {
        self.is_primary
    }

    #[must_use]
    pub(crate) const fn library(&self) -> &SharedHostfxrLibrary {
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
    ) -> Result<RawFnPtr, HostingError> {
        self.runtime_delegates[r#type]
            .get_or_try_init(|| self.get_runtime_delegate_uncached(r#type))
            .copied()
    }
    fn get_runtime_delegate_uncached(
        &self,
        r#type: hostfxr_delegate_type,
    ) -> Result<RawFnPtr, HostingError> {
        let mut delegate = MaybeUninit::uninit();
        let result = unsafe {
            self.hostfxr.hostfxr_get_runtime_delegate(
                self.handle.as_raw(),
                r#type,
                delegate.as_mut_ptr(),
            )
        }
        .unwrap();

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
    #[cfg(feature = "net8_0")]
    fn get_load_assembly_delegate(&self) -> Result<load_assembly_fn, HostingError> {
        unsafe {
            self.get_runtime_delegate(hostfxr_delegate_type::hdt_load_assembly)
                .map(|ptr| mem::transmute(ptr))
        }
    }
    #[cfg(feature = "net8_0")]
    fn get_load_assembly_bytes_delegate(&self) -> Result<load_assembly_bytes_fn, HostingError> {
        unsafe {
            self.get_runtime_delegate(hostfxr_delegate_type::hdt_load_assembly_bytes)
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

    /// Loads the specified assembly in the default load context from the given path.
    /// It uses [`AssemblyDependencyResolver`] to register additional dependency resolution for the load context.
    /// Function pointers to methods in the assembly can then be loaded using a [`DelegateLoader`].
    ///
    /// [`AssemblyDependencyResolver`]: https://learn.microsoft.com/en-us/dotnet/api/system.runtime.loader.assemblydependencyresolver
    /// [`AssemblyLoadContext.LoadFromAssembly`]: https://learn.microsoft.com/en-us/dotnet/api/system.runtime.loader.assemblyloadcontext.loadfromassemblypath
    #[cfg(feature = "net8_0")]
    pub fn load_assembly_from_path(
        &self,
        assembly_path: impl AsRef<PdCStr>,
    ) -> Result<(), HostingError> {
        let assembly_path = assembly_path.as_ref();
        let load_assembly = self.get_load_assembly_delegate()?;
        let result = unsafe { load_assembly(assembly_path.as_ptr(), ptr::null(), ptr::null()) };
        HostingResult::from(result).into_result()?;
        Ok(())
    }

    /// Loads the specified assembly in the default load context from the given buffers.
    /// It does not provide a mechanism for registering additional dependency resolution, as mechanisms like `.deps.json` and [`AssemblyDependencyResolver`] are file-based.
    /// Dependencies can be pre-loaded (for example, via a previous call to this function) or the specified assembly can explicitly register its own resolution logic (for example, via the [`AssemblyLoadContext.Resolving`] event).
    /// It uses [`AssemblyDependencyResolver`] to register additional dependency resolution for the load context.
    /// Function pointers to methods in the assembly can then be loaded using a [`DelegateLoader`].
    ///
    /// [`AssemblyDependencyResolver`]: https://learn.microsoft.com/en-us/dotnet/api/system.runtime.loader.assemblydependencyresolver
    /// [`AssemblyLoadContext.Resolving`]: https://learn.microsoft.com/en-us/dotnet/api/system.runtime.loader.assemblyloadcontext.resolving?view=net-7.0
    #[cfg(feature = "net8_0")]
    pub fn load_assembly_from_bytes(
        &self,
        assembly_bytes: impl AsRef<[u8]>,
        symbols_bytes: impl AsRef<[u8]>,
    ) -> Result<(), HostingError> {
        let symbols_bytes = symbols_bytes.as_ref();
        let assembly_bytes = assembly_bytes.as_ref();
        let load_assembly_bytes = self.get_load_assembly_bytes_delegate()?;
        let result = unsafe {
            load_assembly_bytes(
                assembly_bytes.as_ptr(),
                assembly_bytes.len(),
                symbols_bytes.as_ptr(),
                symbols_bytes.len(),
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };
        HostingResult::from(result).into_result()?;
        Ok(())
    }

    /// Closes an initialized host context.
    /// This method is automatically called on drop, but can be explicitely called to handle errors during closing.
    pub fn close(self) -> Result<HostingSuccess, HostingError> {
        let result = unsafe { self.close_raw() };
        self.destruct_drop();
        result
    }

    /// Internal non-consuming version of [`close`](HostfxrContext::close)
    unsafe fn close_raw(&self) -> Result<HostingSuccess, HostingError> {
        let result = unsafe { self.hostfxr.hostfxr_close(self.handle.as_raw()) }.unwrap();
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
        let result = unsafe { self.hostfxr.hostfxr_run_app(self.handle.as_raw()) }.unwrap();
        AppOrHostingResult::from(result)
    }
}

impl<I> Drop for HostfxrContext<I> {
    fn drop(&mut self) {
        let _ = unsafe { self.close_raw() };
    }
}
