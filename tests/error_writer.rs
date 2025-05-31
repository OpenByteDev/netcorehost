#![allow(deprecated)]

use netcorehost::{hostfxr::Hostfxr, nethost, pdcstr};
use rusty_fork::rusty_fork_test;
use std::cell::Cell;

#[path = "common.rs"]
mod common;

fn cause_error(hostfxr: &Hostfxr) {
    let bad_path = pdcstr!("bad.runtimeconfig.json");
    let _ = hostfxr.initialize_for_runtime_config(bad_path);
}

rusty_fork_test! {
    #[test]
    #[cfg(feature = "netcore3_0")]
    fn gets_called() {
        common::setup();

        let hostfxr = nethost::load_hostfxr().unwrap();
        let was_called = Box::leak(Box::new(Cell::new(false)));
        hostfxr.set_error_writer(Some(Box::new(
            |_| { was_called.set(true); }
        )));
        cause_error(&hostfxr);

        assert!(was_called.get());
    }

    #[test]
    #[cfg(feature = "netcore3_0")]
    fn can_be_replaced() {
        common::setup();

        let hostfxr = nethost::load_hostfxr().unwrap();

        let counter = Box::leak(Box::new(Cell::new(0)));
        hostfxr.set_error_writer(Some(Box::new(
            |_| { counter.set(counter.get() + 1); }
        )));
        cause_error(&hostfxr);
        hostfxr.set_error_writer(Some(Box::new(
            |_| { }
        )));
        cause_error(&hostfxr);
        hostfxr.set_error_writer(Some(Box::new(
            |_| { counter.set(counter.get() + 1); }
        )));
        cause_error(&hostfxr);
        hostfxr.set_error_writer(None);
        cause_error(&hostfxr);

        assert_eq!(counter.get(), 2);
    }
}
