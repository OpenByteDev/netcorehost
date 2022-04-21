use netcorehost::{hostfxr::AssemblyDelegateLoader, nethost, pdcstr, pdcstring::PdCStr};

#[path = "../helpers/dotnet-build.rs"]
mod dotnet_build;

fn main() {
    dotnet_build::build_example_project("passing-parameters");

    let hostfxr = nethost::load_hostfxr().unwrap();
    let context = hostfxr.initialize_for_runtime_config(pdcstr!("examples/passing-parameters/ExampleProject/bin/Debug/net6.0/ExampleProject.runtimeconfig.json")).unwrap();
    let delegate_loader = context
        .get_delegate_loader_for_assembly(pdcstr!(
            "examples/passing-parameters/ExampleProject/bin/Debug/net6.0/ExampleProject.dll"
        ))
        .unwrap();

    print_utf8_example(&delegate_loader);
    print_utf16_example(&delegate_loader);
    is_palindrom_example(&delegate_loader);
    get_length_example(&delegate_loader);
}

fn print_utf8_example<A: AsRef<PdCStr>>(delegate_loader: &AssemblyDelegateLoader<A>) {
    let print_utf8 = delegate_loader
        .get_function_with_unmanaged_callers_only::<fn(text_ptr: *const u8, text_length: i32)>(
            pdcstr!("ExampleProject.Program, ExampleProject"),
            pdcstr!("PrintUtf8"),
        )
        .unwrap();
    let test_string = "Hello World!";
    print_utf8(test_string.as_ptr(), test_string.len() as i32);
}

fn print_utf16_example<A: AsRef<PdCStr>>(delegate_loader: &AssemblyDelegateLoader<A>) {
    let print_utf16 = delegate_loader
        .get_function_with_unmanaged_callers_only::<fn(text_ptr: *const u16, text_length: i32)>(
            pdcstr!("ExampleProject.Program, ExampleProject"),
            pdcstr!("PrintUtf16"),
        )
        .unwrap();
    let test_string = widestring::U16String::from_str("Hello World!");
    print_utf16(test_string.as_ptr(), test_string.len() as i32);
}

fn is_palindrom_example<A: AsRef<PdCStr>>(delegate_loader: &AssemblyDelegateLoader<A>) {
    let is_palindrom = delegate_loader
        .get_function_with_unmanaged_callers_only::<fn(text_ptr: *const u16, text_length: i32) -> i32>(
            pdcstr!("ExampleProject.Program, ExampleProject"),
            pdcstr!("IsPalindrom"),
        )
        .unwrap();
    for s in ["Racecar", "stats", "hello", "test"].iter() {
        let widestring = widestring::U16String::from_str(s);
        let palindrom_answer = if is_palindrom(widestring.as_ptr(), widestring.len() as i32) != 0 {
            "Yes"
        } else {
            "No"
        };
        println!("Is '{}' a palindrom? {}!", s, palindrom_answer);
    }
}

fn get_length_example<A: AsRef<PdCStr>>(delegate_loader: &AssemblyDelegateLoader<A>) {
    let get_length = delegate_loader
        .get_function_with_unmanaged_callers_only::<fn(text_ptr: *const Vector2f) -> f32>(
            pdcstr!("ExampleProject.Program, ExampleProject"),
            pdcstr!("GetLength"),
        )
        .unwrap();
    let vec = Vector2f {
        x: 3.0f32,
        y: 4.0f32,
    };
    let length = get_length(&vec);
    println!("The length of {:?} is {:?}", vec, length);
}

#[derive(Debug)]
#[repr(C)]
struct Vector2f {
    x: f32,
    y: f32,
}
