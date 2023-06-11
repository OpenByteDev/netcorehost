use std::{collections::HashMap, mem::MaybeUninit, ptr};

use crate::{
    error::{HostingError, HostingResult},
    pdcstring::PdCStr,
};

use super::HostfxrContext;

impl<I> HostfxrContext<I> {
    /// Gets the runtime property value for the given key of this host context.
    pub fn get_runtime_property_value(
        &self,
        name: impl AsRef<PdCStr>,
    ) -> Result<&'_ PdCStr, HostingError> {
        let mut value = MaybeUninit::uninit();

        let result = unsafe {
            self.library().hostfxr_get_runtime_property_value(
                self.handle().as_raw(),
                name.as_ref().as_ptr(),
                value.as_mut_ptr(),
            )
        };
        HostingResult::from(result).into_result()?;

        Ok(unsafe { PdCStr::from_str_ptr(value.assume_init()) })
    }

    /// Sets the value of a runtime property for this host context.
    pub fn set_runtime_property_value(
        &mut self,
        name: impl AsRef<PdCStr>,
        value: impl AsRef<PdCStr>,
    ) -> Result<(), HostingError> {
        let result = unsafe {
            self.library().hostfxr_set_runtime_property_value(
                self.handle().as_raw(),
                name.as_ref().as_ptr(),
                value.as_ref().as_ptr(),
            )
        };
        HostingResult::from(result).into_result().map(|_| ())
    }

    /// Remove a runtime property for this host context.
    pub fn remove_runtime_property_value(
        &mut self,
        name: impl AsRef<PdCStr>,
    ) -> Result<(), HostingError> {
        let result = unsafe {
            self.library().hostfxr_set_runtime_property_value(
                self.handle().as_raw(),
                name.as_ref().as_ptr(),
                ptr::null(),
            )
        };
        HostingResult::from(result).into_result().map(|_| ())
    }

    /// Get all runtime properties for this host context.
    pub fn runtime_properties(&self) -> Result<HashMap<&'_ PdCStr, &'_ PdCStr>, HostingError> {
        // get count
        let mut count = MaybeUninit::uninit();
        let mut result = unsafe {
            self.library().hostfxr_get_runtime_properties(
                self.handle().as_raw(),
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
            self.library().hostfxr_get_runtime_properties(
                self.handle().as_raw(),
                &mut count,
                keys.as_mut_ptr(),
                values.as_mut_ptr(),
            )
        };
        HostingResult::from(result).into_result()?;

        unsafe { keys.set_len(count) };
        unsafe { values.set_len(count) };

        let keys = keys.into_iter().map(|e| unsafe { PdCStr::from_str_ptr(e) });
        let values = values
            .into_iter()
            .map(|e| unsafe { PdCStr::from_str_ptr(e) });

        let map = keys.zip(values).collect();
        Ok(map)
    }
}
