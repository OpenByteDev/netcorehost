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
};

use std::{
    mem::{self, MaybeUninit},
    ptr,
};

pub type MethodWithDefaultSignature = component_entry_point_fn;
pub type MethodWithUnknownSignature = *const ();

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
    ) -> MethodWithUnknownSignature {
        let mut delegate = MaybeUninit::uninit();
        (self.get_load_assembly_and_get_function_pointer)(
            assembly_path,
            type_name,
            method_name,
            delegate_type_name,
            ptr::null(),
            delegate.as_mut_ptr(),
        );
        delegate.assume_init()
    }

    unsafe fn _get_function_pointer(
        &self,
        type_name: *const char_t,
        method_name: *const char_t,
        delegate_type_name: *const char_t,
    ) -> MethodWithUnknownSignature {
        let mut delegate = MaybeUninit::uninit();
        (self.get_function_pointer)(
            type_name,
            method_name,
            delegate_type_name,
            ptr::null(),
            ptr::null(),
            delegate.as_mut_ptr(),
        );
        delegate.assume_init()
    }

    pub fn load_assembly_and_get_function_pointer(
        &self,
        assembly_path: impl AsRef<PdCStr>,
        type_name: impl AsRef<PdCStr>,
        method_name: impl AsRef<PdCStr>,
        delegate_type_name: impl AsRef<PdCStr>,
    ) -> MethodWithUnknownSignature {
        unsafe {
            self._load_assembly_and_get_function_pointer(
                assembly_path.as_ref().as_ptr(),
                type_name.as_ref().as_ptr(),
                method_name.as_ref().as_ptr(),
                delegate_type_name.as_ref().as_ptr(),
            )
        }
    }

    pub fn load_assembly_and_get_function_pointer_with_default_signature(
        &self,
        assembly_path: impl AsRef<PdCStr>,
        type_name: impl AsRef<PdCStr>,
        method_name: impl AsRef<PdCStr>,
    ) -> MethodWithDefaultSignature {
        unsafe {
            let fn_ptr = self._load_assembly_and_get_function_pointer(
                assembly_path.as_ref().as_ptr(),
                type_name.as_ref().as_ptr(),
                method_name.as_ref().as_ptr(),
                ptr::null(),
            );
            mem::transmute(fn_ptr)
        }
    }

    pub fn load_assembly_and_get_function_pointer_for_unmanaged_callers_only_method(
        &self,
        assembly_path: impl AsRef<PdCStr>,
        type_name: impl AsRef<PdCStr>,
        method_name: impl AsRef<PdCStr>,
    ) -> MethodWithUnknownSignature {
        unsafe {
            self._load_assembly_and_get_function_pointer(
                assembly_path.as_ref().as_ptr(),
                type_name.as_ref().as_ptr(),
                method_name.as_ref().as_ptr(),
                UNMANAGED_CALLERS_ONLY_METHOD,
            )
        }
    }

    pub fn get_function_pointer(
        &self,
        type_name: impl AsRef<PdCStr>,
        method_name: impl AsRef<PdCStr>,
        delegate_type_name: impl AsRef<PdCStr>,
    ) -> MethodWithUnknownSignature {
        unsafe {
            self._get_function_pointer(
                type_name.as_ref().as_ptr(),
                method_name.as_ref().as_ptr(),
                delegate_type_name.as_ref().as_ptr(),
            )
        }
    }

    pub fn get_function_pointer_with_default_signature(
        &self,
        type_name: impl AsRef<PdCStr>,
        method_name: impl AsRef<PdCStr>,
    ) -> MethodWithDefaultSignature {
        unsafe {
            let fn_ptr = self._get_function_pointer(
                type_name.as_ref().as_ptr(),
                method_name.as_ref().as_ptr(),
                ptr::null(),
            );
            mem::transmute(fn_ptr)
        }
    }

    pub fn get_function_pointer_for_unmanaged_callers_only_method(
        &self,
        type_name: impl AsRef<PdCStr>,
        method_name: impl AsRef<PdCStr>,
    ) -> MethodWithUnknownSignature {
        unsafe {
            self._get_function_pointer(
                type_name.as_ref().as_ptr(),
                method_name.as_ref().as_ptr(),
                UNMANAGED_CALLERS_ONLY_METHOD,
            )
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct AssemblyDelegateLoader<A: AsRef<PdCStr>> {
    loader: DelegateLoader,
    assembly_path: A,
    assembly_loaded: bool,
}

impl<A: AsRef<PdCStr>> AssemblyDelegateLoader<A> {
    pub fn new(loader: DelegateLoader, assembly_path: A) -> Self {
        Self {
            loader,
            assembly_path,
            assembly_loaded: false,
        }
    }

    pub fn get_function_pointer(
        &self,
        type_name: impl AsRef<PdCStr>,
        method_name: impl AsRef<PdCStr>,
        delegate_type_name: impl AsRef<PdCStr>,
    ) -> MethodWithUnknownSignature {
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

    pub fn get_function_pointer_with_default_signature(
        &self,
        type_name: impl AsRef<PdCStr>,
        method_name: impl AsRef<PdCStr>,
    ) -> MethodWithDefaultSignature {
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

    pub fn get_function_pointer_for_unmanaged_callers_only_method(
        &self,
        type_name: impl AsRef<PdCStr>,
        method_name: impl AsRef<PdCStr>,
    ) -> MethodWithUnknownSignature {
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
