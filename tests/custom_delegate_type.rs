use netcorehost::{cast_managed_fn, nethost, pdcstr};

#[path = "common.rs"]
mod common;

#[test]
fn hello_world_with_custom_delegate_type() {
    common::setup();

    let hostfxr = nethost::load_hostfxr().unwrap();

    let context = hostfxr
        .initialize_for_runtime_config(pdcstr!(
            "tests/Test/bin/Debug/net5.0/Test.runtimeconfig.json"
        ))
        .unwrap();
    let fn_loader = context
        .get_delegate_loader_for_assembly(pdcstr!("tests/Test/bin/Debug/net5.0/Test.dll"))
        .unwrap();
    let hello = fn_loader
        .get_function_pointer(
            pdcstr!("Test.Program, Test"),
            pdcstr!("CustomHello"),
            pdcstr!("Test.Program+CustomHelloFunc, Test"),
        )
        .unwrap();
    let hello = unsafe { cast_managed_fn!(hello, fn()) };
    hello();
}
