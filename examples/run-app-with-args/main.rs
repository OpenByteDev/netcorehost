use std::{env, iter};

use netcorehost::{nethost, pdcstr, pdcstring::PdCString};

fn main() {
    let hostfxr = nethost::load_hostfxr().unwrap();

    let program_args = env::args()
        .skip(1) // skip rust host program name
        .map(|arg| PdCString::from_os_str(arg).unwrap())
        .collect::<Vec<_>>();
    let borrowed_args = program_args.iter().map(|arg| arg.as_ref());
    let app_and_args = iter::once(pdcstr!(
        "examples/run-app-with-args/ExampleProject/bin/Debug/net6.0/ExampleProject.dll"
    ))
    .chain(borrowed_args)
    .collect::<Vec<_>>();

    let context = hostfxr
        .initialize_for_dotnet_command_line_with_args(&app_and_args)
        .unwrap();

    let result = context.run_app().value();
    println!("Exit code: {}", result);
}
