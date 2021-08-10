use std::io;

use crate::hostfxr::HostExitCode;

quick_error! {
    /// An error struct encompassing all possible errors of this crate.
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
    }
}
