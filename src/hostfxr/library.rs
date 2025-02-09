use crate::{
    dlopen2::wrapper::Container,
    error::{HostingError, HostingResult},
    pdcstring::PdCString,
};
use derive_more::From;
use std::{
    env::consts::EXE_SUFFIX,
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Arc,
};

pub(crate) type HostfxrLibrary = Container<crate::bindings::hostfxr::wrapper_option::Hostfxr>;
pub(crate) type SharedHostfxrLibrary = Arc<HostfxrLibrary>;
#[allow(unused, clippy::cast_possible_wrap)]
pub(crate) const UNSUPPORTED_HOST_VERSION_ERROR_CODE: i32 =
    HostingError::HostApiUnsupportedVersion.value() as i32;

/// A struct representing a loaded hostfxr library.
#[derive(Clone, From)]
pub struct Hostfxr {
    /// The underlying hostfxr library.
    pub lib: SharedHostfxrLibrary,
    pub(crate) dotnet_exe: PdCString,
}

fn find_dotnet_bin(hostfxr_path: impl AsRef<Path>) -> PathBuf {
    let mut p = hostfxr_path.as_ref().to_path_buf();
    loop {
        if let Some(dir) = p.file_name() {
            if dir == "dotnet" || dir == ".dotnet" {
                break;
            }
            p.pop();
        } else {
            p.clear();
            break;
        }
    }
    p.push("dotnet");
    let mut p = OsString::from(p);
    p.extend(Path::new(EXE_SUFFIX));
    PathBuf::from(p)
}

impl Hostfxr {
    /// Loads the hostfxr library from the given path.
    pub fn load_from_path(path: impl AsRef<Path>) -> Result<Self, crate::dlopen2::Error> {
        let path = path.as_ref();
        let lib = SharedHostfxrLibrary::new(unsafe { Container::load(path) }?);

        // Some APIs of hostfxr.dll require a path to the dotnet executable, so we try to locate it here based on the hostfxr path.
        let dotnet_exe = PdCString::from_os_str(find_dotnet_bin(path)).unwrap();

        Ok(Self { lib, dotnet_exe })
    }

    /// Locates the hostfxr library using [`nethost`](crate::nethost) and loads it.
    #[cfg(feature = "nethost")]
    pub fn load_with_nethost() -> Result<Self, crate::nethost::LoadHostfxrError> {
        crate::nethost::load_hostfxr()
    }

    /// Returns the path to the dotnet root.
    #[must_use]
    pub fn get_dotnet_root(&self) -> PathBuf {
        self.get_dotnet_exe().parent().unwrap().to_owned()
    }

    /// Returns the path to the dotnet executable of the same installation as hostfxr.
    #[must_use]
    pub fn get_dotnet_exe(&self) -> PathBuf {
        self.dotnet_exe.to_os_string().into()
    }
}

/// Either the exit code of the app if it ran successful, otherwise the error from the hosting components.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AppOrHostingResult(i32);

impl AppOrHostingResult {
    /// Gets the raw value of the result.
    #[must_use]
    pub const fn value(&self) -> i32 {
        self.0
    }

    /// Converts the result to an hosting exit code.
    pub fn as_hosting_exit_code(self) -> HostingResult {
        HostingResult::from(self.0)
    }
}

impl From<AppOrHostingResult> for i32 {
    fn from(code: AppOrHostingResult) -> Self {
        code.value()
    }
}

impl From<i32> for AppOrHostingResult {
    fn from(code: i32) -> Self {
        Self(code)
    }
}
