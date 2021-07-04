use super::{char_t, size_t};

pub const PATH_MAX: size_t = 256;

pub const UNMANAGED_CALLERS_ONLY_METHOD: *const char_t = usize::MAX as *const _;
