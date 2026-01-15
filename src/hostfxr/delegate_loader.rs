use crate::{
    bindings::{
        char_t,
        hostfxr::{component_entry_point_fn, load_assembly_and_get_function_pointer_fn},
    },
    error::{HostingError, HostingResult, HostingSuccess},
    pdcstring::{PdCStr, PdCString},
};
use fn_ptr::{WithAbi, abi::System};
use num_enum::TryFromPrimitive;
use std::{convert::TryFrom, mem::MaybeUninit, path::Path, ptr};
use thiserror::Error;

use super::{FnPtr, ManagedFunction, RawFnPtr, SharedHostfxrLibrary};

#[cfg(feature = "net5_0")]
use crate::bindings::hostfxr::{UNMANAGED_CALLERS_ONLY_METHOD, get_function_pointer_fn};

/// A pointer to a function with the default signature.
pub type ManagedFunctionWithDefaultSignature = ManagedFunction<component_entry_point_fn>;
/// A pointer to a function with an unknown signature.
pub type ManagedFunctionWithUnknownSignature = ManagedFunction<RawFnPtr>;

/// A struct for loading pointers to managed functions for a given [`HostfxrContext`].
///
/// [`HostfxrContext`]: super::HostfxrContext
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "netcore3_0")))]
pub struct DelegateLoader {
    pub(crate) get_load_assembly_and_get_function_pointer:
        load_assembly_and_get_function_pointer_fn,
    #[cfg(feature = "net5_0")]
    pub(crate) get_function_pointer: get_function_pointer_fn,
    #[allow(unused)]
    pub(crate) hostfxr: SharedHostfxrLibrary,
}

impl Clone for DelegateLoader {
    fn clone(&self) -> Self {
        Self {
            get_load_assembly_and_get_function_pointer: self
                .get_load_assembly_and_get_function_pointer,
            #[cfg(feature = "net5_0")]
            get_function_pointer: self.get_function_pointer,
            hostfxr: self.hostfxr.clone(),
        }
    }
}

impl DelegateLoader {
    unsafe fn load_assembly_and_get_function_pointer_raw(
        &self,
        assembly_path: *const char_t,
        type_name: *const char_t,
        method_name: *const char_t,
        delegate_type_name: *const char_t,
    ) -> Result<RawFnPtr, GetManagedFunctionError> {
        let mut delegate = MaybeUninit::uninit();

        let result = unsafe {
            (self.get_load_assembly_and_get_function_pointer)(
                assembly_path,
                type_name,
                method_name,
                delegate_type_name,
                ptr::null(),
                delegate.as_mut_ptr(),
            )
        };
        GetManagedFunctionError::from_status_code(result)?;

        Ok(unsafe { delegate.assume_init() }.cast())
    }

    fn validate_assembly_path(
        assembly_path: impl AsRef<PdCStr>,
    ) -> Result<(), GetManagedFunctionError> {
        #[cfg(windows)]
        let assembly_path = assembly_path.as_ref().to_os_string();

        #[cfg(not(windows))]
        let assembly_path = <std::ffi::OsStr as std::os::unix::prelude::OsStrExt>::from_bytes(
            assembly_path.as_ref().as_slice(),
        );

        if Path::new(&assembly_path).exists() {
            Ok(())
        } else {
            Err(GetManagedFunctionError::AssemblyNotFound)
        }
    }

    #[cfg(feature = "net5_0")]
    unsafe fn get_function_pointer_raw(
        &self,
        type_name: *const char_t,
        method_name: *const char_t,
        delegate_type_name: *const char_t,
    ) -> Result<RawFnPtr, GetManagedFunctionError> {
        let mut delegate = MaybeUninit::uninit();

        let result = unsafe {
            (self.get_function_pointer)(
                type_name,
                method_name,
                delegate_type_name,
                ptr::null(),
                ptr::null(),
                delegate.as_mut_ptr(),
            )
        };
        GetManagedFunctionError::from_status_code(result)?;

        Ok(unsafe { delegate.assume_init() }.cast())
    }

