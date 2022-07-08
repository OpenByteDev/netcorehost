#![cfg(feature = "netcore3_0")]

use netcorehost::{nethost, pdcstr};
use rusty_fork::rusty_fork_test;
use std::ptr;

#[path = "common.rs"]
mod common;

rusty_fork_test! {
    #[test]
    fn hello_world() {
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
            .get_function_with_default_signature(pdcstr!("Test.Program, Test"), pdcstr!("Hello"))
            .unwrap();
        let result = unsafe { hello(ptr::null(), 0) };
        assert_eq!(result, 42);
    }

    #[test]
    fn hello_world_twice() {
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

        let hello_one = fn_loader
            .get_function_with_default_signature(pdcstr!("Test.Program, Test"), pdcstr!("Hello"))
            .unwrap();
        let result = unsafe { hello_one(ptr::null(), 0) };
        assert_eq!(result, 42);

        let hello_two = fn_loader
            .get_function_with_default_signature(pdcstr!("Test.Program, Test"), pdcstr!("Hello2"))
            .unwrap();
        let result = unsafe { hello_two(ptr::null(), 0) };
        assert_eq!(result, 0);
    }
}
