use netcorehost::{nethost, pdcstr};

#[path = "common.rs"]
mod common;

#[test]
#[cfg(feature = "netcore3_0")]
fn run_app_with_context() {
    common::setup();

    let hostfxr = nethost::load_hostfxr().unwrap();
    let context = hostfxr
        .initialize_for_dotnet_command_line(pdcstr!("tests/Test/bin/Debug/net6.0/Test.dll"))
        .unwrap();
    let result = context.run_app().value();
    assert_eq!(result, 42);
}

/* fails on Github CI
rusty_fork_test! {
    #[test]
    #[cfg(feature = "netcore1_0")]
    fn run_app_direct() {
        common::setup();

        let hostfxr = nethost::load_hostfxr().unwrap();
        let result = hostfxr.run_app(pdcstr!("./tests/Test/bin/Debug/net6.0/Test.dll"));
        assert_eq!(result.value(), 42);
    }
}
*/
