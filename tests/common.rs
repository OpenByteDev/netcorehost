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
    env::var("NETCOREHOST_TEST_NETCORE_VERSION").unwrap_or_else(|_| "net8.0".to_string())
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

pub fn setup() {
    

    build_test_project();
    build_library_project();
}

pub fn build_test_project() {
    if Path::new(&test_dll_path().to_os_string()).exists() {
        return;
    }

    Command::new("dotnet")
        .arg("build")
        .arg("Test.sln")
        .arg("--framework")
        .arg(&test_netcore_version())
        .current_dir("tests/Test")
        .spawn()
        .expect("dotnet build failed")
        .wait()
        .expect("dotnet build failed");
}

pub fn build_library_project() {
    if Path::new(&library_dll_path().to_os_string()).exists() {
        return;
    }

    Command::new("dotnet")
        .arg("build")
        .arg("ClassLibrary.sln")
        .arg("--framework")
        .arg(&test_netcore_version())
        .current_dir("tests/ClassLibrary")
        .spawn()
        .expect("dotnet build failed")
        .wait()
        .expect("dotnet build failed");
}
