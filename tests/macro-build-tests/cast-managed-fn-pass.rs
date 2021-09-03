use std::{ffi::c_void, ptr};
use netcorehost::{cast_managed_fn, hostfxr::MethodWithUnknownSignature};

fn main() {
    let f: extern "system" fn() = cast_managed_fn!(ptr::null::<MethodWithUnknownSignature>(), fn());
    let f: extern "system" fn() -> i32 = cast_managed_fn!(ptr::null(), fn() -> i32);
    let f: extern "system" fn(f32) = cast_managed_fn!(ptr::null(), fn(f32));
    let f: extern "system" fn(*const c_void) -> *const c_void = cast_managed_fn!(ptr::null(), fn(*const c_void) -> *const c_void);
    let f: unsafe extern "system" fn(*const c_void) = cast_managed_fn!(ptr::null(), unsafe fn(*const c_void));
    let f: extern "system" fn(*const c_void) = cast_managed_fn!(ptr::null(), extern "system" fn(*const c_void));
    let f: unsafe extern "system" fn(*const c_void) = cast_managed_fn!(ptr::null(), unsafe extern "SyStEm" fn(*const c_void));
    let f: unsafe extern "system" fn(*const c_void) = cast_managed_fn!(ptr::null(), unsafe fn(*const c_void));
    let f: unsafe extern "system" fn(*const c_void) = cast_managed_fn!(ptr::null(), unsafe extern "system" fn(*const c_void));
}
