/// Module for constants related to hostfxr or nethost or useful for interacting with them.
pub mod consts;

/// Module containing the raw bindings for hostfxr.
pub mod hostfxr;

/// Module containing the raw bindings for nethost.
#[cfg(feature = "nethost")]
pub mod nethost;

mod type_aliases;
pub use type_aliases::*;
