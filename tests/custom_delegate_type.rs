use std::mem;
use std::{path::Path, ptr};

use netcorehost::pdcstring::PdCString;
use netcorehost::{nethost, pdcstr};
use path_absolutize::Absolutize;

#[path = "common.rs"]
mod common;

#[test]
fn hello_world_with_custom_delegate_type() -> Result<(), Box<dyn std::error::Error>> {
    common::setup();

    let test_out_dir = Path::new("tests/Test/bin/Debug/net5.0").absolutize()?;
    let runtime_config_path = Path::join(&test_out_dir, "Test.runtimeconfig.json");
    let assembly_path = Path::join(&test_out_dir, "Test.dll");

    let hostfxr = nethost::load_hostfxr()?;

    let context =
        hostfxr.initialize_for_runtime_config(PdCString::from_os_str(runtime_config_path)?)?;
    let fn_loader =
        context.get_delegate_loader_for_assembly(PdCString::from_os_str(assembly_path)?)?;
    let hello = fn_loader.get_function_pointer(
        pdcstr!("Test.Program, Test"),
        pdcstr!("Hello"),
        pdcstr!(
            "Test.Program+HelloFunc, Test, Version=1.0.0.0, Culture=neutral, PublicKeyToken=null"
        ),
    )?;
    let hello: extern "stdcall" fn(*const (), i32) -> i32 = unsafe { mem::transmute(hello) };
    let result = hello(ptr::null(), 0);
    assert_eq!(result, 42);

    Ok(())
}
