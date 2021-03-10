use std::path::Path;

use path_absolutize::Absolutize;
use widestring::WideCString;

#[test]
fn runtime_properties() -> Result<(), Box<dyn std::error::Error>> {
    let test_out_dir = Path::new("tests\\Test\\bin\\Debug\\net5.0").absolutize()?;
    let runtime_config_path = Path::join(&test_out_dir, "Test.runtimeconfig.json");

    let hostfxr = netcorehost::nethost::load_hostfxr()?;
    let context = hostfxr.initialize_for_runtime_config(&WideCString::from_os_str(
        runtime_config_path.as_os_str(),
    )?)?;

    let test_property_name = WideCString::from_str("TEST_PROPERTY")?;
    let test_property_value = WideCString::from_str("TEST_VALUE")?;
    context.set_runtime_property_value(&test_property_name, &test_property_value)?;
    let property_value = context.get_runtime_property_value_owned(&test_property_name)?;
    assert_eq!(test_property_value, property_value);

    let properties = context.get_runtime_properties_as_map_owned()?;
    let property_value = properties.get(&test_property_name).unwrap();
    assert_eq!(test_property_value, *property_value);

    Ok(())
}
