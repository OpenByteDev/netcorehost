use crate::{
    bindings::{
        char_t,
        consts::UNMANAGED_CALLERS_ONLY_METHOD,
        hostfxr::{
            component_entry_point_fn, get_function_pointer_fn,
            load_assembly_and_get_function_pointer_fn,
        },
    },
    pdcstring::PdCStr,
    Error,
};

use std::{
    mem::{self, MaybeUninit},
    ptr,
};

use super::HostExitCode;

/// A function pointer for a method with the default signature.
pub type MethodWithDefaultSignature = component_entry_point_fn;
/// A function pointer for a method with an unknown signature.
pub type MethodWithUnknownSignature = *const ();

/// A struct for loading pointers to managed functions for a given [`HostfxrContext`].
///
/// [`HostfxrContext`]: super::HostfxrContext
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
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
    ) -> Result<MethodWithUnknownSignature, Error> {
        let mut delegate = MaybeUninit::uninit();

        let result = (self.get_load_assembly_and_get_function_pointer)(
            assembly_path,
            type_name,
            method_name,
            delegate_type_name,
            ptr::null(),
            delegate.as_mut_ptr(),
        );
        HostExitCode::from(result).to_result()?;

        Ok(delegate.assume_init())
    }

    unsafe fn _get_function_pointer(
        &self,
        type_name: *const char_t,
        method_name: *const char_t,
        delegate_type_name: *const char_t,
    ) -> Result<MethodWithUnknownSignature, Error> {
        let mut delegate = MaybeUninit::uninit();

        let result = (self.get_function_pointer)(
            type_name,
            method_name,
            delegate_type_name,
            ptr::null(),
            ptr::null(),
            delegate.as_mut_ptr(),
        );
        HostExitCode::from(result).to_result()?;

        Ok(delegate.assume_init())
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
    ) -> Result<MethodWithUnknownSignature, Error> {
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
    ) -> Result<MethodWithDefaultSignature, Error> {
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
    ) -> Result<MethodWithUnknownSignature, Error> {
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
    ) -> Result<MethodWithUnknownSignature, Error> {
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
    ) -> Result<MethodWithDefaultSignature, Error> {
        unsafe {
            let fn_ptr = self._get_function_pointer(
                type_name.as_ref().as_ptr(),
                method_name.as_ref().as_ptr(),
                ptr::null(),
            );
            mem::transmute(fn_ptr)
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
    ) -> Result<MethodWithUnknownSignature, Error> {
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
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct AssemblyDelegateLoader<A: AsRef<PdCStr>> {
    loader: DelegateLoader,
    assembly_path: A,
    assembly_loaded: bool,
}

impl<A: AsRef<PdCStr>> AssemblyDelegateLoader<A> {
    /// Creates a new [`AssemblyDelegateLoader`] wrapping the given [`DelegateLoader`] loading the assembly
    /// from the given path on the first access.
    pub fn new(loader: DelegateLoader, assembly_path: A) -> Self {
        Self {
            loader,
            assembly_path,
            assembly_loaded: false,
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
    ) -> Result<MethodWithUnknownSignature, Error> {
        if !self.assembly_loaded {
            self.loader.load_assembly_and_get_function_pointer(
                self.assembly_path.as_ref(),
                type_name,
                method_name,
                delegate_type_name,
            )
        } else {
            self.loader
                .get_function_pointer(type_name, method_name, delegate_type_name)
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
    ///     Name of the method on the `type_name` to find. The method must be static and must match the following signature:
    ///     `public delegate int ComponentEntryPoint(IntPtr args, int sizeBytes);`
    pub fn get_function_pointer_with_default_signature(
        &self,
        type_name: impl AsRef<PdCStr>,
        method_name: impl AsRef<PdCStr>,
    ) -> Result<MethodWithDefaultSignature, Error> {
        if !self.assembly_loaded {
            self.loader
                .load_assembly_and_get_function_pointer_with_default_signature(
                    self.assembly_path.as_ref(),
                    type_name,
                    method_name,
                )
        } else {
            self.loader
                .get_function_pointer_with_default_signature(type_name, method_name)
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
    ///     Name of the method on the `type_name` to find. The method must be static and must match be annotated with [`\[UnmanagedCallersOnly\]`].
    ///
    /// [`UnmanagedCallersOnlyAttribute`]: https://docs.microsoft.com/en-us/dotnet/api/system.runtime.interopservices.unmanagedcallersonlyattribute
    /// [`\[UnmanagedCallersOnly\]`]: https://docs.microsoft.com/en-us/dotnet/api/system.runtime.interopservices.unmanagedcallersonlyattribute
    pub fn get_function_pointer_for_unmanaged_callers_only_method(
        &self,
        type_name: impl AsRef<PdCStr>,
        method_name: impl AsRef<PdCStr>,
    ) -> Result<MethodWithUnknownSignature, Error> {
        if !self.assembly_loaded {
            self.loader
                .load_assembly_and_get_function_pointer_for_unmanaged_callers_only_method(
                    self.assembly_path.as_ref(),
                    type_name,
                    method_name,
                )
        } else {
            self.loader
                .get_function_pointer_for_unmanaged_callers_only_method(type_name, method_name)
        }
    }
}
