use netcorehost::{nethost, pdcstr};

fn main() {
    let hostfxr = nethost::load_hostfxr().unwrap();
    let context = hostfxr.initialize_for_runtime_config(pdcstr!("examples/call-managed-function/ExampleProject/bin/Debug/net8.0/ExampleProject.runtimeconfig.json")).unwrap();
    let delegate_loader = context
        .get_delegate_loader_for_assembly(pdcstr!(
            "examples/call-managed-function/ExampleProject/bin/Debug/net8.0/ExampleProject.dll"
        ))
        .unwrap();

    let hello_world1 = delegate_loader
        .get_function::<fn()>(
            pdcstr!("ExampleProject.Program, ExampleProject"),
            pdcstr!("HelloWorld1"),
            pdcstr!("ExampleProject.Program+HelloWorld1Delegate, ExampleProject"),
        )
        .unwrap();
    hello_world1();

    let hello_world2 = delegate_loader
        .get_function_with_unmanaged_callers_only::<fn()>(
            pdcstr!("ExampleProject.Program, ExampleProject"),
            pdcstr!("HelloWorld2"),
        )
        .unwrap();
    hello_world2();

    let hello_world3 = delegate_loader
        .get_function_with_default_signature(
            pdcstr!("ExampleProject.Program, ExampleProject"),
            pdcstr!("HelloWorld3"),
        )
        .unwrap();
    unsafe { hello_world3(std::ptr::null(), 0) };
}
