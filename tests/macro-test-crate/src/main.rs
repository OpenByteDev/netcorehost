fn main() {
    let _ = netcorehost::pdcstr!("test");
    let _ = unsafe { netcorehost::cast_managed_fn!(std::ptr::null(), fn(*const core::ffi::c_void, i32) -> i32) };
}
