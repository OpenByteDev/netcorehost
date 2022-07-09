#![cfg(feature = "netcore3_0")]

use netcorehost::nethost;
use rusty_fork::rusty_fork_test;

#[path = "common.rs"]
mod common;

rusty_fork_test! {
    #[test]
    fn primary_is_primary() {
        common::setup();

        let hostfxr = nethost::load_hostfxr().unwrap();
        let context = hostfxr
            .initialize_for_runtime_config(common::test_runtime_config_path())
            .unwrap();
        assert!(context.is_primary());
        unsafe { context.close() }.unwrap();
    }

    #[test]
    fn secondary_is_secondary() {
        common::setup();

        let hostfxr = nethost::load_hostfxr().unwrap();
        let context = hostfxr
            .initialize_for_dotnet_command_line(common::test_dll_path())
            .unwrap();
        assert!(context.is_primary());
        context.run_app().as_hosting_exit_code().unwrap();

        let context2 = hostfxr
            .initialize_for_runtime_config(common::test_runtime_config_path())
            .unwrap();
        assert!(!context2.is_primary());

        unsafe { context2.close() }.unwrap();
    }
}
