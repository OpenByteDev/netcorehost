use crate::{
    bindings::hostfxr::{
        get_function_pointer_fn, hostfxr_delegate_type, load_assembly_and_get_function_pointer_fn,
    },
    pdcstring::{PdCStr, PdCString},
    Error,
};

use std::{
    collections::HashMap,
    iter::FromIterator,
    marker::PhantomData,
    mem::{self, MaybeUninit},
    ptr,
};

use super::{
    AssemblyDelegateLoader, DelegateLoader, HostExitCode, Hostfxr, HostfxrHandle,
    KnownHostExitCode, MethodWithUnknownSignature,
};

/// A marker struct indicating that the context was initialized with a runtime config.
/// This means that it is not possible to run the application associated with the context.
pub struct InitializedForRuntimeConfig;

/// A marker struct indicating that the context was initialized for the dotnet command line.
/// This means that it is possible to run the application associated with the context.
pub struct InitializedForCommandLine;

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

    pub fn get_runtime_property_value_owned(
        &self,
        name: impl AsRef<PdCStr>,
    ) -> Result<PdCString, Error> {
        unsafe { self.get_runtime_property_value_borrowed(name) }.map(|str| str.to_owned())
    }
    pub unsafe fn get_runtime_property_value_borrowed(
        &self,
        name: impl AsRef<PdCStr>,
    ) -> Result<&PdCStr, Error> {
        let mut value = MaybeUninit::uninit();

        let result = self.hostfxr.lib.hostfxr_get_runtime_property_value(
            self.handle.as_raw(),
            name.as_ref().as_ptr(),
            value.as_mut_ptr(),
        );
        HostExitCode::from(result).to_result()?;

        Ok(PdCStr::from_str_ptr(value.assume_init()))
    }

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

    pub unsafe fn get_runtime_properties_borrowed(
        &self,
    ) -> Result<(Vec<&PdCStr>, Vec<&PdCStr>), Error> {
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
    pub fn get_runtime_properties_owned(&self) -> Result<(Vec<PdCString>, Vec<PdCString>), Error> {
        unsafe { self.get_runtime_properties_borrowed() }.map(|(keys, values)| {
            let owned_keys = keys.into_iter().map(|key| key.to_owned()).collect();
            let owned_values = values.into_iter().map(|value| value.to_owned()).collect();
            (owned_keys, owned_values)
        })
    }
    pub fn collect_runtime_properties<T: FromIterator<(PdCString, PdCString)>>(
        &self,
    ) -> Result<T, Error> {
        self.get_runtime_properties_owned()
            .map(|(keys, values)| keys.into_iter().zip(values.into_iter()).collect())
    }
    pub unsafe fn get_runtime_properties_borrowed_as_map(
        &self,
    ) -> Result<HashMap<&PdCStr, &PdCStr>, Error> {
        self.get_runtime_properties_borrowed()
            .map(|(keys, values)| keys.into_iter().zip(values.into_iter()).collect())
    }
    pub fn get_runtime_properties_owned_as_map(
        &self,
    ) -> Result<HashMap<PdCString, PdCString>, Error> {
        self.collect_runtime_properties()
    }

    pub unsafe fn get_runtime_delegate(
        &self,
        r#type: hostfxr_delegate_type,
    ) -> Result<MethodWithUnknownSignature, Error> {
        let mut delegate = MaybeUninit::uninit();
        let result = self.hostfxr.lib.hostfxr_get_runtime_delegate(
            self.handle.as_raw(),
            r#type,
            delegate.as_mut_ptr(),
        );

        HostExitCode::from(result).to_result()?;

        Ok(delegate.assume_init())
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
    pub fn get_delegate_loader(&self) -> Result<DelegateLoader, Error> {
        Ok(DelegateLoader {
            get_load_assembly_and_get_function_pointer: self
                .get_load_assembly_and_get_function_pointer_delegate()?,
            get_function_pointer: self.get_get_function_pointer_delegate()?,
        })
    }
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
