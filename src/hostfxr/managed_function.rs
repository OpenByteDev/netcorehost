use std::{fmt::Debug, ops::Deref};

pub use fn_ptr::{FnPtr, UntypedFnPtr as RawFnPtr, abi};

/// A wrapper around a managed function pointer.
pub struct ManagedFunction<F: ManagedFnPtr>(pub(crate) F);

impl<F: ManagedFnPtr> Debug for ManagedFunction<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ManagedFunction")
            .field("ptr", &self.0.as_ptr())
            .field("sig", &std::any::type_name::<F>())
            .finish()
    }
}

impl<F: ManagedFnPtr> Deref for ManagedFunction<F> {
    type Target = F;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Trait representing a managed function pointer.
pub trait ManagedFnPtr: FnPtr<Abi = abi!("system")> {}
impl<T: FnPtr<Abi = abi!("system")>> ManagedFnPtr for T {}
