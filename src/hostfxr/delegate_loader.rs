use crate::{
    bindings::{
        char_t,
        consts::UNMANAGED_CALLERS_ONLY_METHOD,
        hostfxr::{
            component_entry_point_fn, get_function_pointer_fn,
            load_assembly_and_get_function_pointer_fn,
        },
    },
    error::{HostingError, HostingResult, HostingSuccess},
    pdcstring::PdCStr,
};
use num_enum::TryFromPrimitive;
use std::{
    convert::TryFrom,
    mem::{self, MaybeUninit},
    path::PathBuf,
    ptr,
};
use thiserror::Error;

/// A function pointer for a method with the default signature.
pub type MethodWithDefaultSignature = component_entry_point_fn;
/// A opaque type representing some method with an unknown signature.
pub enum SomeMethod {}
/// A function pointer for a method with an unknown signature.
pub type MethodWithUnknownSignature = *const SomeMethod;

/// A struct for loading pointers to managed functions for a given [`HostfxrContext`].
///
/// [`HostfxrContext`]: super::HostfxrContext
#[derive(Copy, Clone)]
pub struct DelegateLoader {
    pub(crate) get_load_assembly_and_get_function_pointer:
        load_assembly_and_get_function_pointer_fn,
    pub(crate) get_function_pointer: get_function_pointer_fn,
}

impl DelegateLoader {
    unsafe fn _load_assembly_and_get_function_pointer(
        &self,
        assembly_path: *const char_t,
        type_name: *const char_t,
        method_name: *const char_t,
        delegate_type_name: *const char_t,
    ) -> Result<MethodWithUnknownSignature, GetFunctionPointerError> {
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
        GetFunctionPointerError::from_status_code(result)?;

        Ok(unsafe { mem::transmute(delegate.assume_init()) })
    }

    fn _validate_assembly_path(
        assembly_path: impl AsRef<PdCStr>,
    ) -> Result<(), GetFunctionPointerError> {
        if PathBuf::from(assembly_path.as_ref().to_os_string()).exists() {
            Ok(())
        } else {
            Err(GetFunctionPointerError::AssemblyNotFound)
        }
    }

    unsafe fn _get_function_pointer(
        &self,
        type_name: *const char_t,
        method_name: *const char_t,
        delegate_type_name: *const char_t,
    ) -> Result<MethodWithUnknownSignature, GetFunctionPointerError> {
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
        GetFunctionPointerError::from_status_code(result)?;

        Ok(unsafe { mem::transmute(delegate.assume_init()) })
    }

    /// Calling this function will load the specified assembly in isolation (into its own `AssemblyLoadContext`)
    /// and it will use `AssemblyDependencyResolver` on it to provide dependency resolution.
    /// Once loaded it will find the specified type and method and return a native function pointer
    /// to that method.
    ///
    /// # Arguments
    ///  * `assembly_path`:
    ///     Path to the assembly to load.
    ///     In case of complex component, this should be the main assembly of the component (the one with the .deps.json next to it).
    ///     Note that this does not have to be the assembly from which the `type_name` and `method_name` are.
    ///  * `type_name`:
    ///     Assembly qualified type name to find
    ///  * `method_name`:
    ///     Name of the method on the `type_name` to find. The method must be static and must match the signature of `delegate_type_name`.
    ///  * `delegate_type_name`:
    ///     Assembly qualified delegate type name for the method signature.
    pub fn load_assembly_and_get_function_pointer(
        &self,
        assembly_path: impl AsRef<PdCStr>,
        type_name: impl AsRef<PdCStr>,
        method_name: impl AsRef<PdCStr>,
        delegate_type_name: impl AsRef<PdCStr>,
    ) -> Result<MethodWithUnknownSignature, GetFunctionPointerError> {
        Self::_validate_assembly_path(assembly_path.as_ref())?;
        unsafe {
            self._load_assembly_and_get_function_pointer(
                assembly_path.as_ref().as_ptr(),
                type_name.as_ref().as_ptr(),
                method_name.as_ref().as_ptr(),
                delegate_type_name.as_ref().as_ptr(),
            )
        }
    }

