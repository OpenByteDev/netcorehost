use std::ptr;

use netcorehost::{nethost, pdcstr};

#[path = "common.rs"]
mod common;

#[test]
fn hello_world() -> Result<(), Box<dyn std::error::Error>> {
    common::setup();

    let hostfxr = nethost::load_hostfxr()?;

    let context = hostfxr.initialize_for_runtime_config(pdcstr!(
        "tests/Test/bin/Debug/net5.0/Test.runtimeconfig.json"
    ))?;
    let fn_loader = context
        .get_delegate_loader_for_assembly(pdcstr!("tests/Test/bin/Debug/net5.0/Test.dll"))?;
    let hello = fn_loader.get_function_pointer_with_default_signature(
        pdcstr!("Test.Program, Test"),
        pdcstr!("Hello"),
    )?;
    let result = unsafe { hello(ptr::null(), 0) };
    assert_eq!(result, 42);

    Ok(())
}

#[test]
fn hello_world_twice() -> Result<(), Box<dyn std::error::Error>> {
    common::setup();

    let hostfxr = nethost::load_hostfxr()?;

    let context = hostfxr.initialize_for_runtime_config(pdcstr!(
        "tests/Test/bin/Debug/net5.0/Test.runtimeconfig.json"
    ))?;
    let fn_loader = context
        .get_delegate_loader_for_assembly(pdcstr!("tests/Test/bin/Debug/net5.0/Test.dll"))?;

    let hello_one = fn_loader.get_function_pointer_with_default_signature(
        pdcstr!("Test.Program, Test"),
        pdcstr!("Hello"),
    )?;
    let result = unsafe { hello_one(ptr::null(), 0) };
    assert_eq!(result, 42);

    let hello_two = fn_loader.get_function_pointer_with_default_signature(
        pdcstr!("Test.Program, Test"),
        pdcstr!("Hello2"),
    )?;
    let result = unsafe { hello_two(ptr::null(), 0) };
    assert_eq!(result, 0);

    Ok(())
}
