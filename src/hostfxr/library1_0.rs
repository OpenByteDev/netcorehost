use std::{io, iter};

use crate::{
    hostfxr::{AppOrHostingResult, Hostfxr},
    pdcstring::{PdCStr, PdChar},
};

impl Hostfxr {
    /// Run an application.
    ///
    /// # Note
    /// This function does not return until the application completes execution.
    /// It will shutdown CoreCLR after the application executes.
    /// If the application is successfully executed, this value will return the exit code of the application.
    /// Otherwise, it will return an error code indicating the failure.
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "netcore1_0")))]
    #[must_use]
    pub fn run_app(&self, app_path: &PdCStr) -> AppOrHostingResult {
        self._run_app(&[app_path.as_ptr()])
    }

    /// Run an application.
    ///
    /// # Arguments
    ///  * `args` - command-line arguments
    ///
    /// # Note
    /// This function does not return until the application completes execution.
    /// It will shutdown CoreCLR after the application executes.
    /// If the application is successfully executed, this value will return the exit code of the application.
    /// Otherwise, it will return an error code indicating the failure.
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "netcore1_0")))]
    pub fn run_app_with_args<A: AsRef<PdCStr>>(
        &self,
        app_path: &PdCStr,
        args: &[A],
    ) -> io::Result<AppOrHostingResult> {
        let args = iter::once(app_path)
            .chain(args.iter().map(|s| s.as_ref()))
            .map(|s| s.as_ptr())
            .collect::<Vec<_>>();
        let result = self._run_app(&args);
        Ok(result)
    }

    fn _run_app(&self, args: &[*const PdChar]) -> AppOrHostingResult {
        let result = unsafe {
            self.0
                .hostfxr_main(args.len().try_into().unwrap(), args.as_ptr())
        };
        AppOrHostingResult::from(result)
    }
}