    /// Calling this function will load the specified assembly in isolation (into its own `AssemblyLoadContext`)
    /// and it will use `AssemblyDependencyResolver` on it to provide dependency resolution.
    /// Once loaded it will find the specified type and method and return a native function pointer
    /// to that method.
    ///
    /// # Arguments
    ///  * `assembly_path`:
    ///     Path to the assembly to load.
    ///     In case of complex component, this should be the main assembly of the component (the one with the .deps.json next to it).
    ///     Note that this does not have to be the assembly from which the `type_name` and `method_name` are.
    ///  * `type_name`:
    ///     Assembly qualified type name to find
    ///  * `method_name`:
    ///     Name of the method on the `type_name` to find. The method must be static and must match the following signature:
    ///     `public delegate int ComponentEntryPoint(IntPtr args, int sizeBytes);`
    pub fn load_assembly_and_get_function_pointer_with_default_signature(
        &self,
        assembly_path: impl AsRef<PdCStr>,
        type_name: impl AsRef<PdCStr>,
        method_name: impl AsRef<PdCStr>,
    ) -> Result<MethodWithDefaultSignature, GetFunctionPointerError> {
        Self::_validate_assembly_path(assembly_path.as_ref())?;
        unsafe {
            self._load_assembly_and_get_function_pointer(
                assembly_path.as_ref().as_ptr(),
                type_name.as_ref().as_ptr(),
                method_name.as_ref().as_ptr(),
                ptr::null(),
            )
            .map(|fn_ptr| mem::transmute(fn_ptr))
        }
    }

    /// Calling this function will load the specified assembly in isolation (into its own `AssemblyLoadContext`)
    /// and it will use `AssemblyDependencyResolver` on it to provide dependency resolution.
    /// Once loaded it will find the specified type and method and return a native function pointer
    /// to that method. The target method has to be annotated with the [`UnmanagedCallersOnlyAttribute`].
    ///
    /// # Arguments
    ///  * `assembly_path`:
    ///     Path to the assembly to load.
    ///     In case of complex component, this should be the main assembly of the component (the one with the .deps.json next to it).
    ///     Note that this does not have to be the assembly from which the `type_name` and `method_name` are.
    ///  * `type_name`:
    ///     Assembly qualified type name to find
    ///  * `method_name`:
    ///     Name of the method on the `type_name` to find. The method must be static and must match be annotated with [`\[UnmanagedCallersOnly\]`].
    ///
    /// [`UnmanagedCallersOnlyAttribute`]: https://docs.microsoft.com/en-us/dotnet/api/system.runtime.interopservices.unmanagedcallersonlyattribute
    /// [`\[UnmanagedCallersOnly\]`]: https://docs.microsoft.com/en-us/dotnet/api/system.runtime.interopservices.unmanagedcallersonlyattribute
    pub fn load_assembly_and_get_function_pointer_for_unmanaged_callers_only_method(
        &self,
        assembly_path: impl AsRef<PdCStr>,
        type_name: impl AsRef<PdCStr>,
        method_name: impl AsRef<PdCStr>,
    ) -> Result<MethodWithUnknownSignature, GetFunctionPointerError> {
        Self::_validate_assembly_path(assembly_path.as_ref())?;
        unsafe {
            self._load_assembly_and_get_function_pointer(
                assembly_path.as_ref().as_ptr(),
                type_name.as_ref().as_ptr(),
                method_name.as_ref().as_ptr(),
                UNMANAGED_CALLERS_ONLY_METHOD,
            )
        }
    }

