use netcorehost::{nethost, pdcstr};

#[path = "common.rs"]
mod common;

#[test]
fn runtime_properties() -> Result<(), Box<dyn std::error::Error>> {
    common::setup();

    let hostfxr = nethost::load_hostfxr()?;
    let context = hostfxr.initialize_for_runtime_config(pdcstr!(
        "tests/Test/bin/Debug/net5.0/Test.runtimeconfig.json"
    ))?;

    let test_property_name = pdcstr!("TEST_PROPERTY");
    let test_property_value = pdcstr!("TEST_VALUE");
    context.set_runtime_property_value(&test_property_name, &test_property_value)?;
    let property_value = context.get_runtime_property_value_owned(&test_property_name)?;
    assert_eq!(test_property_value, property_value.as_ref());

    let properties = context.get_runtime_properties_owned_as_map()?;
    let property_value = properties.get(test_property_name).unwrap();
    assert_eq!(test_property_value, property_value.as_ref());

    Ok(())
}
