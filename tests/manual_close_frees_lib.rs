use std::rc::Rc;

use netcorehost::{nethost, pdcstr};

#[path = "common.rs"]
mod common;

#[test]
fn manual_close_frees_lib() {
    common::setup();

    let hostfxr = nethost::load_hostfxr().unwrap();
    let context = hostfxr
        .initialize_for_runtime_config(pdcstr!(
            "tests/Test/bin/Debug/net5.0/Test.runtimeconfig.json"
        ))
        .unwrap();

    let weak = Rc::downgrade(&hostfxr.0);
    drop(hostfxr);
    context.close().unwrap();

    assert_eq!(weak.strong_count(), 0);
}