use std::mem;

use netcorehost::{nethost, pdcstr};

#[path = "common.rs"]
mod common;

#[test]
fn unmanaged_caller_hello_world() -> Result<(), Box<dyn std::error::Error>> {
    common::setup();

    let hostfxr = nethost::load_hostfxr()?;

    let context = hostfxr.initialize_for_runtime_config(pdcstr!(
        "tests/Test/bin/Debug/net5.0/Test.runtimeconfig.json"
    ))?;
    let fn_loader = context
        .get_delegate_loader_for_assembly(pdcstr!("tests/Test/bin/Debug/net5.0/Test.dll"))?;
    let hello = fn_loader.get_function_pointer_for_unmanaged_callers_only_method(
        pdcstr!("Test.Program, Test"),
        pdcstr!("UnmanagedHello"),
    )?;
    let hello: extern "stdcall" fn() -> i32 = unsafe { mem::transmute(hello) };

    let result = hello();
    assert_eq!(result, 42);

    Ok(())
}
