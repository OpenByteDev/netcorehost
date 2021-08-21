use netcorehost::{hostfxr::HostExitCode, nethost, pdcstr};

#[path = "common.rs"]
mod common;

#[test]
fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    common::setup();

    let hostfxr = nethost::load_hostfxr()?;
    let context = hostfxr
        .initialize_for_dotnet_command_line(pdcstr!("tests/Test/bin/Debug/net5.0/Test.dll"))?;
    let result = context.run_app();
    assert_eq!(result, HostExitCode::from(42));

    Ok(())
}