    /// Calling this function will load the specified assembly in isolation (into its own `AssemblyLoadContext`)
    /// and it will use `AssemblyDependencyResolver` on it to provide dependency resolution.
    /// Once loaded it will find the specified type and method and return a native function pointer
    /// to that method.
    ///
    /// # Arguments
    ///  * `assembly_path`:
    ///    Path to the assembly to load.
    ///    In case of complex component, this should be the main assembly of the component (the one with the .deps.json next to it).
    ///    Note that this does not have to be the assembly from which the `type_name` and `method_name` are.
    ///  * `type_name`:
    ///    Assembly qualified type name to find
    ///  * `method_name`:
    ///    Name of the method on the `type_name` to find. The method must be static and must match the signature of `delegate_type_name`.
    ///  * `delegate_type_name`:
    ///    Assembly qualified delegate type name for the method signature.
    pub fn load_assembly_and_get_function<F: FnPtr + WithAbi<System>>(
        &self,
        assembly_path: &PdCStr,
        type_name: &PdCStr,
        method_name: &PdCStr,
        delegate_type_name: &PdCStr,
    ) -> Result<ManagedFunction<<F as WithAbi<System>>::F>, GetManagedFunctionError> {
        Self::validate_assembly_path(assembly_path)?;
        let function = unsafe {
            self.load_assembly_and_get_function_pointer_raw(
                assembly_path.as_ptr(),
                type_name.as_ptr(),
                method_name.as_ptr(),
                delegate_type_name.as_ptr(),
            )
        }?;
        Ok(ManagedFunction(unsafe {
            <<F as WithAbi<System>>::F>::from_ptr(function)
        }))
    }

    /// Calling this function will load the specified assembly in isolation (into its own `AssemblyLoadContext`)
    /// and it will use `AssemblyDependencyResolver` on it to provide dependency resolution.
    /// Once loaded it will find the specified type and method and return a native function pointer
    /// to that method.
    ///
    /// # Arguments
    ///  * `assembly_path`:
    ///    Path to the assembly to load.
    ///    In case of complex component, this should be the main assembly of the component (the one with the .deps.json next to it).
    ///    Note that this does not have to be the assembly from which the `type_name` and `method_name` are.
    ///  * `type_name`:
    ///    Assembly qualified type name to find
    ///  * `method_name`:
    ///    Name of the method on the `type_name` to find. The method must be static and must match the following signature:
    ///    `public delegate int ComponentEntryPoint(IntPtr args, int sizeBytes);`
    pub fn load_assembly_and_get_function_with_default_signature(
        &self,
        assembly_path: &PdCStr,
        type_name: &PdCStr,
        method_name: &PdCStr,
    ) -> Result<ManagedFunctionWithDefaultSignature, GetManagedFunctionError> {
        Self::validate_assembly_path(assembly_path)?;
        let function = unsafe {
            self.load_assembly_and_get_function_pointer_raw(
                assembly_path.as_ptr(),
                type_name.as_ptr(),
                method_name.as_ptr(),
                ptr::null(),
            )
        }?;
        Ok(ManagedFunction(unsafe { FnPtr::from_ptr(function) }))
    }