    /// Calling this function will find the specified type and method and return a native function pointer to that method.
    /// This will **NOT** load the containing assembly.
    ///
    /// # Arguments
    ///  * `type_name`:
    ///     Assembly qualified type name to find
    ///  * `method_name`:
    ///     Name of the method on the `type_name` to find. The method must be static and must match the signature of `delegate_type_name`.
    ///  * `delegate_type_name`:
    ///     Assembly qualified delegate type name for the method signature.
    pub fn get_function_pointer(
        &self,
        type_name: impl AsRef<PdCStr>,
        method_name: impl AsRef<PdCStr>,
        delegate_type_name: impl AsRef<PdCStr>,
    ) -> Result<MethodWithUnknownSignature, GetFunctionPointerError> {
        unsafe {
            self._get_function_pointer(
                type_name.as_ref().as_ptr(),
                method_name.as_ref().as_ptr(),
                delegate_type_name.as_ref().as_ptr(),
            )
        }
    }

    /// Calling this function will find the specified type and method and return a native function pointer to that method.
    /// This will **NOT** load the containing assembly.
    ///
    /// # Arguments
    ///  * `type_name`:
    ///     Assembly qualified type name to find
    ///  * `method_name`:
    ///     Name of the method on the `type_name` to find. The method must be static and must match the following signature:
    ///     `public delegate int ComponentEntryPoint(IntPtr args, int sizeBytes);`
    pub fn get_function_pointer_with_default_signature(
        &self,
        type_name: impl AsRef<PdCStr>,
        method_name: impl AsRef<PdCStr>,
    ) -> Result<MethodWithDefaultSignature, GetFunctionPointerError> {
        unsafe {
            self._get_function_pointer(
                type_name.as_ref().as_ptr(),
                method_name.as_ref().as_ptr(),
                ptr::null(),
            )
            .map(|fn_ptr| mem::transmute(fn_ptr))
        }
    }

    /// Calling this function will find the specified type and method and return a native function pointer to that method.
    /// This will **NOT** load the containing assembly.
    ///
    /// # Arguments
    ///  * `type_name`:
    ///     Assembly qualified type name to find
    ///  * `method_name`:
    ///     Name of the method on the `type_name` to find. The method must be static and must match be annotated with [`\[UnmanagedCallersOnly\]`].
    ///
    /// [`UnmanagedCallersOnlyAttribute`]: https://docs.microsoft.com/en-us/dotnet/api/system.runtime.interopservices.unmanagedcallersonlyattribute
    /// [`\[UnmanagedCallersOnly\]`]: https://docs.microsoft.com/en-us/dotnet/api/system.runtime.interopservices.unmanagedcallersonlyattribute
    pub fn get_function_pointer_for_unmanaged_callers_only_method(
        &self,
        type_name: impl AsRef<PdCStr>,
        method_name: impl AsRef<PdCStr>,
    ) -> Result<MethodWithUnknownSignature, GetFunctionPointerError> {
        unsafe {
            self._get_function_pointer(
                type_name.as_ref().as_ptr(),
                method_name.as_ref().as_ptr(),
                UNMANAGED_CALLERS_ONLY_METHOD,
            )
        }
    }
}

/// A struct for loading pointers to managed functions for a given [`HostfxrContext`] which automatically loads the
/// assembly from the given path on the first access.
///
/// [`HostfxrContext`]: super::HostfxrContext
pub struct AssemblyDelegateLoader<A: AsRef<PdCStr>> {
    loader: DelegateLoader,
    assembly_path: A,
}

