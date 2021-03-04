use crate::{
    bindings::{
        hostfxr::{
            component_entry_point_fn, get_function_pointer_fn, hostfxr_delegate_type,
            hostfxr_handle, hostfxr_initialize_parameters,
            load_assembly_and_get_function_pointer_fn, HostfxrLib,
        },
        type_aliases::char_t,
    },
    error::Error,
    error::{HostExitCode, KnownHostExitCode},
};
use dlopen::wrapper::Container;
use std::{
    collections::HashMap,
    ffi::OsStr,
    iter::FromIterator,
    mem::{self, MaybeUninit},
    ptr,
};
use widestring::{WideCStr, WideCString};

pub struct Hostfxr {
    lib: Container<HostfxrLib>,
}

impl Hostfxr {
    pub fn load_from_path<T: AsRef<OsStr>>(path: T) -> Result<Self, Error> {
        Ok(Self {
            lib: unsafe { Container::load(path)? },
        })
    }

    pub fn initialize_for_dotnet_command_line(
        &self,
        args: &[&WideCStr],
    ) -> Result<HostfxrContext, Error> {
        unsafe { self.initialize_for_dotnet_command_line_with_parameters(args, ptr::null()) }
    }
    pub fn initialize_for_dotnet_command_line_with_host_path<H: AsRef<WideCStr>>(
        &self,
        args: &[&WideCStr],
        host_path: H,
    ) -> Result<HostfxrContext, Error> {
        let parameters = hostfxr_initialize_parameters::with_host_path(host_path.as_ref().as_ptr());
        unsafe { self.initialize_for_dotnet_command_line_with_parameters(args, &parameters) }
    }
    pub fn initialize_for_dotnet_command_line_with_dotnet_root<R: AsRef<WideCStr>>(
        &self,
        args: &[&WideCStr],
        dotnet_root: R,
    ) -> Result<HostfxrContext, Error> {
        let parameters =
            hostfxr_initialize_parameters::with_dotnet_root(dotnet_root.as_ref().as_ptr());
        unsafe { self.initialize_for_dotnet_command_line_with_parameters(args, &parameters) }
    }
    unsafe fn initialize_for_dotnet_command_line_with_parameters(
        &self,
        args: &[&WideCStr],
        parameters: *const hostfxr_initialize_parameters,
    ) -> Result<HostfxrContext, Error> {
        let mut hostfxr_handle = MaybeUninit::<hostfxr_handle>::uninit();

        let result = self.lib.hostfxr_initialize_for_dotnet_command_line(
            args.len() as i32,
            args.as_ptr() as *const *const char_t,
            parameters,
            hostfxr_handle.as_mut_ptr(),
        );

        HostExitCode::from(result).to_result()?;

        Ok(HostfxrContext::new(hostfxr_handle.assume_init(), self))
    }

    pub fn initialize_for_runtime_config<P: AsRef<WideCStr>>(
        &self,
        runtime_config_path: P,
    ) -> Result<HostfxrContext, Error> {
        unsafe {
            self.initialize_for_runtime_config_with_parameters(runtime_config_path, ptr::null())
        }
    }
    pub fn initialize_for_runtime_config_with_host_path<P: AsRef<WideCStr>, H: AsRef<WideCStr>>(
        &self,
        runtime_config_path: P,
        host_path: H,
    ) -> Result<HostfxrContext, Error> {
        let parameters = hostfxr_initialize_parameters::with_host_path(host_path.as_ref().as_ptr());
        unsafe {
            self.initialize_for_runtime_config_with_parameters(runtime_config_path, &parameters)
        }
    }
    pub fn initialize_for_runtime_config_with_dotnet_root<
        P: AsRef<WideCStr>,
        R: AsRef<WideCStr>,
    >(
        &self,
        runtime_config_path: P,
        dotnet_root: R,
    ) -> Result<HostfxrContext, Error> {
        let parameters =
            hostfxr_initialize_parameters::with_dotnet_root(dotnet_root.as_ref().as_ptr());
        unsafe {
            self.initialize_for_runtime_config_with_parameters(runtime_config_path, &parameters)
        }
    }
    unsafe fn initialize_for_runtime_config_with_parameters<P: AsRef<WideCStr>>(
        &self,
        runtime_config_path: P,
        parameters: *const hostfxr_initialize_parameters,
    ) -> Result<HostfxrContext, Error> {
        let mut hostfxr_handle = MaybeUninit::uninit();

        let result = self.lib.hostfxr_initialize_for_runtime_config(
            runtime_config_path.as_ref().as_ptr(),
            parameters,
            hostfxr_handle.as_mut_ptr(),
        );

        HostExitCode::from(result).to_result()?;

        Ok(HostfxrContext::new(hostfxr_handle.assume_init(), self))
    }
}