    /// Calling this function will load the specified assembly in isolation (into its own `AssemblyLoadContext`)
    /// and it will use `AssemblyDependencyResolver` on it to provide dependency resolution.
    /// Once loaded it will find the specified type and method and return a native function pointer
    /// to that method. The target method has to be annotated with the [`UnmanagedCallersOnlyAttribute`].
    ///
    /// # Arguments
    ///  * `assembly_path`:
    ///    Path to the assembly to load.
    ///    In case of complex component, this should be the main assembly of the component (the one with the .deps.json next to it).
    ///    Note that this does not have to be the assembly from which the `type_name` and `method_name` are.
    ///  * `type_name`:
    ///    Assembly qualified type name to find
    ///  * `method_name`:
    ///    Name of the method on the `type_name` to find. The method must be static and must match be annotated with [`[UnmanagedCallersOnly]`][UnmanagedCallersOnly].
    ///
    /// [`UnmanagedCallersOnlyAttribute`]: <https://docs.microsoft.com/en-us/dotnet/api/system.runtime.interopservices.unmanagedcallersonlyattribute>
    /// [UnmanagedCallersOnly]: <https://docs.microsoft.com/en-us/dotnet/api/system.runtime.interopservices.unmanagedcallersonlyattribute>
    #[cfg(feature = "net5_0")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "net5_0")))]
    pub fn load_assembly_and_get_function_with_unmanaged_callers_only<
        F: FnPtr + WithAbi<System>,
    >(
        &self,
        assembly_path: &PdCStr,
        type_name: &PdCStr,
        method_name: &PdCStr,
    ) -> Result<ManagedFunction<<F as WithAbi<System>>::F>, GetManagedFunctionError> {
        Self::validate_assembly_path(assembly_path)?;
        let function = unsafe {
            self.load_assembly_and_get_function_pointer_raw(
                assembly_path.as_ptr(),
                type_name.as_ptr(),
                method_name.as_ptr(),
                UNMANAGED_CALLERS_ONLY_METHOD,
            )
        }?;
        Ok(ManagedFunction(unsafe {
            <<F as WithAbi<System>>::F>::from_ptr(function)
        }))
    }

    /// Calling this function will find the specified type and method and return a native function pointer to that method.
    /// This will **NOT** load the containing assembly.
    ///
    /// # Arguments
    ///  * `type_name`:
    ///    Assembly qualified type name to find
    ///  * `method_name`:
    ///    Name of the method on the `type_name` to find. The method must be static and must match the signature of `delegate_type_name`.
    ///  * `delegate_type_name`:
    ///    Assembly qualified delegate type name for the method signature.
    #[cfg(feature = "net5_0")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "net5_0")))]
    pub fn get_function<F: FnPtr + WithAbi<System>>(
        &self,
        type_name: &PdCStr,
        method_name: &PdCStr,
        delegate_type_name: &PdCStr,
    ) -> Result<ManagedFunction<<F as WithAbi<System>>::F>, GetManagedFunctionError> {
        let function = unsafe {
            self.get_function_pointer_raw(
                type_name.as_ptr(),
                method_name.as_ptr(),
                delegate_type_name.as_ptr(),
            )
        }?;
        Ok(ManagedFunction(unsafe {
            <<F as WithAbi<System>>::F>::from_ptr(function)
        }))
    }

    /// Calling this function will find the specified type and method and return a native function pointer to that method.
    /// This will **NOT** load the containing assembly.
    ///
    /// # Arguments
    ///  * `type_name`:
    ///    Assembly qualified type name to find
    ///  * `method_name`:
    ///    Name of the method on the `type_name` to find. The method must be static and must match the following signature:
    ///    `public delegate int ComponentEntryPoint(IntPtr args, int sizeBytes);`
    #[cfg(feature = "net5_0")]
    pub fn get_function_with_default_signature(
        &self,
        type_name: &PdCStr,
        method_name: &PdCStr,
    ) -> Result<ManagedFunctionWithDefaultSignature, GetManagedFunctionError> {
        let function = unsafe {
            self.get_function_pointer_raw(type_name.as_ptr(), method_name.as_ptr(), ptr::null())
        }?;
        Ok(ManagedFunction(unsafe { FnPtr::from_ptr(function) }))
    }

    /// Calling this function will find the specified type and method and return a native function pointer to that method.
    /// This will **NOT** load the containing assembly.
    ///
    /// # Arguments
    ///  * `type_name`:
    ///    Assembly qualified type name to find
    ///  * `method_name`:
    ///    Name of the method on the `type_name` to find. The method must be static and must match be annotated with [`UnmanagedCallersOnly`].
    ///
    /// [`UnmanagedCallersOnlyAttribute`]: https://docs.microsoft.com/en-us/dotnet/api/system.runtime.interopservices.unmanagedcallersonlyattribute
    /// [`UnmanagedCallersOnly`]: https://docs.microsoft.com/en-us/dotnet/api/system.runtime.interopservices.unmanagedcallersonlyattribute
    #[cfg(feature = "net5_0")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "net5_0")))]
    pub fn get_function_with_unmanaged_callers_only<F: FnPtr + WithAbi<System>>(
        &self,
        type_name: &PdCStr,
        method_name: &PdCStr,
    ) -> Result<ManagedFunction<<F as WithAbi<System>>::F>, GetManagedFunctionError> {
        let function = unsafe {
            self.get_function_pointer_raw(
                type_name.as_ptr(),
                method_name.as_ptr(),
                UNMANAGED_CALLERS_ONLY_METHOD,
            )
        }?;
        Ok(ManagedFunction(unsafe {
            <<F as WithAbi<System>>::F>::from_ptr(function)
        }))
    }
}

