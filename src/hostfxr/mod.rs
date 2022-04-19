mod library;
pub use library::*;

#[cfg(feature = "netcore3_0")]
mod library3_0;
#[cfg(feature = "netcore3_0")]
pub use library3_0::*;

#[cfg(feature = "netcore2_1")]
mod library2_1;
#[cfg(feature = "netcore2_1")]
pub use library2_1::*;

#[cfg(feature = "netcore3_0")]
mod context;
#[cfg(feature = "netcore3_0")]
pub use context::*;

#[cfg(feature = "netcore3_0")]
mod delegate_loader;
#[cfg(feature = "netcore3_0")]
pub use delegate_loader::*;

#[cfg(feature = "netcore3_0")]
mod managed_function;
#[cfg(feature = "netcore3_0")]
pub use managed_function::*;
