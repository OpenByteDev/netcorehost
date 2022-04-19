use netcorehost::{nethost, pdcstr};

#[path = "common.rs"]
mod common;

#[test]
#[cfg(feature = "netcore3_0")]
fn run_app_with_context() {
    common::setup();

    let hostfxr = nethost::load_hostfxr().unwrap();
    let context = hostfxr
        .initialize_for_dotnet_command_line(pdcstr!("tests/Test/bin/Debug/net5.0/Test.dll"))
        .unwrap();
    let result = context.run_app().value();
    assert_eq!(result, 42);
}

#[test]
#[cfg(feature = "netcore4_0")]
fn run_app_direct() {
    common::setup();

    let hostfxr = nethost::load_hostfxr().unwrap();
    let result = hostfxr.run_app(&[]).value();
    assert_eq!(result, 42);
}
