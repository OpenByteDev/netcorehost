use glob::glob;
use std::{env, process::Command};

#[allow(unused)]
pub fn setup() {
    build_test_project();
}

#[allow(unused)]
pub fn build_test_project() {
    if glob("tests/Test/bin/**/Test.runtimeconfig.json")
        .unwrap()
        .next()
        .is_some()
    {
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
