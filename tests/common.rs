use std::{path::Path, process::Command};

#[allow(unused)]
pub fn setup() {
    build_test_project();
}

#[allow(unused)]
pub fn build_test_project() {
    if Path::new("tests/Test/bin/Debug/net6.0/Test.runtimeconfig.json").exists() {
        return;
    }

    Command::new("dotnet")
        .arg("build")
        .current_dir("tests/Test")
        .spawn()
        .expect("dotnet build failed")
        .wait()
        .expect("dotnet build failed");
}
