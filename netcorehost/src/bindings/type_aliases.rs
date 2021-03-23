#[cfg(windows)]
pub(crate) type char_t = u16;
#[cfg(not(windows))]
pub(crate) type char_t = i8;

pub(crate) type size_t = usize;
