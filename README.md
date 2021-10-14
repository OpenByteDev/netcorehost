# netcorehost

[![CI](https://github.com/OpenByteDev/netcorehost/actions/workflows/ci.yml/badge.svg)](https://github.com/OpenByteDev/netcorehost/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/netcorehost.svg)](https://crates.io/crates/netcorehost)
[![Documentation](https://docs.rs/netcorehost/badge.svg)](https://docs.rs/netcorehost)
[![dependency status](https://deps.rs/repo/github/openbytedev/netcorehost/status.svg)](https://deps.rs/repo/github/openbytedev/netcorehost)
[![MIT](https://img.shields.io/crates/l/netcorehost.svg)](https://github.com/OpenByteDev/netcorehost/blob/master/LICENSE)

<!-- cargo-sync-readme start -->

A Rust library for hosting the .NET Core runtime.

It utilizes the .NET Core hosting API to load and execute managed code from withing the current process. 

## Usage
### Running an application
The example below will setup the runtime, load `Test.dll` and run its `Main` method:
```rust
let hostfxr = nethost::load_hostfxr().unwrap();
let context = hostfxr.initialize_for_dotnet_command_line(pdcstr!("Test.dll")).unwrap();
let result = context.run_app().value();
```
The full example can be found in [examples/run-app](https://github.com/OpenByteDev/netcorehost/tree/master/examples/run-app).

### Calling a managed function
A function pointer to a managed method can be aquired using an [`AssemblyDelegateLoader`](https://docs.rs/netcorehost/*/netcorehost/hostfxr/struct.AssemblyDelegateLoader.html).
This is only supported for [`HostfxrContext`'s](https://docs.rs/netcorehost/*/netcorehost/hostfxr/struct.HostfxrContext.html) that are initialized using [`Hostfxr::initialize_for_runtime_config`](https://docs.rs/netcorehost/*/netcorehost/hostfxr/struct.Hostfxr.html#method.initialize_for_runtime_config). The [`runtimeconfig.json`](https://docs.microsoft.com/en-us/dotnet/core/run-time-config/) is automatically generated for executables, for libraries it is neccessary to add  `<GenerateRuntimeConfigurationFiles>True</GenerateRuntimeConfigurationFiles>` to the projects `.csproj` file.

#### Using the default signature
The default method signature is defined as follows:
```csharp
public delegate int ComponentEntryPoint(IntPtr args, int sizeBytes);
```

A method with the default signature (see code below) can be loaded using [`AssemblyDelegateLoader::get_function_pointer_with_default_signature`](https://docs.rs/netcorehost/*/netcorehost/hostfxr/struct.AssemblyDelegateLoader.html#method.get_function_pointer_with_default_signature).

**C#**
```cs
using System;

namespace Test {
    public static class Program {
        public static int Hello(IntPtr args, int sizeBytes) {
            Console.WriteLine("Hello from C#!");
            return 42;
        }
    }
}
```

**Rust**
```rust
let hostfxr = nethost::load_hostfxr().unwrap();
let context =
    hostfxr.initialize_for_runtime_config(pdcstr!("Test.runtimeconfig.json")).unwrap();
let fn_loader =
    context.get_delegate_loader_for_assembly(pdcstr!("Test.dll")).unwrap();
let hello = fn_loader.get_function_pointer_with_default_signature(
    pdcstr!("Test.Program, Test"),
    pdcstr!("Hello"),
).unwrap();
let result = unsafe { hello(std::ptr::null(), 0) };
```

#### Using UnmanagedCallersOnly
A function pointer to a method annotated with [`UnmanagedCallersOnly`](https://docs.microsoft.com/en-us/dotnet/api/system.runtime.interopservices.unmanagedcallersonlyattribute) can be loaded without specifying its signature (as these methods cannot be overloaded).

**C#**
```cs
using System;
using System.Runtime.InteropServices;

namespace Test {
    public static class Program {
        [UnmanagedCallersOnly]
        public static void UnmanagedHello() {
            Console.WriteLine("Hello from C#!");
        }
    }
}
```

**Rust**
```rust
let hostfxr = nethost::load_hostfxr().unwrap();
let context =
    hostfxr.initialize_for_runtime_config(pdcstr!("Test.runtimeconfig.json")).unwrap();
let fn_loader =
    context.get_delegate_loader_for_assembly(pdcstr!("Test.dll")).unwrap();
let hello = fn_loader.get_function_pointer_with_default_signature(
    pdcstr!("Test.Program, Test"),
    pdcstr!("UnmanagedHello"),
).unwrap();
let hello = unsafe { cast_managed_fn!(hello, fn()) };
hello(); // prints "Hello from C#!"
```


#### Specifying the delegate type
Another option is to define a custom delegate type and passing its assembly qualified name to [`AssemblyDelegateLoader::get_function_pointer`](https://docs.rs/netcorehost/*/netcorehost/hostfxr/struct.AssemblyDelegateLoader.html#method.get_function_pointer).

**C#**
```cs
using System;

namespace Test {
    public static class Program {
        public delegate void CustomHelloFunc();
    
        public static void CustomHello() {
            Console.WriteLine("Hello from C#!");
        }
    }
}
```

**Rust**
```rust
let hostfxr = nethost::load_hostfxr().unwrap();
let context =
    hostfxr.initialize_for_runtime_config(pdcstr!("Test.runtimeconfig.json")).unwrap();
let fn_loader =
    context.get_delegate_loader_for_assembly(pdcstr!("Test.dll")).unwrap();
let hello = fn_loader.get_function_pointer(
    pdcstr!("Test.Program, Test"),
    pdcstr!("CustomHello"),
    pdcstr!("Test.Program+CustomHelloFunc, Test")
).unwrap();
let hello = unsafe { cast_managed_fn!(hello, fn()) };
hello(); // prints "Hello from C#!"
```

The full examples can be found in [examples/call-managed-function](https://github.com/OpenByteDev/netcorehost/tree/master/examples/call-managed-function).

### Passing complex parameters
Examples for passing non-primitive parameters can be found in [examples/passing-parameters](https://github.com/OpenByteDev/netcorehost/tree/master/examples/passing-parameters).

## Features
- `nethost` - Links against nethost and allows for automatic detection of the hostfxr library.
- `download-nethost` - Automatically downloads the latest nethost binary from [NuGet](https://www.nuget.org/packages/Microsoft.NETCore.DotNetHost/).

<!-- cargo-sync-readme end -->


## Related crates
- [nethost-sys](https://crates.io/crates/nethost-sys) - bindings for the nethost library.
- [hostfxr-sys](https://crates.io/crates/hostfxr-sys) - bindings for the hostfxr library.
- [coreclr-hosting-shared](https://crates.io/crates/coreclr-hosting-shared) - shared bindings between [hostfxr-sys](https://crates.io/crates/hostfxr-sys) and [nethost-sys](https://crates.io/crates/nethost-sys).

## Additional Information
- [Hosting layer APIs](https://github.com/dotnet/core-setup/blob/master/Documentation/design-docs/hosting-layer-apis.md)
- [Native hosting](https://github.com/dotnet/core-setup/blob/master/Documentation/design-docs/native-hosting.md#runtime-properties)
- [Write a custom .NET Core host to control the .NET runtime from your native code](https://docs.microsoft.com/en-us/dotnet/core/tutorials/netcore-hosting)

## License
Licensed under the MIT license ([LICENSE](https://github.com/OpenByteDev/netcorehost/blob/master/LICENSE) or http://opensource.org/licenses/MIT)