/// A struct for loading pointers to managed functions for a given [`HostfxrContext`] which automatically loads the
/// assembly from the given path on the first access.
///
/// [`HostfxrContext`]: super::HostfxrContext
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "netcore3_0")))]
#[derive(Clone)]
pub struct AssemblyDelegateLoader {
    loader: DelegateLoader,
    assembly_path: PdCString,
}

impl AssemblyDelegateLoader {
    /// Creates a new [`AssemblyDelegateLoader`] wrapping the given [`DelegateLoader`] loading the assembly
    /// from the given path on the first access.
    pub fn new(loader: DelegateLoader, assembly_path: impl Into<PdCString>) -> Self {
        let assembly_path = assembly_path.into();
        Self {
            loader,
            assembly_path,
        }
    }

    /// If this is the first loaded function pointer, calling this function will load the specified assembly in
    /// isolation (into its own `AssemblyLoadContext`) and it will use `AssemblyDependencyResolver` on it to provide
    /// dependency resolution.
    /// Otherwise or once loaded it will find the specified type and method and return a native function pointer to that method.
    /// Calling this function will find the specified type and method and return a native function pointer to that method.
    ///
    /// # Arguments
    ///  * `type_name`:
    ///    Assembly qualified type name to find
    ///  * `method_name`:
    ///    Name of the method on the `type_name` to find. The method must be static and must match the signature of `delegate_type_name`.
    ///  * `delegate_type_name`:
    ///    Assembly qualified delegate type name for the method signature.
    pub fn get_function<F: FnPtr + WithAbi<System>>(
        &self,
        type_name: &PdCStr,
        method_name: &PdCStr,
        delegate_type_name: &PdCStr,
    ) -> Result<ManagedFunction<<F as WithAbi<System>>::F>, GetManagedFunctionError> {
        self.loader.load_assembly_and_get_function::<F>(
            self.assembly_path.as_ref(),
            type_name,
            method_name,
            delegate_type_name,
        )
    }

    /// If this is the first loaded function pointer, calling this function will load the specified assembly in
    /// isolation (into its own `AssemblyLoadContext`) and it will use `AssemblyDependencyResolver` on it to provide
    /// dependency resolution.
    /// Otherwise or once loaded it will find the specified type and method and return a native function pointer to that method.
    /// Calling this function will find the specified type and method and return a native function pointer to that method.
    ///
    /// # Arguments
    ///  * `type_name`:
    ///    Assembly qualified type name to find
    ///  * `method_name`:
    ///    Name of the method on the `type_name` to find. The method must be static and must match the following signature:
    ///    `public delegate int ComponentEntryPoint(IntPtr args, int sizeBytes);`
    pub fn get_function_with_default_signature(
        &self,
        type_name: &PdCStr,
        method_name: &PdCStr,
    ) -> Result<ManagedFunctionWithDefaultSignature, GetManagedFunctionError> {
        self.loader
            .load_assembly_and_get_function_with_default_signature(
                self.assembly_path.as_ref(),
                type_name,
                method_name,
            )
    }

