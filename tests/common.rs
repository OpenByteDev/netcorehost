#![allow(unused)]

use netcorehost::pdcstring::PdCString;
use path_absolutize::Absolutize;
use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
};

pub fn test_netcore_version() -> String {
    env::var("NETCOREHOST_TEST_NETCORE_VERSION").unwrap_or_else(|_| "net10.0".to_string())
}

pub fn test_project_file_path() -> PathBuf {
    PathBuf::from_str(&format!(
        "tests/Test/Test-{}.csproj",
        test_netcore_version()
    ))
    .unwrap()
    .absolutize()
    .unwrap()
    .to_path_buf()
}

pub fn test_runtime_config_path() -> PdCString {
    PdCString::from_os_str(
        PathBuf::from_str(&format!(
            "tests/Test/bin/Debug/{}/Test.runtimeconfig.json",
            test_netcore_version()
        ))
        .unwrap()
        .absolutize()
        .unwrap()
        .as_os_str(),
    )
    .unwrap()
}

pub fn test_dll_path() -> PdCString {
    PdCString::from_os_str(
        PathBuf::from_str(&format!(
            "tests/Test/bin/Debug/{}/Test.dll",
            test_netcore_version()
        ))
        .unwrap()
        .absolutize()
        .unwrap()
        .as_os_str(),
    )
    .unwrap()
}

pub fn library_project_file_path() -> PathBuf {
    PathBuf::from_str(&format!(
        "tests/ClassLibrary/ClassLibrary-{}.csproj",
        test_netcore_version()
    ))
    .unwrap()
    .absolutize()
    .unwrap()
    .to_path_buf()
}

pub fn library_symbols_path() -> PdCString {
    PdCString::from_os_str(
        PathBuf::from_str(&format!(
            "tests/ClassLibrary/bin/Debug/{}/ClassLibrary.pdb",
            test_netcore_version()
        ))
        .unwrap()
        .absolutize()
        .unwrap()
        .as_os_str(),
    )
    .unwrap()
}

pub fn library_dll_path() -> PdCString {
    PdCString::from_os_str(
        PathBuf::from_str(&format!(
            "tests/ClassLibrary/bin/Debug/{}/ClassLibrary.dll",
            test_netcore_version()
        ))
        .unwrap()
        .absolutize()
        .unwrap()
        .as_os_str(),
    )
    .unwrap()
}

pub fn display_framework_id(id: &str) -> String {
    let s = id.trim_start_matches('.');

    if let Some(rest) = s.strip_prefix("netcoreapp") {
        // .netcoreappX.Y  → .NET Core X.Y
        let version = rest.trim_start_matches('.');
        return format!(".NET Core {}", version);
    }

    if let Some(rest) = s.strip_prefix("net") {
        // .netX.Y → .NET X.Y
        let version = rest.trim_start_matches('.');
        return format!(".NET {}", version);
    }

    // Fallback
    id.to_string()
}

pub fn setup() {
    println!("Running Test Setup");
    println!("Using {}", display_framework_id(&test_netcore_version()));

    println!("Building Test Project");
    build_test_project();
    println!("Building Library Project");
    build_library_project();
}

pub fn build_test_project() {
    if Path::new(&test_dll_path().to_os_string()).exists() {
        return;
    }

    let netcore_version = test_netcore_version();
    let project_file_path = test_project_file_path();
    let project_dir = project_file_path.parent().unwrap();

    Command::new("dotnet")
        .arg("build")
        .arg(&project_file_path)
        .arg("--framework")
        .arg(netcore_version)
        .current_dir(project_dir)
        .spawn()
        .expect("dotnet build failed")
        .wait()
        .expect("dotnet build failed");
}

pub fn build_library_project() {
    if Path::new(&library_dll_path().to_os_string()).exists() {
        return;
    }

    let netcore_version = test_netcore_version();
    let project_file_path = library_project_file_path();
    let project_dir = project_file_path.parent().unwrap();

    Command::new("dotnet")
        .arg("build")
        .arg(&project_file_path)
        .arg("--framework")
        .arg(netcore_version)
        .current_dir(project_dir)
        .spawn()
        .expect("dotnet build failed")
        .wait()
        .expect("dotnet build failed");
}