impl<A: AsRef<PdCStr>> AssemblyDelegateLoader<A> {
    /// Creates a new [`AssemblyDelegateLoader`] wrapping the given [`DelegateLoader`] loading the assembly
    /// from the given path on the first access.
    pub fn new(loader: DelegateLoader, assembly_path: A) -> Self {
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
    ///     Assembly qualified type name to find
    ///  * `method_name`:
    ///     Name of the method on the `type_name` to find. The method must be static and must match the signature of `delegate_type_name`.
    ///  * `delegate_type_name`:
    ///     Assembly qualified delegate type name for the method signature.
    pub fn get_function_pointer(
        &self,
        type_name: impl AsRef<PdCStr>,
        method_name: impl AsRef<PdCStr>,
        delegate_type_name: impl AsRef<PdCStr>,
    ) -> Result<MethodWithUnknownSignature, GetFunctionPointerError> {
        self.loader.load_assembly_and_get_function_pointer(
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
    ///     Assembly qualified type name to find
    ///  * `method_name`:
    ///     Name of the method on the `type_name` to find. The method must be static and must match the following signature:
    ///     `public delegate int ComponentEntryPoint(IntPtr args, int sizeBytes);`
    pub fn get_function_pointer_with_default_signature(
        &self,
        type_name: impl AsRef<PdCStr>,
        method_name: impl AsRef<PdCStr>,
    ) -> Result<MethodWithDefaultSignature, GetFunctionPointerError> {
        self.loader
            .load_assembly_and_get_function_pointer_with_default_signature(
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
    ///     Assembly qualified type name to find
    ///  * `method_name`:
    ///     Name of the method on the `type_name` to find. The method must be static and must match be annotated with [`\[UnmanagedCallersOnly\]`].
    ///
    /// [`UnmanagedCallersOnlyAttribute`]: https://docs.microsoft.com/en-us/dotnet/api/system.runtime.interopservices.unmanagedcallersonlyattribute
    /// [`\[UnmanagedCallersOnly\]`]: https://docs.microsoft.com/en-us/dotnet/api/system.runtime.interopservices.unmanagedcallersonlyattribute
    pub fn get_function_pointer_for_unmanaged_callers_only_method(
        &self,
        type_name: impl AsRef<PdCStr>,
        method_name: impl AsRef<PdCStr>,
    ) -> Result<MethodWithUnknownSignature, GetFunctionPointerError> {
        self.loader
            .load_assembly_and_get_function_pointer_for_unmanaged_callers_only_method(
                self.assembly_path.as_ref(),
                type_name,
                method_name,
            )
    }
}

/// Enum for errors that can occur while loading a managed assembly or managed function pointers.
#[derive(Error, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GetFunctionPointerError {
    /// An error occured inside the hosting components.
    #[error("Error from hosting components: {}.", .0)]
    Hosting(#[from] HostingError),

    /// A type with the specified name could not be found or loaded.
    #[error("Failed to load type containing method of delegate type.")]
    TypeNotFound,

    /// A method with the required signature and name could not be found.
    #[error("Specified method does not exists or has an incompatible signature.")]
    MissingMethod,

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

impl GetFunctionPointerError {
    pub fn from_status_code(code: i32) -> Result<HostingSuccess, Self> {
        let code = code as u32;
        match HostingResult::known_from_status_code(code) {
            Ok(HostingResult(Ok(code))) => return Ok(code),
            Ok(HostingResult(Err(code))) => return Err(GetFunctionPointerError::Hosting(code)),
            _ => {}
        }
        match HResult::try_from(code) {
            Ok(HResult::COR_E_TYPELOAD) => return Err(Self::TypeNotFound),
            Ok(HResult::COR_E_MISSINGMETHOD | HResult::COR_E_ARGUMENT) => {
                return Err(Self::MissingMethod)
            }
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
enum HResult {
    E_POINTER = 0x80004003,                // System.ArgumentNullException
    COR_E_ARGUMENTOUTOFRANGE = 0x80131502, // System.ArgumentOutOfRangeException (reserved was not 0)
    COR_E_TYPELOAD = 0x80131522,           // invalid type
    COR_E_MISSINGMETHOD = 2148734227,      // invalid method
    /*COR_E_*/
    FILE_NOT_FOUND = 2147942402, // assembly with specified name not found (from type name)
    COR_E_ARGUMENT = 0x80070057, // invalid method signature or method not found
    COR_E_INVALIDOPERATION = 0x80131509, // invalid assembly path or not unmanaged,
}
