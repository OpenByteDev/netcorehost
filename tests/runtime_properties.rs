#![cfg(feature = "netcore3_0")]

use netcorehost::{nethost, pdcstr};
use rusty_fork::rusty_fork_test;

#[path = "common.rs"]
mod common;

rusty_fork_test! {
    #[test]
    fn runtime_properties() {
        common::setup();

        let hostfxr = nethost::load_hostfxr().unwrap();
        let context = hostfxr
            .initialize_for_runtime_config(common::test_runtime_config_path())
            .unwrap();

        let test_property_name = pdcstr!("TEST_PROPERTY");
        let test_property_value = pdcstr!("TEST_VALUE");
        context
            .set_runtime_property_value(test_property_name, test_property_value)
            .unwrap();
        let property_value = context
            .get_runtime_property_value(test_property_name)
            .unwrap();
        assert_eq!(test_property_value, property_value.as_ref());

        let properties = context.get_runtime_properties_as_map().unwrap();
        let property_value = properties.get(test_property_name).unwrap();
        assert_eq!(test_property_value, property_value.as_ref());
    }
}
