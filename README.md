# netcorehost

[![Build](https://github.com/OpenByteDev/netcorehost/actions/workflows/build.yml/badge.svg)](https://github.com/OpenByteDev/netcorehost/actions/workflows/build.yml)
[![crates.io](https://img.shields.io/crates/v/netcorehost.svg)](https://crates.io/crates/netcorehost)
[![Documentation](https://docs.rs/netcorehost/badge.svg)](https://docs.rs/netcorehost)
[![dependency status](https://deps.rs/repo/github/openbytedev/netcorehost/status.svg)](https://deps.rs/repo/github/openbytedev/netcorehost)
[![MIT](https://img.shields.io/crates/l/netcorehost.svg)](https://github.com/OpenByteDev/netcorehost/blob/master/LICENSE)

A .NET Core hosting library written in Rust.

## Usage

Running an app:

```rust
use netcorehost::nethost;
use widestring::WideCString;

fn run_app() {
    let hostfxr = netcorehost::nethost::load_hostfxr().unwrap();
    
    let assembly_path = WideCString::from_str("Test.dll").unwrap();
    let args = [assembly_path.borrow()]; // first argument is the app path
    let context = hostfxr.initialize_for_dotnet_command_line(&args).unwrap();
    let result = context.run_app().unwrap();
}
```


Getting a function pointer to call a managed method:

```rust
use netcorehost::nethost;
use widestring::WideCString;

fn hello_world() {
    let hostfxr = netcorehost::nethost::load_hostfxr().unwrap();

    let context = hostfxr.initialize_for_runtime_config(
        &WideCString::from_str("Test.runtimeconfig.json").unwrap()
    ).unwrap();
    let fn_loader = context.get_delegate_loader_for_assembly(
        &WideCString::from_str("Test.dll").unwrap()
    ).unwrap();
    let hello = fn_loader.get_function_pointer_with_default_signature(
        &WideCString::from_str("Test.Program, Test").unwrap(),
        &WideCString::from_str("Hello").unwrap(),
    );
    let result = unsafe { hello(ptr::null(), 0) };
}
```


## License
Licensed under MIT license ([LICENSE](https://github.com/OpenByteDev/netcorehost/blob/master/LICENSE) or http://opensource.org/licenses/MIT)
