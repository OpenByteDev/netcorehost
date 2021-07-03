use std::{mem, path::Path};

use netcorehost::nethost;
use netcorehost::pdcstring::PdCString;
use path_absolutize::Absolutize;

#[path = "common.rs"]
mod common;

#[test]
fn unmanaged_caller_hello_world() -> Result<(), Box<dyn std::error::Error>> {
    common::setup();

    let test_out_dir = Path::new("tests/Test/bin/Debug/net5.0").absolutize()?;
    let runtime_config_path = Path::join(&test_out_dir, "Test.runtimeconfig.json");
    let assembly_path = Path::join(&test_out_dir, "Test.dll");
    let type_name = "Test.Program, Test";
    let method_name = "UnmanagedHello";

    let hostfxr = nethost::load_hostfxr()?;

    let context =
        hostfxr.initialize_for_runtime_config(&PdCString::from_os_str(runtime_config_path)?)?;
    let fn_loader =
        context.get_delegate_loader_for_assembly(PdCString::from_os_str(assembly_path)?)?;
    let hello = fn_loader.get_function_pointer_for_unmanaged_callers_only_method(
        &PdCString::from_str(type_name)?,
        &PdCString::from_str(method_name)?,
    );
    let hello: extern "C" fn() -> i32 = unsafe { mem::transmute(hello) };

    let result = hello();
    assert_eq!(result, 42);

    Ok(())
}
