use std::{borrow::Borrow, path::Path};

use netcorehost::HostExitCode;
use path_absolutize::Absolutize;
use widestring::WideCString;

#[test]
fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    let test_out_dir = Path::new("tests\\Test\\bin\\Debug\\net5.0").absolutize()?;
    let assembly_path = WideCString::from_os_str(Path::join(&test_out_dir, "Test.dll"))?;

    let hostfxr = netcorehost::nethost::load_hostfxr()?;
    let args = [assembly_path.borrow()];
    let context = hostfxr.initialize_for_dotnet_command_line(&args)?;
    let result = context.run_app();
    assert_eq!(result, HostExitCode::from(42));

    Ok(())
}
