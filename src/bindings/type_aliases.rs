/// The char type used in nethost and hostfxr. Either u8 on unix systems or u16 on windows.
#[cfg(windows)]
pub type char_t = u16;
/// The char type used in nethost and hostfxr. Either u8 on unix systems or u16 on windows.
#[cfg(not(windows))]
pub type char_t = std::os::raw::c_char;

pub type size_t = usize;
