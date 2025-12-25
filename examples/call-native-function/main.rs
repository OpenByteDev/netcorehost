// Note that this example requires the unstable rustc flag "-Z export-executable-symbols"

use netcorehost::{nethost, pdcstr};

#[no_mangle]
pub extern "system" fn rusty_increment(n: i32) -> i32 {
    println!("Called rusty increment with {n}");
    n + 1
}

fn main() {
    let hostfxr = nethost::load_hostfxr().unwrap();
    let context = hostfxr.initialize_for_runtime_config(pdcstr!("examples/call-native-function/ExampleProject/bin/Debug/net10.0/ExampleProject.runtimeconfig.json")).unwrap();
    let delegate_loader = context
        .get_delegate_loader_for_assembly(pdcstr!(
            "examples/call-native-function/ExampleProject/bin/Debug/net10.0/ExampleProject.dll"
        ))
        .unwrap();

    let increment1 = delegate_loader
        .get_function_with_unmanaged_callers_only::<fn(i32) -> i32>(
            pdcstr!("ExampleProject.Program, ExampleProject"),
            pdcstr!("IndirectIncrement1"),
        )
        .unwrap();
    #[cfg(windows)]
    let increment2 = delegate_loader
        .get_function_with_unmanaged_callers_only::<fn(i32) -> i32>(
            pdcstr!("ExampleProject.Program, ExampleProject"),
            pdcstr!("IndirectIncrement2"),
        )
        .unwrap();

    assert_eq!(2, increment1(1));
    #[cfg(windows)]
    assert_eq!(3, increment2(2));
}