pub struct HostfxrContext<'a> {
    handle: hostfxr_handle,
    hostfxr: &'a Hostfxr,
}

impl<'a> HostfxrContext<'a> {
    fn new(handle: hostfxr_handle, hostfxr: &'a Hostfxr) -> Self {
        Self { handle, hostfxr }
    }

    pub fn get_runtime_property_value_owned<N: AsRef<WideCStr>>(
        &self,
        name: N,
    ) -> Result<WideCString, Error> {
        unsafe { self.get_runtime_property_value_borrowed(name) }.map(|str| str.to_owned())
    }
    pub unsafe fn get_runtime_property_value_borrowed<N: AsRef<WideCStr>>(
        &self,
        name: N,
    ) -> Result<&WideCStr, Error> {
        let mut value = MaybeUninit::uninit();

        let result = self.hostfxr.lib.hostfxr_get_runtime_property_value(
            self.handle,
            name.as_ref().as_ptr(),
            value.as_mut_ptr(),
        );
        HostExitCode::from(result).to_result()?;

        Ok(WideCStr::from_ptr_str(value.assume_init()))
    }

    pub fn set_runtime_property_value<N: AsRef<WideCStr>, V: AsRef<WideCStr>>(
        &self,
        name: N,
        value: V,
    ) -> Result<(), Error> {
        let result = unsafe {
            self.hostfxr.lib.hostfxr_set_runtime_property_value(
                self.handle,
                name.as_ref().as_ptr(),
                value.as_ref().as_ptr(),
            )
        };
        HostExitCode::from(result).to_result().map(|_| ())
    }

    pub unsafe fn get_runtime_properties_borrowed(
        &self,
    ) -> Result<(Vec<&WideCStr>, Vec<&WideCStr>), Error> {
        // get count
        let mut count = MaybeUninit::uninit();
        let result = self.hostfxr.lib.hostfxr_get_runtime_properties(
            self.handle,
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
            self.handle,
            &mut count,
            keys.as_mut_ptr(),
            values.as_mut_ptr(),
        );
        HostExitCode::from(result).to_result()?;

        keys.set_len(count);
        values.set_len(count);

        let keys = keys
            .into_iter()
            .map(|e| WideCStr::from_ptr_str(e))
            .collect();
        let values = values
            .into_iter()
            .map(|e| WideCStr::from_ptr_str(e))
            .collect();

        Ok((keys, values))
    }
    pub fn get_runtime_properties_owned(
        &self,
    ) -> Result<(Vec<WideCString>, Vec<WideCString>), Error> {
        unsafe { self.get_runtime_properties_borrowed() }.map(|(keys, values)| {
            let owned_keys = keys.into_iter().map(|key| key.to_owned()).collect();
            let owned_values = values.into_iter().map(|value| value.to_owned()).collect();
            (owned_keys, owned_values)
        })
    }
    pub fn get_runtime_properties_collected<T: FromIterator<(WideCString, WideCString)>>(
        &self,
    ) -> Result<T, Error> {
        self.get_runtime_properties_owned()
            .map(|(keys, values)| keys.into_iter().zip(values.into_iter()).collect())
    }
    pub unsafe fn get_runtime_properties_as_map_borrowed(
        &self,
    ) -> Result<HashMap<&WideCStr, &WideCStr>, Error> {
        self.get_runtime_properties_borrowed()
            .map(|(keys, values)| keys.into_iter().zip(values.into_iter()).collect())
    }
    pub fn get_runtime_properties_as_map_owned(
        &self,
    ) -> Result<HashMap<WideCString, WideCString>, Error> {
        self.get_runtime_properties_collected()
    }

    pub fn run_app(&self) -> HostExitCode {
        let result = unsafe { self.hostfxr.lib.hostfxr_run_app(self.handle) };
        HostExitCode::from(result)
    }

