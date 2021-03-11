use std::io;

use crate::HostExitCode;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        DlOpen(err: dlopen::Error) {
            from()
            display("dlopen error: {}", err)
            source(err)
        }
        IO(err: io::Error) {
            from()
            display("io error: {}", err)
            source(err)
        }
        Hostfxr(error_code: HostExitCode)
        UnsupportedOS
        Other(descr: &'static str) {
            display("error: {}", descr)
        }
    }
}
