use std::str::FromStr;
use std::{path::Path, ptr};

use netcorehost::nethost;
use netcorehost::pdcstring::PdCString;
use path_absolutize::Absolutize;

#[path = "common.rs"]
mod common;

#[test]
fn hello_world() -> Result<(), Box<dyn std::error::Error>> {
    common::setup();

    let test_out_dir = Path::new("tests/Test/bin/Debug/net5.0").absolutize()?;
    let runtime_config_path = Path::join(&test_out_dir, "Test.runtimeconfig.json");
    let assembly_path = Path::join(&test_out_dir, "Test.dll");
    let type_name = "Test.Program, Test";
    let method_name = "Hello";

    let hostfxr = nethost::load_hostfxr()?;

    let context =
        hostfxr.initialize_for_runtime_config(PdCString::from_os_str(runtime_config_path)?)?;
    let mut fn_loader =
        context.get_delegate_loader_for_assembly(PdCString::from_os_str(assembly_path)?)?;
    let hello = fn_loader.get_function_pointer_with_default_signature(
        PdCString::from_str(type_name)?,
        PdCString::from_str(method_name)?,
    )?;
    let result = unsafe { hello(ptr::null(), 0) };
    assert_eq!(result, 42);

    Ok(())
}
