#![cfg_attr(
    any(nightly, feature = "nightly"),
    feature(try_trait_v2, maybe_uninit_uninit_array, maybe_uninit_slice)
)]
#![cfg_attr(feature = "doc-cfg", feature(doc_cfg))]
#![warn(clippy::pedantic, clippy::cargo, unsafe_op_in_unsafe_fn, missing_docs)]
#![allow(
    clippy::missing_safety_doc,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::multiple_crate_versions,
    clippy::doc_markdown,
    clippy::cast_sign_loss,
    clippy::shadow_unrelated,
    clippy::redundant_closure_for_method_calls,
    clippy::transmute_ptr_to_ptr
)]

//! A Rust library for hosting the .NET Core runtime.
//!
//! It utilizes the .NET Core hosting API to load and execute managed code from withing the current process.
//!
//! # Usage
//! ## Running an application
//! The example below will setup the runtime, load `Test.dll` and run its `Main` method:
//! ```rust
//! # #[path = "../tests/common.rs"]
//! # mod common;
//! # common::setup();
//! # use netcorehost::{nethost, pdcstr};
//! let hostfxr = nethost::load_hostfxr().unwrap();
//! let context = hostfxr.initialize_for_dotnet_command_line(pdcstr!("tests/Test/bin/Debug/net5.0/Test.dll")).unwrap();
//! let result = context.run_app().value();
//! ```
//! The full example can be found in [examples/run-app](https://github.com/OpenByteDev/netcorehost/tree/master/examples/run-app).
//!
//! ## Calling a managed function
//! A function pointer to a managed method can be aquired using an [`AssemblyDelegateLoader`](crate::hostfxr::AssemblyDelegateLoader).
//! This is only supported for [`HostfxrContext`'s](crate::hostfxr::HostfxrContext) that are initialized using [`Hostfxr::initialize_for_runtime_config`](crate::hostfxr::Hostfxr::initialize_for_runtime_config).
//! The [`runtimeconfig.json`](https://docs.microsoft.com/en-us/dotnet/core/run-time-config/) is automatically generated for executables, for libraries it is neccessary to add  `<GenerateRuntimeConfigurationFiles>True</GenerateRuntimeConfigurationFiles>` to the projects `.csproj` file.
//! ### Using the default signature
//! The default method signature is defined as follows:
//! ```cs
//! public delegate int ComponentEntryPoint(IntPtr args, int sizeBytes);
//! ```
//!
//! A method with the default signature (see code below) can be loaded using [`AssemblyDelegateLoader::get_function_with_default_signature`].
//!
//! **C#**
//! ```cs
//! using System;
//!
//! namespace Test {
//!     public static class Program {
//!         public static int Hello(IntPtr args, int sizeBytes) {
//!             Console.WriteLine("Hello from C#!");
//!             return 42;
//!         }
//!     }
//! }
//! ```
//!
//! **Rust**
//! ```rust
//! # #[path = "../tests/common.rs"]
//! # mod common;
//! # common::setup();
//! # use netcorehost::{nethost, pdcstr};
//! let hostfxr = nethost::load_hostfxr().unwrap();
//! let context =
//!     hostfxr.initialize_for_runtime_config(pdcstr!("tests/Test/bin/Debug/net5.0/Test.runtimeconfig.json")).unwrap();
//! let fn_loader =
//!     context.get_delegate_loader_for_assembly(pdcstr!("tests/Test/bin/Debug/net5.0/Test.dll")).unwrap();
//! let hello = fn_loader.get_function_with_default_signature(
//!     pdcstr!("Test.Program, Test"),
//!     pdcstr!("Hello"),
//! ).unwrap();
//! let result = unsafe { hello(std::ptr::null(), 0) };
//! assert_eq!(result, 42);
//! ```
//!
//! ### Using UnmanagedCallersOnly
//! A function pointer to a method annotated with [`UnmanagedCallersOnly`] can be loaded without
//! specifying its signature (as these methods cannot be overloaded).
//!
//! **C#**
//! ```cs
//! using System;
//! using System.Runtime.InteropServices;
//!
//! namespace Test {
//!     public static class Program {
//!         [UnmanagedCallersOnly]
//!         public static void UnmanagedHello() {
//!             Console.WriteLine("Hello from C#!");
//!         }
//!     }
//! }
//! ```
//!
//! **Rust**
//! ```rust
//! # #[path = "../tests/common.rs"]
//! # mod common;
//! # common::setup();
//! # use netcorehost::{nethost, pdcstr};
//! let hostfxr = nethost::load_hostfxr().unwrap();
//! let context =
//!     hostfxr.initialize_for_runtime_config(pdcstr!("tests/Test/bin/Debug/net5.0/Test.runtimeconfig.json")).unwrap();
//! let fn_loader =
//!     context.get_delegate_loader_for_assembly(pdcstr!("tests/Test/bin/Debug/net5.0/Test.dll")).unwrap();
//! let hello = fn_loader.get_function_with_unmanaged_callers_only::<fn()>(
//!     pdcstr!("Test.Program, Test"),
//!     pdcstr!("UnmanagedHello"),
//! ).unwrap();
//! hello();
//! ```
//!
//!
//! ### Specifying the delegate type
//! Another option is to define a custom delegate type and passing its assembly qualified name to [`AssemblyDelegateLoader::get_function`].
//!
//! **C#**
//! ```cs
//! using System;
//!
//! namespace Test {
//!     public static class Program {
//!         public delegate void CustomHelloFunc();
//!     
//!         public static void CustomHello() {
//!             Console.WriteLine("Hello from C#!");
//!         }
//!     }
//! }
//! ```
//!
//! **Rust**
//! ```rust
//! # #[path = "../tests/common.rs"]
//! # mod common;
//! # common::setup();
//! # use netcorehost::{nethost, pdcstr};
//! let hostfxr = nethost::load_hostfxr().unwrap();
//! let context =
//!     hostfxr.initialize_for_runtime_config(pdcstr!("tests/Test/bin/Debug/net5.0/Test.runtimeconfig.json")).unwrap();
//! let fn_loader =
//!     context.get_delegate_loader_for_assembly(pdcstr!("tests/Test/bin/Debug/net5.0/Test.dll")).unwrap();
//! let hello = fn_loader.get_function::<fn()>(
//!     pdcstr!("Test.Program, Test"),
//!     pdcstr!("CustomHello"),
//!     pdcstr!("Test.Program+CustomHelloFunc, Test")
//! ).unwrap();
//! hello();
//! ```
//!
//! The full examples can be found in [examples/call-managed-function](https://github.com/OpenByteDev/netcorehost/tree/master/examples/call-managed-function).
//!
//! ## Passing complex parameters
//! Examples for passing non-primitive parameters can be found in [examples/passing-parameters](https://github.com/OpenByteDev/netcorehost/tree/master/examples/passing-parameters).
//!
//! # Features
//! - `nethost` - Links against nethost and allows for automatic detection of the hostfxr library.
//! - `download-nethost` - Automatically downloads the latest nethost binary from [NuGet](https://www.nuget.org/packages/Microsoft.NETCore.DotNetHost/).
//!
//! [`UnmanagedCallersOnly`]: <https://docs.microsoft.com/en-us/dotnet/api/system.runtime.interopservices.unmanagedcallersonlyattribute>
//! [`AssemblyDelegateLoader`]: crate::hostfxr::AssemblyDelegateLoader
//! [`AssemblyDelegateLoader::get_function_with_default_signature`]: crate::hostfxr::AssemblyDelegateLoader::get_function_with_default_signature
//! [`AssemblyDelegateLoader::get_function`]: crate::hostfxr::AssemblyDelegateLoader::get_function

/// Module for the raw bindings for hostfxr and nethost.
pub mod bindings;

/// Module for abstractions of the hostfxr library.
pub mod hostfxr;

/// Module for abstractions of the nethost library.
#[cfg(feature = "nethost")]
pub mod nethost;

/// Module for a platform dependent c-like string type.
#[allow(missing_docs)]
pub mod pdcstring;

/// Module containing error enums.
pub mod error;

#[doc(hidden)]
pub use hostfxr_sys::dlopen2;
