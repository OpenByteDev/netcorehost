use crate::{
    bindings::hostfxr::wrapper::Hostfxr as HostfxrLib, dlopen2::wrapper::Container,
    error::HostingResult, nethost::LoadHostfxrError, pdcstring::PdCString,
};
use derive_more::From;
use std::{
    env::consts::EXE_SUFFIX,
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
    rc::Rc,
};

/// A struct representing a loaded hostfxr library.
#[derive(Clone, From)]
pub struct Hostfxr {
    /// The underlying hostfxr library.
    pub lib: Rc<Container<HostfxrLib>>,
    pub(crate) dotnet_bin: PdCString,
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
    pub fn load_from_path(path: impl AsRef<OsStr>) -> Result<Self, crate::dlopen2::Error> {
        let lib = Rc::new(unsafe { Container::load(&path) }?);
        let dotnet_bin = PdCString::from_os_str(find_dotnet_bin(path.as_ref())).unwrap();
        Ok(Self { lib, dotnet_bin })
    }

    /// Locates the hostfxr library using [`nethost`](crate::nethost) and loads it.
    #[cfg(feature = "nethost")]
    pub fn load_with_nethost() -> Result<Self, LoadHostfxrError> {
        crate::nethost::load_hostfxr()
    }
}

/// Either the exit code of the app if it ran successful, otherwise the error from the hosting components.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AppOrHostingResult(i32);

impl AppOrHostingResult {
    /// Gets the raw value of the result.
    #[must_use]
    pub fn value(self) -> i32 {
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
