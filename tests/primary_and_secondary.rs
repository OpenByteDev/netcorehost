use netcorehost::{nethost, pdcstr};

#[path = "common.rs"]
mod common;

#[test]
fn primary_is_primary() {
    common::setup();

    let hostfxr = nethost::load_hostfxr().unwrap();
    let context = hostfxr
        .initialize_for_runtime_config(pdcstr!(
            "tests/Test/bin/Debug/net5.0/Test.runtimeconfig.json"
        ))
        .unwrap();
    assert!(context.is_primary());
    context.close().unwrap();

    let context2 = hostfxr
        .initialize_for_dotnet_command_line(pdcstr!("tests/Test/bin/Debug/net5.0/Test.dll"))
        .unwrap();
    assert!(context2.is_primary());
    context2.close().unwrap();
}

#[test]
fn secondary_is_secondary() {
    common::setup();

    let hostfxr = nethost::load_hostfxr().unwrap();
    let context = hostfxr
        .initialize_for_dotnet_command_line(pdcstr!("tests/Test/bin/Debug/net5.0/Test.dll"))
        .unwrap();
    assert!(context.is_primary());
    context.run_app().as_hosting_exit_code().unwrap();

    let context2 = hostfxr
        .initialize_for_runtime_config(pdcstr!(
            "tests/Test/bin/Debug/net5.0/Test.runtimeconfig.json"
        ))
        .unwrap();
    assert!(!context2.is_primary());

    context2.close().unwrap();
}
