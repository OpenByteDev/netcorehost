use netcorehost::{nethost, pdcstr};

#[path = "common.rs"]
mod common;

#[test]
#[cfg(all(feature = "netcore3_0", feature = "sdk-resolver"))]
fn resolve_sdk() {
    let hostfxr = nethost::load_hostfxr().unwrap();
    let sdk = hostfxr
        .resolve_sdk(pdcstr!("."), pdcstr!("."), true)
        .unwrap();
    let sdks = hostfxr.get_available_sdks(pdcstr!("."));
    assert!(sdks.contains(&sdk.into_path()));
}

#[test]
#[cfg(all(feature = "netcore3_0", feature = "sdk-resolver"))]
fn list_sdks() {
    let hostfxr = nethost::load_hostfxr().unwrap();
    let sdks = hostfxr.get_available_sdks(pdcstr!("."));
    assert!(!sdks.is_empty());
}

#[test]
#[cfg(all(feature = "netcore2_1"))]
fn get_native_search_directories() {
    use netcorehost::pdcstring::PdCStr;

    let hostfxr = nethost::load_hostfxr().unwrap();
    let sdks = hostfxr
        .get_native_search_directories::<&PdCStr>(&[])
        .unwrap();
    assert!(!sdks.is_empty());
}