    pub unsafe fn get_runtime_delegate(
        &self,
        r#type: hostfxr_delegate_type,
    ) -> Result<*const (), Error> {
        let mut delegate = MaybeUninit::uninit();
        let result = self.hostfxr.lib.hostfxr_get_runtime_delegate(
            self.handle,
            r#type,
            delegate.as_mut_ptr(),
        );

        HostExitCode::from(result).to_result()?;

        Ok(delegate.assume_init())
    }
    pub fn get_load_assembly_and_get_function_pointer_delegate(
        &self,
    ) -> Result<load_assembly_and_get_function_pointer_fn, Error> {
        unsafe {
            self.get_runtime_delegate(
                hostfxr_delegate_type::hdt_load_assembly_and_get_function_pointer,
            )
            .map(|ptr| mem::transmute(ptr))
        }
    }
    pub fn get_get_function_pointer_delegate(&self) -> Result<get_function_pointer_fn, Error> {
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
    pub fn get_delegate_loader_for_assembly<A: AsRef<WideCStr>>(
        &self,
        assembly_path: A,
    ) -> Result<AssemblyDelegateLoader<A>, Error> {
        self.get_delegate_loader()
            .map(|loader| AssemblyDelegateLoader::new(loader, assembly_path))
    }

    unsafe fn close(&self) -> Result<(), Error> {
        self.hostfxr.lib.hostfxr_close(self.handle);
        Ok(())
    }
}

impl Drop for HostfxrContext<'_> {
    fn drop(&mut self) {
        let _ = unsafe { self.close() };
    }
}

pub struct DelegateLoader {
    get_load_assembly_and_get_function_pointer: load_assembly_and_get_function_pointer_fn,
    get_function_pointer: get_function_pointer_fn,
}

impl DelegateLoader {
    pub fn load_assembly_and_get_function_pointer<
        A: AsRef<WideCStr>,
        T: AsRef<WideCStr>,
        M: AsRef<WideCStr>,
    >(
        &self,
        assembly_path: A,
        type_name: T,
        method_name: M,
        delegate_type_name: Option<&WideCStr>,
    ) -> *const () {
        unsafe {
            let mut delegate = MaybeUninit::uninit();
            (self.get_load_assembly_and_get_function_pointer)(
                assembly_path.as_ref().as_ptr(),
                type_name.as_ref().as_ptr(),
                method_name.as_ref().as_ptr(),
                delegate_type_name.map_or(ptr::null(), |e| e.as_ptr()),
                ptr::null(),
                delegate.as_mut_ptr(),
            );
            delegate.assume_init()
        }
    }

    pub fn load_assembly_and_get_function_pointer_with_default_signature<
        A: AsRef<WideCStr>,
        T: AsRef<WideCStr>,
        M: AsRef<WideCStr>,
    >(
        &self,
        assembly_path: A,
        type_name: T,
        method_name: M,
    ) -> component_entry_point_fn {
        let fn_ptr = self.load_assembly_and_get_function_pointer(
            assembly_path.as_ref(),
            type_name.as_ref(),
            method_name.as_ref(),
            None,
        );
        unsafe { mem::transmute(fn_ptr) }
    }

    pub fn get_function_pointer<T: AsRef<WideCStr>, M: AsRef<WideCStr>>(
        &self,
        type_name: T,
        method_name: M,
        delegate_type_name: Option<&WideCStr>,
    ) -> *const () {
        unsafe {
            let mut delegate = MaybeUninit::uninit();
            (self.get_function_pointer)(
                type_name.as_ref().as_ptr(),
                method_name.as_ref().as_ptr(),
                delegate_type_name.map_or(ptr::null(), |e| e.as_ptr()),
                ptr::null(),
                ptr::null(),
                delegate.as_mut_ptr(),
            );
            delegate.assume_init()
        }
    }

    pub fn get_function_pointer_with_default_signature<T: AsRef<WideCStr>, M: AsRef<WideCStr>>(
        &self,
        type_name: T,
        method_name: M,
    ) -> component_entry_point_fn {
        let fn_ptr = self.get_function_pointer(type_name, method_name, None);
        unsafe { mem::transmute(fn_ptr) }
    }
}

pub struct AssemblyDelegateLoader<A: AsRef<WideCStr>> {
    loader: DelegateLoader,
    assembly_path: A,
    assembly_loaded: bool,
}

impl<A: AsRef<WideCStr>> AssemblyDelegateLoader<A> {
    pub fn new(loader: DelegateLoader, assembly_path: A) -> Self {
        Self {
            loader,
            assembly_path,
            assembly_loaded: false,
        }
    }

    pub fn get_function_pointer<T: AsRef<WideCStr>, M: AsRef<WideCStr>>(
        &self,
        type_name: T,
        method_name: M,
        delegate_type_name: Option<&WideCStr>,
    ) -> *const () {
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

    pub fn get_function_pointer_with_default_signature<T: AsRef<WideCStr>, M: AsRef<WideCStr>>(
        &self,
        type_name: T,
        method_name: M,
    ) -> component_entry_point_fn {
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
}
