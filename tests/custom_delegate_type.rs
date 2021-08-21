use std::{mem, ptr};

use netcorehost::{nethost, pdcstr};

#[path = "common.rs"]
mod common;

#[test]
fn hello_world_with_custom_delegate_type() {
    common::setup();

    let hostfxr = nethost::load_hostfxr().unwrap();

    let context = hostfxr.initialize_for_runtime_config(pdcstr!(
        "tests/Test/bin/Debug/net5.0/Test.runtimeconfig.json"
    )).unwrap();
    let fn_loader = context
        .get_delegate_loader_for_assembly(pdcstr!("tests/Test/bin/Debug/net5.0/Test.dll")).unwrap();
    let hello = fn_loader.get_function_pointer(
        pdcstr!("Test.Program, Test"),
        pdcstr!("Hello"),
        pdcstr!("Test.Program+HelloFunc, Test"),
    ).unwrap();
    let hello: extern "stdcall" fn(*const (), i32) -> i32 = unsafe { mem::transmute(hello) };
    let result = hello(ptr::null(), 0);
    assert_eq!(result, 42);
}
