use std::path::Path;

use netcorehost::nethost;
use netcorehost::pdcstring::PdCString;
use path_absolutize::Absolutize;

#[path = "common.rs"]
mod common;

#[test]
fn runtime_properties() -> Result<(), Box<dyn std::error::Error>> {
    common::setup();

    let test_out_dir = Path::new("tests/Test/bin/Debug/net5.0").absolutize()?;
    let runtime_config_path = Path::join(&test_out_dir, "Test.runtimeconfig.json");

    let hostfxr = nethost::load_hostfxr()?;
    let context =
        hostfxr.initialize_for_runtime_config(&PdCString::from_os_str(runtime_config_path)?)?;

    let test_property_name = PdCString::from_str("TEST_PROPERTY")?;
    let test_property_value = PdCString::from_str("TEST_VALUE")?;
    context.set_runtime_property_value(&test_property_name, &test_property_value)?;
    let property_value = context.get_runtime_property_value_owned(&test_property_name)?;
    assert_eq!(test_property_value, property_value);

    let properties = context.get_runtime_properties_owned_as_map()?;
    let property_value = properties.get(&test_property_name).unwrap();
    assert_eq!(test_property_value, *property_value);

    Ok(())
}
