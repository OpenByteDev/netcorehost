use crate::{
    bindings::hostfxr::{
        wrapper::Hostfxr as HostfxrLib,
    },
    dlopen::wrapper::Container,
    nethost::LoadHostfxrError,
};

#[cfg_attr(all(nightly, feature = "doc-cfg"), attr(doc(cfg(feature = "netcore1_0"))))]
impl Hostfxr {
    /// Run an application.
    ///
    /// # Arguments
    ///  * `args` - command-line arguments
    ///
    /// This function does not return until the application completes execution.
    /// It will shutdown CoreCLR after the application executes.
    /// If the application is successfully executed, this value will return the exit code of the application.
    /// Otherwise, it will return an error code indicating the failure.
    pub fn run_app<A: AsRef<PdCStr>>(
        &self,
        args: &[A]
    ) -> AppOrHostingResult {
        let args = args.iter().map(|s| s.as_ref().as_ptr()).collect::<Vec<_>>();
        let result = unsafe {
            self.0.hostfxr_main(
                args.len().try_into().unwrap(),
                args.as_ptr(),
            )
        };
        AppOrHostingResult::from(result)
    }
}
