use netcorehost::{nethost, pdcstr, cast_managed_fn};

#[path = "../helpers/dotnet-build.rs"]
mod dotnet_build;

fn main() {
    dotnet_build::build_example_project("call-managed-function");

    let hostfxr = nethost::load_hostfxr().unwrap();
    let context = hostfxr.initialize_for_runtime_config(pdcstr!("examples/call-managed-function/ExampleProject/bin/Debug/net5.0/ExampleProject.runtimeconfig.json")).unwrap();
    let delegate_loader = context
        .get_delegate_loader_for_assembly(pdcstr!(
            "examples/call-managed-function/ExampleProject/bin/Debug/net5.0/ExampleProject.dll"
        ))
        .unwrap();

    let hello_world1 = delegate_loader
        .get_function_pointer(
            pdcstr!("ExampleProject.Program, ExampleProject"),
            pdcstr!("HelloWorld1"),
            pdcstr!("ExampleProject.Program+HelloWorld1Delegate, ExampleProject"),
        )
        .unwrap();
    let hello_world1 = unsafe { cast_managed_fn!(hello_world1, fn()) };
    hello_world1();

    let hello_world2 = delegate_loader
        .get_function_pointer_for_unmanaged_callers_only_method(
            pdcstr!("ExampleProject.Program, ExampleProject"),
            pdcstr!("HelloWorld2"),
        )
        .unwrap();
    let hello_world2 = unsafe { cast_managed_fn!(hello_world2, fn()) };
    hello_world2();

    let hello_world3 = delegate_loader
        .get_function_pointer_with_default_signature(
            pdcstr!("ExampleProject.Program, ExampleProject"),
            pdcstr!("HelloWorld3"),
        )
        .unwrap();
    unsafe { hello_world3(std::ptr::null(), 0) };
}
