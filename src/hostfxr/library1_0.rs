use crate::{
    hostfxr::{AppOrHostingResult, Hostfxr},
    pdcstr,
    pdcstring::PdCStr,
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
        self.run_app_with_args::<&PdCStr>(app_path, &[])
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
    ) -> AppOrHostingResult {
        let args = [pdcstr!("dotnet"), app_path]
            .into_iter()
            .chain(args.iter().map(|s| s.as_ref()))
            .map(|s| s.as_ptr())
            .collect::<Vec<_>>();

        let result = unsafe {
            self.0
                .hostfxr_main(args.len().try_into().unwrap(), args.as_ptr())
        };

        AppOrHostingResult::from(result)
    }
}
