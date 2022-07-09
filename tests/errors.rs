#![cfg(feature = "netcore3_0")]

use netcorehost::{hostfxr::GetManagedFunctionError, nethost, pdcstr};
use rusty_fork::rusty_fork_test;

#[path = "common.rs"]
mod common;

rusty_fork_test! {
    #[test]
    fn get_function_pointer() {
        common::setup();

        let hostfxr = nethost::load_hostfxr().unwrap();

        let context = hostfxr
            .initialize_for_runtime_config(common::test_runtime_config_path())
            .unwrap();
        let fn_loader = context
            .get_delegate_loader_for_assembly(common::test_dll_path())
            .unwrap();

        let invalid_method_name = fn_loader.get_function_with_default_signature(
            pdcstr!("Test.Program, Test"),
            pdcstr!("SomeMethodThatDoesNotExist"),
        );
        assert!(invalid_method_name.is_err());
        assert_eq!(
            unsafe { invalid_method_name.unwrap_err_unchecked() },
            GetManagedFunctionError::MissingMethod
        );

        let invalid_method_signature = fn_loader
            .get_function_with_default_signature(pdcstr!("Test.Program, Test"), pdcstr!("Main"));
        assert!(invalid_method_signature.is_err());
        assert_eq!(
            unsafe { invalid_method_signature.unwrap_err_unchecked() },
            GetManagedFunctionError::MissingMethod
        );

        let invalid_type_name = fn_loader.get_function_with_default_signature(
            pdcstr!("Test.SomeTypeThatDoesNotExist, Test"),
            pdcstr!("Hello"),
        );
        assert!(invalid_type_name.is_err());
        assert_eq!(
            unsafe { invalid_type_name.unwrap_err_unchecked() },
            GetManagedFunctionError::TypeNotFound
        );

        let invalid_namespace_name = fn_loader.get_function_with_default_signature(
            pdcstr!("SomeNamespaceThatDoesNotExist.Program, Test"),
            pdcstr!("Hello"),
        );
        assert!(invalid_namespace_name.is_err());
        assert_eq!(
            unsafe { invalid_namespace_name.unwrap_err_unchecked() },
            GetManagedFunctionError::TypeNotFound
        );

        let invalid_assembly_name = fn_loader.get_function_with_default_signature(
            pdcstr!("Test.Program, SomeAssemblyThatDoesNotExist"),
            pdcstr!("Hello"),
        );
        assert!(invalid_assembly_name.is_err());
        assert_eq!(
            unsafe { invalid_assembly_name.unwrap_err_unchecked() },
            GetManagedFunctionError::AssemblyNotFound
        );

        let method_not_marked = fn_loader.get_function_with_unmanaged_callers_only::<fn()>(
            pdcstr!("Test.Program, Test"),
            pdcstr!("Hello"),
        );
        assert!(method_not_marked.is_err());
        assert_eq!(
            unsafe { method_not_marked.unwrap_err_unchecked() },
            GetManagedFunctionError::MethodNotUnmanagedCallersOnly
        );

        let invalid_delegate_type_name = fn_loader.get_function::<fn()>(
            pdcstr!("Test.Program, Test"),
            pdcstr!("Hello"),
            pdcstr!("Test.Program+SomeDelegateThatDoesNotExist, Test"),
        );
        assert!(invalid_delegate_type_name.is_err());
        assert_eq!(
            unsafe { invalid_delegate_type_name.unwrap_err_unchecked() },
            GetManagedFunctionError::TypeNotFound
        );

        unsafe { context.close() }.unwrap();
    }

    #[test]
    fn get_delegate_loader_for_assembly() {
        common::setup();

        let hostfxr = nethost::load_hostfxr().unwrap();

        let context = hostfxr
            .initialize_for_runtime_config(common::test_runtime_config_path())
            .unwrap();

        let fn_loader = context
            .get_delegate_loader_for_assembly(pdcstr!("tests/errors.rs"))
            .unwrap();
        let invalid_assembly_path = fn_loader
            .get_function_with_default_signature(pdcstr!("Test.Program, Test"), pdcstr!("Hello"));
        assert!(invalid_assembly_path.is_err());
        assert_eq!(
            unsafe { invalid_assembly_path.unwrap_err_unchecked() },
            GetManagedFunctionError::AssemblyNotFound
        );

        let fn_loader = context
            .get_delegate_loader_for_assembly(pdcstr!("PathThatDoesNotExist.dll"))
            .unwrap();
        let non_existant_assembly_path = fn_loader
            .get_function_with_default_signature(pdcstr!("Test.Program, Test"), pdcstr!("Hello"));
        assert!(non_existant_assembly_path.is_err());
        assert_eq!(
            unsafe { non_existant_assembly_path.unwrap_err_unchecked() },
            GetManagedFunctionError::AssemblyNotFound
        );

        unsafe { context.close() }.unwrap();
    }
}
