#![cfg(all(feature = "netcore3_0", feature = "utils", unix))]
// see https://github.com/OpenByteDev/netcorehost/issues/38

#[path = "common.rs"]
mod common;

use netcorehost::{
    nethost, pdcstr,
    utils::altstack::{self, State},
};
use rusty_fork::{fork, rusty_fork_id, ChildWrapper, ExitStatusWrapper};
use std::{io::Read, process::Stdio};

macro_rules! function_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        type_name_of(f)
            .rsplit("::")
            .find(|&part| part != "f" && part != "{{closure}}")
            .expect("faled to get function name")
    }};
}

macro_rules! assert_contains {
    ($string:expr, $substring:expr $(,)?) => {{
        let string_ref: &str = &$string;
        let substring_ref: &str = &$substring;
        assert!(
            string_ref.contains(substring_ref),
            "Expected `{}` to contain `{}`",
            string_ref,
            substring_ref
        );
    }};
}

macro_rules! assert_not_contains {
    ($string:expr, $substring:expr $(,)?) => {{
        let string_ref: &str = &$string;
        let substring_ref: &str = &$substring;
        assert!(
            !string_ref.contains(substring_ref),
            "Expected `{}` NOT to contain `{}`",
            string_ref,
            substring_ref
        );
    }};
}

const MANAGED_HANDLER_OUTPUT: &str = "Unhandled exception. System.NullReferenceException: Object reference not set to an instance of an object.";

#[test]
fn segfault_with_small_altstack() {
    common::setup();
    altstack_test(
        function_name!(),
        || {
            altstack::set(State::Enabled { size: 2 * 1024 }).unwrap();
        },
        |status, _, stderr| {
            assert_eq!(status.unix_signal(), Some(libc::SIGSEGV));
            assert_not_contains!(stderr, MANAGED_HANDLER_OUTPUT);
        },
    );
}

#[test]
fn no_segfault_with_large_altstack() {
    common::setup();
    altstack_test(
        function_name!(),
        || {
            altstack::set(State::Enabled { size: 16 * 1024 }).unwrap();
        },
        |status, _, stderr| {
            assert_ne!(status.unix_signal(), Some(libc::SIGSEGV));
            assert_contains!(stderr, MANAGED_HANDLER_OUTPUT);
        },
    );
}

#[test]
fn no_segfault_with_altstack_disabled() {
    common::setup();
    altstack_test(
        function_name!(),
        || {
            altstack::set(State::Disabled).unwrap();
        },
        |status, _, stderr| {
            assert_ne!(status.unix_signal(), Some(libc::SIGSEGV));
            assert_contains!(stderr, MANAGED_HANDLER_OUTPUT);
        },
    );
}

fn altstack_test(
    test_name: &str,
    configure_altstack: impl FnOnce(),
    verify: impl FnOnce(ExitStatusWrapper, /* stdout */ String, /* stderr */ String),
) {
    common::setup();

    let body = || {
        configure_altstack();

        let hostfxr = nethost::load_hostfxr().unwrap();

        let context = hostfxr
            .initialize_for_runtime_config(common::test_runtime_config_path())
            .unwrap();
        let fn_loader = context
            .get_delegate_loader_for_assembly(common::test_dll_path())
            .unwrap();

        let throw_fn = fn_loader
            .get_function_with_unmanaged_callers_only::<unsafe fn()>(
                pdcstr!("Test.Program, Test"),
                pdcstr!("Throw"),
            )
            .unwrap();
        unsafe { throw_fn() };
    };

    fn configure_child(child: &mut std::process::Command) {
        child.stdout(Stdio::piped());
        child.stderr(Stdio::piped());
    }

    // Define how to supervise the child process
    let supervise = |child: &mut ChildWrapper, _file: &mut std::fs::File| {
        let mut stdout = String::new();
        child
            .inner_mut()
            .stdout
            .as_mut()
            .unwrap()
            .read_to_string(&mut stdout)
            .unwrap();

        let mut stderr = String::new();
        child
            .inner_mut()
            .stderr
            .as_mut()
            .unwrap()
            .read_to_string(&mut stderr)
            .unwrap();

        let status = child.wait().expect("unable to wait for child");
        println!("status: {status}");
        println!("stdout: {stdout}");
        println!("stderr: {stderr}");
        verify(status, stdout, stderr);
    };

    // Run the test in a forked child
    fork(
        test_name,
        rusty_fork_id!(),
        configure_child,
        supervise,
        body,
    )
    .expect("fork failed");
}