    /// If this is the first loaded function pointer, calling this function will load the specified assembly in
    /// isolation (into its own `AssemblyLoadContext`) and it will use `AssemblyDependencyResolver` on it to provide
    /// dependency resolution.
    /// Otherwise or once loaded it will find the specified type and method and return a native function pointer to that method.
    /// Calling this function will find the specified type and method and return a native function pointer to that method.
    ///
    /// # Arguments
    ///  * `type_name`:
    ///    Assembly qualified type name to find
    ///  * `method_name`:
    ///    Name of the method on the `type_name` to find. The method must be static and must match be annotated with [`UnmanagedCallersOnly`].
    ///
    /// [`UnmanagedCallersOnlyAttribute`]: https://docs.microsoft.com/en-us/dotnet/api/system.runtime.interopservices.unmanagedcallersonlyattribute
    /// [`UnmanagedCallersOnly`]: https://docs.microsoft.com/en-us/dotnet/api/system.runtime.interopservices.unmanagedcallersonlyattribute
    #[cfg(feature = "net5_0")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "net5_0")))]
    pub fn get_function_with_unmanaged_callers_only<F: FnPtr + WithAbi<System>>(
        &self,
        type_name: &PdCStr,
        method_name: &PdCStr,
    ) -> Result<ManagedFunction<<F as WithAbi<System>>::F>, GetManagedFunctionError> {
        self.loader
            .load_assembly_and_get_function_with_unmanaged_callers_only::<F>(
                self.assembly_path.as_ref(),
                type_name,
                method_name,
            )
    }
}

/// Enum for errors that can occur while loading a managed assembly or managed function pointers.
#[derive(Error, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "netcore3_0")))]
pub enum GetManagedFunctionError {
    /// An error occured inside the hosting components.
    #[error("Error from hosting components: {}.", .0)]
    Hosting(#[from] HostingError),

    /// A type with the specified name could not be found or loaded.
    #[error("Failed to load the type or method or it has an incompatible signature.")]
    TypeOrMethodNotFound,

    /// The specified assembly could not be found.
    #[error("The specified assembly could not be found.")]
    AssemblyNotFound,

    /// The target method is not annotated with [`UnmanagedCallersOnly`](https://docs.microsoft.com/en-us/dotnet/api/system.runtime.interopservices.unmanagedcallersonlyattribute).
    #[error("The target method is not annotated with UnmanagedCallersOnly.")]
    MethodNotUnmanagedCallersOnly,

    /// Some other unknown error occured.
    #[error("Unknown error code: {}", format!("{:#08X}", .0))]
    Other(u32),
}

impl GetManagedFunctionError {
    /// Converts the given staus code to a [`GetManagedFunctionError`].
    pub fn from_status_code(code: i32) -> Result<HostingSuccess, Self> {
        let code = code as u32;
        match HostingResult::known_from_status_code(code) {
            Ok(HostingResult(Ok(code))) => return Ok(code),
            Ok(HostingResult(Err(code))) => return Err(GetManagedFunctionError::Hosting(code)),
            _ => {}
        }
        match HResult::try_from(code) {
            Ok(
                HResult::COR_E_TYPELOAD | HResult::COR_E_MISSINGMETHOD | HResult::COR_E_ARGUMENT,
            ) => return Err(Self::TypeOrMethodNotFound),
            Ok(HResult::FILE_NOT_FOUND) => return Err(Self::AssemblyNotFound),
            Ok(HResult::COR_E_INVALIDOPERATION) => return Err(Self::MethodNotUnmanagedCallersOnly),
            _ => {}
        }
        Err(Self::Other(code))
    }
}

#[repr(u32)]
#[non_exhaustive]
#[derive(TryFromPrimitive, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[rustfmt::skip]
enum HResult {
    E_POINTER = 0x8000_4003,                 // System.ArgumentNullException
    COR_E_ARGUMENTOUTOFRANGE = 0x8013_1502,  // System.ArgumentOutOfRangeException (reserved was not 0)
    COR_E_TYPELOAD = 0x8013_1522,            // invalid type
    COR_E_MISSINGMETHOD = 0x8013_1513,       // invalid method
    /*COR_E_*/FILE_NOT_FOUND = 0x8007_0002,  // assembly with specified name not found (from type name)
    COR_E_ARGUMENT = 0x8007_0057,            // invalid method signature or method not found
    COR_E_INVALIDOPERATION = 0x8013_1509,    // invalid assembly path or not unmanaged,
}
