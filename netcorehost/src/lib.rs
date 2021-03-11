#![feature(maybe_uninit_uninit_array, maybe_uninit_extra, maybe_uninit_slice)]
#![allow(clippy::missing_safety_doc)]

#[macro_use]
extern crate dlopen_derive;
#[macro_use]
extern crate quick_error;

#[allow(non_camel_case_types, dead_code)]
pub mod bindings;
pub mod hostfxr;
pub mod nethost;

mod host_exit_code;
pub use host_exit_code::*;

mod error;
pub use error::*;
