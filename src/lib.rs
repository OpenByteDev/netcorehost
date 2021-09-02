#![feature(
    maybe_uninit_uninit_array,
    maybe_uninit_slice,
    negative_impls,
    try_trait_v2
)]
#![allow(clippy::missing_safety_doc)] // TODO: add safety docs
#![warn(unsafe_op_in_unsafe_fn)]

//! A Rust library for hosting the dotnet core runtime.
//! 
//! It utilizes the dotnet core hosting API to load and execute managed code from withing the current process. 
//! 
//! # Usage
//! ## Running an application
//! The example below will setup the runtime, load `Test.dll` and run its `Main` method:
//! ```rust
//! # use netcorehost::{nethost, pdcstr};
//! # fn test() {
//! let hostfxr = nethost::load_hostfxr().unwrap();
//! let context = hostfxr.initialize_for_dotnet_command_line(pdcstr!("Test.dll")).unwrap();
//! let result = context.run_app();
//! # }
//! ```
//! The full example can be found in [examples/run-app](https://github.com/OpenByteDev/netcorehost/tree/master/examples/run-app).
//! 
//! ## Calling a managed function
//! A function pointer to a managed method can be aquired using an [`AssemblyDelegateLoader`].
//! 
//! ### Using the default signature
//! The default method signature is defined as follows:
//! ```csharp
//! public delegate int ComponentEntryPoint(IntPtr args, int sizeBytes);
//! ```
//! 
//! A method with the default signature (see code below) can be loaded using [`AssemblyDelegateLoader::get_function_pointer_with_default_signature`].
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
//! # use netcorehost::{nethost, pdcstr};
//! # fn test() {
//! let hostfxr = nethost::load_hostfxr().unwrap();
//! let context =
//!     hostfxr.initialize_for_runtime_config(pdcstr!("Test.runtimeconfig.json")).unwrap();
//! let fn_loader =
//!     context.get_delegate_loader_for_assembly(pdcstr!("Test.dll")).unwrap();
//! let hello = fn_loader.get_function_pointer_with_default_signature(
//!     pdcstr!("Test.Program, Test"),
//!     pdcstr!("Hello"),
//! ).unwrap();
//! let result = unsafe { hello(std::ptr::null(), 0) };
//! # }
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
//! # use netcorehost::{nethost, pdcstr};
//! # fn test() {
//! let hostfxr = nethost::load_hostfxr().unwrap();
//! let context =
//!     hostfxr.initialize_for_runtime_config(pdcstr!("Test.runtimeconfig.json")).unwrap();
//! let fn_loader =
//!     context.get_delegate_loader_for_assembly(pdcstr!("Test.dll")).unwrap();
//! let hello = fn_loader.get_function_pointer_with_default_signature(
//!     pdcstr!("Test.Program, Test"),
//!     pdcstr!("UnmanagedHello"),
//! ).unwrap();
//! let hello: unsafe extern "system" fn() = unsafe { std::mem::transmute(hello) };
//! let result = unsafe { hello() };
//! # }
//! ```
//! 
//! 
//! ### Specifying the delegate type
//! Another option is to define a custom delegate type and passing its assembly qualified name to [`AssemblyDelegateLoader::get_function_pointer`].
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
//! # use netcorehost::{nethost, pdcstr};
//! # fn test() {
//! let hostfxr = nethost::load_hostfxr().unwrap();
//! let context =
//!     hostfxr.initialize_for_runtime_config(pdcstr!("Test.runtimeconfig.json")).unwrap();
//! let fn_loader =
//!     context.get_delegate_loader_for_assembly(pdcstr!("Test.dll")).unwrap();
//! let hello = fn_loader.get_function_pointer(
//!     pdcstr!("Test.Program, Test"),
//!     pdcstr!("UnmanagedHello"),
//!     pdcstr!("Test.Program+CustomHelloFunc, Test")
//! ).unwrap();
//! let hello: unsafe extern "system" fn() = unsafe { std::mem::transmute(hello) };
//! let result = unsafe { hello() };
//! # }
//! ```
//! 
//! The full examples can be found in [examples/call-managed-function](https://github.com/OpenByteDev/netcorehost/tree/master/examples/call-managed-function).
//! 
//! ## Passing complex parameters
//! Examples for passing non-primitive parameters can be found in [examples/passing-parameters](https://github.com/OpenByteDev/netcorehost/tree/master/examples/passing-parameters).
//! 
//! [`UnmanagedCallersOnly`]: <https://docs.microsoft.com/en-us/dotnet/api/system.runtime.interopservices.unmanagedcallersonlyattribute>
//! [`AssemblyDelegateLoader`]: crate::hostfxr::AssemblyDelegateLoader
//! [`AssemblyDelegateLoader::get_function_pointer_with_default_signature`]: crate::hostfxr::AssemblyDelegateLoader::get_function_pointer_with_default_signature
//! [`AssemblyDelegateLoader::get_function_pointer`]: crate::hostfxr::AssemblyDelegateLoader::get_function_pointer


#[macro_use]
extern crate dlopen_derive;

/// Module for the raw bindings for hostfxr and nethost.
#[allow(non_camel_case_types, dead_code)]
pub mod bindings;
/// Module for abstractions of the hostfxr library.
pub mod hostfxr;
/// Module for abstractions of the nethost library.
#[cfg(feature = "nethost")]
pub mod nethost;

/// Module for a platform dependent c-like string type.
pub mod pdcstring;
