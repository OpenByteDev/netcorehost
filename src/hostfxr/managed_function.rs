use std::{ops::Deref, rc::Rc};

use super::HostfxrLibrary;

/// A wrapper around a managed function pointer.
pub struct ManagedFunction<F: ManagedFunctionPtr>(pub(crate) F, pub(crate) Rc<HostfxrLibrary>);

impl<F: ManagedFunctionPtr> Deref for ManagedFunction<F> {
    type Target = F;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

ffi_opaque::opaque! {
    /// A struct representing an opaque function.
    pub struct OpaqueFunction;
}

/// Type alias for a raw untyped function pointer.
pub type RawFunctionPtr = *const OpaqueFunction;

/// Trait representing a function pointer.
///
/// # Safety
/// This trait should only be implemented for function pointers and the associated types and constants have to match the function pointer type.
pub unsafe trait FunctionPtr: Sized + Copy + Send + Sync + 'static {
    /// The argument types as a tuple.
    type Args;

    /// The return type.
    type Output;

    /// The function's arity (number of arguments).
    const ARITY: usize;

    /// The `extern "system"` version of this function pointer.
    type Managed: ManagedFunctionPtr;

    /// Constructs a [`FunctionPtr`] from an untyped function pointer.
    ///
    /// # Safety
    /// This function is unsafe because it can not check if the argument points to a function
    /// of the correct type.
    unsafe fn from_ptr(ptr: RawFunctionPtr) -> Self;

    /// Returns a untyped function pointer for this function.
    fn as_ptr(&self) -> RawFunctionPtr;
}

/// Trait representing a managed function pointer.
///
/// # Safety
/// This trait should only be implemented for `extern "system"` function pointers and the associated types and constants have to match the function pointer type.
pub unsafe trait ManagedFunctionPtr: FunctionPtr {
    /// The argument types as a tuple.
    type Args;

    /// The return type.
    type Output;

    /// The function's arity (number of arguments).
    const ARITY: usize;
}

macro_rules! impl_fn {
    (@recurse () ($($nm:ident : $ty:ident),*)) => {
        impl_fn!(@impl_all ($($nm : $ty),*));
    };
    (@recurse ($hd_nm:ident : $hd_ty:ident $(, $tl_nm:ident : $tl_ty:ident)*) ($($nm:ident : $ty:ident),*)) => {
        impl_fn!(@impl_all ($($nm : $ty),*));
        impl_fn!(@recurse ($($tl_nm : $tl_ty),*) ($($nm : $ty,)* $hd_nm : $hd_ty));
    };

    (@impl_all ($($nm:ident : $ty:ident),*)) => {
        impl_fn!(@impl_u_and_s ($($nm : $ty),*) fn($($ty),*) -> Ret);
    };

    (@impl_u_and_s ($($nm:ident : $ty:ident),*) fn($($param_ty:ident),*) -> $ret:ty) => {
        impl_fn!(@impl_core ($($nm : $ty),*) (fn($($param_ty),*) -> $ret) (extern "system" fn($($param_ty),*) -> $ret));
        impl_fn!(@impl_core ($($nm : $ty),*) (unsafe fn($($param_ty),*) -> $ret) (unsafe extern "system" fn($($param_ty),*) -> $ret));
    };

    (@impl_core ($($nm:ident : $ty:ident),*) ($fn_type:ty) ($managed_fn_type:ty)) => {
        unsafe impl<Ret: 'static, $($ty: 'static),*> crate::hostfxr::FunctionPtr for $fn_type {
            type Args = ($($ty,)*);
            type Output = Ret;
            type Managed = $managed_fn_type;

            const ARITY: ::core::primitive::usize = impl_fn!(@count ($($ty)*));

            unsafe fn from_ptr(ptr: crate::hostfxr::RawFunctionPtr) -> Self {
                ::core::assert!(!ptr.is_null());
                unsafe { ::core::mem::transmute(ptr) }
            }

            fn as_ptr(&self) -> crate::hostfxr::RawFunctionPtr {
                *self as crate::hostfxr::RawFunctionPtr
            }
        }

        unsafe impl<Ret: 'static, $($ty: 'static),*> crate::hostfxr::FunctionPtr for $managed_fn_type {
            type Args = ($($ty,)*);
            type Output = Ret;
            type Managed = $managed_fn_type;

            const ARITY: ::core::primitive::usize = impl_fn!(@count ($($ty)*));

            unsafe fn from_ptr(ptr: crate::hostfxr::RawFunctionPtr) -> Self {
                ::core::assert!(!ptr.is_null());
                unsafe { ::core::mem::transmute(ptr) }
            }

            fn as_ptr(&self) -> crate::hostfxr::RawFunctionPtr {
                *self as crate::hostfxr::RawFunctionPtr
            }
        }

        unsafe impl<Ret: 'static, $($ty: 'static),*> crate::hostfxr::ManagedFunctionPtr for $managed_fn_type {
            type Args = ($($ty,)*);
            type Output = Ret;

            const ARITY: ::core::primitive::usize = impl_fn!(@count ($($ty)*));
        }
    };

    (@count ()) => {
        0
    };
    (@count ($hd:tt $($tl:tt)*)) => {
        1 + impl_fn!(@count ($($tl)*))
    };

    ($($nm:ident : $ty:ident),*) => {
        impl_fn!(@recurse ($($nm : $ty),*) ());
    };
}

impl_fn! {
    __arg_0:  A, __arg_1:  B, __arg_2:  C, __arg_3:  D, __arg_4:  E, __arg_5:  F, __arg_6:  G,
    __arg_7:  H, __arg_8:  I, __arg_9:  J, __arg_10: K, __arg_11: L
}
