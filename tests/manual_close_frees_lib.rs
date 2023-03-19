#![cfg(feature = "netcore3_0")]

use std::sync::Arc;

use netcorehost::nethost;
use rusty_fork::rusty_fork_test;

#[path = "common.rs"]
mod common;

rusty_fork_test! {
#[test]
    fn manual_close_frees_lib() {
        common::setup();

        let hostfxr = nethost::load_hostfxr().unwrap();
        let context = hostfxr
            .initialize_for_runtime_config(common::test_runtime_config_path())
            .unwrap();

        let weak = Arc::downgrade(&hostfxr.lib);
        drop(hostfxr);
        unsafe { context.close() }.unwrap();

        assert_eq!(weak.strong_count(), 0);
    }
}
