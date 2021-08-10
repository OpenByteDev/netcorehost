# netcorehost

[![Build](https://github.com/OpenByteDev/netcorehost/actions/workflows/build.yml/badge.svg)](https://github.com/OpenByteDev/netcorehost/actions/workflows/build.yml)
[![crates.io](https://img.shields.io/crates/v/netcorehost.svg)](https://crates.io/crates/netcorehost)
[![Documentation](https://docs.rs/netcorehost/badge.svg)](https://docs.rs/netcorehost)
[![dependency status](https://deps.rs/repo/github/openbytedev/netcorehost/status.svg)](https://deps.rs/repo/github/openbytedev/netcorehost)
[![MIT](https://img.shields.io/crates/l/netcorehost.svg)](https://github.com/OpenByteDev/netcorehost/blob/master/LICENSE)

A Rust library for hosting .NET Core applications.

This crate allows a .NET Core app to be run inside the current process or to load it and call a contained method directly.

## Basic Usage
The first step is to load the hostfxr library with handles the hosting. This can be done with `nethost::load_hostfxr()`
which automatically locates the library on your system. The returned instance of `Hostfxr` can then be used to initialize
a new `HostfxrContext` through one of the `initialize_*` methods like `initialize_for_dotnet_command_line`
which in turn be used to run the app associated with the context or to load a pointer to a function od the loaded library.

## Examples
### Running an application
Assuming the following app located inside the current folder and named `Test.dll`:
```cs
public static class Program {
    public static int Main() {
        System.Console.WriteLine("Hello from C#!");
    }
}
```
The following code will setup the hostfxr library, load the app and run its `Main` method.
```rust
let hostfxr = nethost::load_hostfxr()?;
let context = hostfxr.initialize_for_dotnet_command_line(pdcstr!("Test.dll")?)?;
let result = context.run_app();
```
### Calling a managed function
Assuming the following app located inside the current folder and named `Test.dll`:
```cs
namespace Test {
    public static class Program {
        public static int Hello(IntPtr arg, int argLength) {
            return 42;
        }
    }
}
```
The following code will setup the hostfxr library, load the app and call the `Hello` method.
The method has the default signature which avoids having to specify it. It accepts a ptr to some data and the size of said data.
```rust
let hostfxr = nethost::load_hostfxr()?;
let context =
    hostfxr.initialize_for_runtime_config(pdcstr!("Test.runtimeconfig.json")?)?;
let fn_loader =
    context.get_delegate_loader_for_assembly(pdcstr!("Test.dll")?)?;
let hello = fn_loader.get_function_pointer_with_default_signature(
    pdcstr!("Test.Program, Test")?,
    pdcstr!("Hello")?,
);
let result = unsafe { hello(ptr::null(), 0) };
```

Alternatively it is possible to call a method with any signature if it is annotated with `UnmanagedCallersOnly` (loaded with
`get_function_pointer_for_unmanaged_callers_only_method`) or if the signature is passed to `get_function_pointer`.

## Additional Information
- [Hosting layer APIs](https://github.com/dotnet/core-setup/blob/master/Documentation/design-docs/hosting-layer-apis.md)
- [Native hosting](https://github.com/dotnet/core-setup/blob/master/Documentation/design-docs/native-hosting.md#runtime-properties)
- [Write a custom .NET Core host to control the .NET runtime from your native code](https://docs.microsoft.com/en-us/dotnet/core/tutorials/netcore-hosting)

## License
Licensed under MIT license ([LICENSE](https://github.com/OpenByteDev/netcorehost/blob/master/LICENSE) or http://opensource.org/licenses/MIT)
