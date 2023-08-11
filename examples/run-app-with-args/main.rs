use std::env;

use netcorehost::{nethost, pdcstr, pdcstring::PdCString};

fn main() {
    let hostfxr = nethost::load_hostfxr().unwrap();

    let args = env::args()
        .skip(1) // skip rust host program name
        .map(|arg| PdCString::from_os_str(arg).unwrap());

    let context = hostfxr
        .initialize_for_dotnet_command_line_with_args(
            pdcstr!(
                "examples/run-app-with-args/ExampleProject/bin/Debug/net6.0/ExampleProject.dll"
            ),
            args,
        )
        .unwrap();

    let result = context.run_app().value();
    println!("Exit code: {}", result);
}
