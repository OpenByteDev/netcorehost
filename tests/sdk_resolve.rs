use netcorehost::{nethost, pdcstr, pdcstring::PdCString};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

#[path = "common.rs"]
mod common;

#[test]
#[cfg(all(feature = "netcore3_0", feature = "sdk-resolver"))]
fn resolve_sdk() {
    let hostfxr = nethost::load_hostfxr().unwrap();

    let actual_sdks = get_sdks();
    let sdks_dir = actual_sdks
        .first()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let sdk = hostfxr
        .resolve_sdk(
            &PdCString::from_os_str(sdks_dir).unwrap(),
            pdcstr!("."),
            true,
        )
        .unwrap();

    assert!(actual_sdks.contains(&sdk.into_path()));
}

#[test]
#[cfg(all(feature = "netcore3_0", feature = "sdk-resolver"))]
fn list_sdks() {
    let hostfxr = nethost::load_hostfxr().unwrap();

    let mut actual_sdks = get_sdks();
    let sdks_dir = actual_sdks
        .first()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let mut sdks = hostfxr.get_available_sdks(&PdCString::from_os_str(sdks_dir).unwrap());

    sdks.sort();
    actual_sdks.sort();
    assert_eq!(actual_sdks, sdks);
}

#[test]
#[cfg(all(feature = "netcore2_1", windows))]
fn get_native_search_directories() {
    common::setup();

    let hostfxr = nethost::load_hostfxr().unwrap();
    let dirs = hostfxr
        .get_native_search_directories(pdcstr!("tests/Test/bin/Debug/net6.0/Test.dll"))
        .unwrap();
    dbg!(dirs);
}

fn get_sdks() -> Vec<PathBuf> {
    let sdks_output = Command::new("dotnet").arg("--list-sdks").output().unwrap();
    assert!(sdks_output.status.success());

    String::from_utf8_lossy(&sdks_output.stdout)
        .lines()
        .map(|line| {
            let (version, path) = line.split_once(" ").unwrap();
            Path::new(&path[1..(path.len() - 1)]).join(version)
        })
        .collect::<Vec<_>>()
}
