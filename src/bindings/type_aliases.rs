#[cfg(windows)]
pub type char_t = u16;
#[cfg(not(windows))]
pub type char_t = i8;

pub type size_t = usize;
