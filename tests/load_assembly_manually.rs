use netcorehost::{nethost, pdcstr};
use rusty_fork::rusty_fork_test;
use std::fs;

#[path = "common.rs"]
mod common;

rusty_fork_test! {
    #[test]
    #[cfg(feature = "net8_0")]
    fn load_from_path() {
        common::setup();

        let hostfxr = nethost::load_hostfxr().unwrap();

        let context = hostfxr
            .initialize_for_runtime_config(common::test_runtime_config_path())
            .unwrap();

        context
            .load_assembly_from_path(common::library_dll_path())
            .unwrap();

        let fn_loader = context
            .get_delegate_loader_for_assembly(common::library_dll_path())
            .unwrap();
        let hello = fn_loader
            .get_function_with_unmanaged_callers_only::<fn() -> i32>(
                pdcstr!("ClassLibrary.Library, ClassLibrary"),
                pdcstr!("Hello"),
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

        let context = hostfxr
            .initialize_for_runtime_config(common::test_runtime_config_path())
            .unwrap();

        let assembly_bytes = fs::read(common::library_dll_path().to_os_string()).unwrap();
        let symbol_bytes = fs::read(common::library_symbols_path().to_os_string()).unwrap();

        context
            .load_assembly_from_bytes(assembly_bytes, symbol_bytes)
            .unwrap();

        let fn_loader = context
            .get_delegate_loader_for_assembly(common::library_dll_path())
            .unwrap();
        let hello = fn_loader
            .get_function_with_unmanaged_callers_only::<fn() -> i32>(
                pdcstr!("ClassLibrary.Library, ClassLibrary"),
                pdcstr!("Hello"),
            )
            .unwrap();

        let result = hello();
        assert_eq!(result, 42);
    }
}
