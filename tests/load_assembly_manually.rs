use netcorehost::{nethost, pdcstr};
use rusty_fork::rusty_fork_test;

#[path = "common.rs"]
mod common;

rusty_fork_test! {
    #[test]
    #[cfg(feature = "net8_0")]
    fn load_from_path() {
        common::setup();

        let hostfxr = nethost::load_hostfxr().unwrap();

        let context = hostfxr
            .initialize_for_dotnet_command_line(common::test_dll_path())
            // .initialize_for_runtime_config(common::test_runtime_config_path())
            .unwrap();

        context
            .load_assembly_from_path(common::test_dll_path())
            .unwrap();

        let fn_loader = context
            .get_delegate_loader_for_assembly(common::test_dll_path())
            .unwrap();
        let hello = fn_loader
            .get_function_with_unmanaged_callers_only::<fn() -> i32>(
                pdcstr!("Test.Program, Test"),
                pdcstr!("UnmanagedHello"),
            )
            .unwrap();

        let result = hello();
        assert_eq!(result, 42);
    }

    #[test]
    #[cfg(feature = "net8_0")]
    fn load_from_bytes() {
        common::setup();

        let hostfxr = nethost::load_hostfxr().unwrap();
        let result = hostfxr.run_app(&common::test_dll_path());
        result.as_hosting_exit_code().unwrap();
        assert_eq!(result.value(), 42);
    }
}
