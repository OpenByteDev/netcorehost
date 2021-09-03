fn main() {
    // wrong ptr type
    let _ = netcorehost::cast_managed_fn!("\0", fn());
    let _ = netcorehost::cast_managed_fn!(netcorehost::hostfxr::MethodWithDefaultSignature, fn());
    let _ = netcorehost::cast_managed_fn!(*const (), fn());

    // with invalid extern
    let _ = netcorehost::cast_managed_fn!(ptr::null(), extern "C" fn());
    let _ = netcorehost::cast_managed_fn!(ptr::null(), extern "rust" fn());
    let _ = netcorehost::cast_managed_fn!(ptr::null(), extern fn());
}
