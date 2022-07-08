use netcorehost::{nethost, pdcstr};

#[path = "../helpers/dotnet-build.rs"]
mod dotnet_build;

fn main() {
    dotnet_build::build_example_project("run-app");

    let hostfxr = nethost::load_hostfxr().unwrap();
    let context = hostfxr
        .initialize_for_dotnet_command_line(pdcstr!(
            "examples/run-app/ExampleProject/bin/Debug/net6.0/ExampleProject.dll"
        ))
        .unwrap();
    context.run_app().as_hosting_exit_code().unwrap();
}
