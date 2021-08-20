use std::{path::Path, ptr};

use netcorehost::pdcstring::PdCString;
use netcorehost::{nethost, pdcstr};
use path_absolutize::Absolutize;

#[path = "common.rs"]
mod common;

#[test]
fn hello_world() -> Result<(), Box<dyn std::error::Error>> {
    common::setup();

    let test_out_dir = Path::new("tests/Test/bin/Debug/net5.0").absolutize()?;
    let runtime_config_path = Path::join(&test_out_dir, "Test.runtimeconfig.json");
    let assembly_path = Path::join(&test_out_dir, "Test.dll");

    let hostfxr = nethost::load_hostfxr()?;

    let context =
        hostfxr.initialize_for_runtime_config(PdCString::from_os_str(runtime_config_path)?)?;
    let fn_loader =
        context.get_delegate_loader_for_assembly(PdCString::from_os_str(assembly_path)?)?;
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

    let test_out_dir = Path::new("tests/Test/bin/Debug/net5.0").absolutize()?;
    let runtime_config_path = Path::join(&test_out_dir, "Test.runtimeconfig.json");
    let assembly_path = Path::join(&test_out_dir, "Test.dll");

    let hostfxr = nethost::load_hostfxr()?;

    let context =
        hostfxr.initialize_for_runtime_config(PdCString::from_os_str(runtime_config_path)?)?;
    let fn_loader =
        context.get_delegate_loader_for_assembly(PdCString::from_os_str(assembly_path)?)?;

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
