use std::path::Path;

use netcorehost::PdCString;
use netcorehost::{nethost, HostExitCode};
use path_absolutize::Absolutize;

#[test]
fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    let test_out_dir = Path::new("tests\\Test\\bin\\Debug\\net5.0").absolutize()?;
    let assembly_path = PdCString::from_os_str(Path::join(&test_out_dir, "Test.dll"))?;

    let hostfxr = nethost::load_hostfxr()?;
    let context = hostfxr.initialize_for_dotnet_command_line(assembly_path)?;
    let result = context.run_app();
    assert_eq!(result, HostExitCode::from(42));

    Ok(())
}
