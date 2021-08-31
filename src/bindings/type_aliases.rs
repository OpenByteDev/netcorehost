/// The char type used in nethost and hostfxr. Either [`c_char`](std::os::raw::c_char) on unix systems or [`u16`] on windows.
#[cfg(not(windows))]
pub type char_t = std::os::raw::c_char;
/// The char type used in nethost and hostfxr. Either [`c_char`](std::os::raw::c_char) on unix systems or [`u16`] on windows.
#[cfg(windows)]
pub type char_t = u16;

/// Equivalent to `size_t` in C.
pub type size_t = usize;
