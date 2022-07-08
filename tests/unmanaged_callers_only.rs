#![cfg(feature = "netcore3_0")]

use netcorehost::{nethost, pdcstr};
use rusty_fork::rusty_fork_test;

#[path = "common.rs"]
mod common;

rusty_fork_test! {
    fn unmanaged_caller_hello_world() {
        common::setup();

        let hostfxr = nethost::load_hostfxr().unwrap();

        let context = hostfxr
            .initialize_for_runtime_config(pdcstr!(
                "tests/Test/bin/Debug/net6.0/Test.runtimeconfig.json"
            ))
            .unwrap();
        let fn_loader = context
            .get_delegate_loader_for_assembly(pdcstr!("tests/Test/bin/Debug/net6.0/Test.dll"))
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
}
