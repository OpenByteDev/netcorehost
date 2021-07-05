use super::{char_t, size_t};

/// The maximum path length on windows
// not really - https://stackoverflow.com/a/837855/6304917
pub const MAX_PATH: size_t = 260;

/// Signifies that the target method is marked with the [`UnmanagedCallersOnlyAttribute`].
/// This means that the name alone can identify the target method.
///
/// [`UnmanagedCallersOnlyAttribute`]: https://docs.microsoft.com/en-us/dotnet/api/system.runtime.interopservices.unmanagedcallersonlyattribute
pub const UNMANAGED_CALLERS_ONLY_METHOD: *const char_t = usize::MAX as *const _;
