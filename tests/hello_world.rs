use std::{path::Path, ptr};

use path_absolutize::Absolutize;
use widestring::WideCString;

#[test]
fn hello_world() -> Result<(), Box<dyn std::error::Error>> {
    let test_out_dir = Path::new("tests\\Test\\bin\\Debug\\net5.0").absolutize()?;
    let runtime_config_path = Path::join(&test_out_dir, "Test.runtimeconfig.json");
    let assembly_path = Path::join(&test_out_dir, "Test.dll");
    let type_name = "Test.Program, Test";
    let method_name = "Hello";

    let hostfxr = netcorehost::nethost::load_hostfxr()?;
    let context = hostfxr.initialize_for_runtime_config(&WideCString::from_os_str(
        runtime_config_path.as_os_str(),
    )?)?;
    let fn_loader = context
        .get_delegate_loader_for_assembly(WideCString::from_os_str(assembly_path.as_os_str())?)?;
    let hello = fn_loader.get_function_pointer_with_default_signature(
        &WideCString::from_str(type_name.to_owned())?,
        &WideCString::from_str(method_name.to_owned())?,
    );
    let result = unsafe { hello(ptr::null(), 0) };
    assert_eq!(result, 42);

    Ok(())
}
