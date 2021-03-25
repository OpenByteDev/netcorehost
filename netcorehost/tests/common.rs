use std::{path::Path, process::Command};

pub fn setup() {
    build_test_project();
}

pub fn build_test_project() {
    if Path::new("tests/Test/bin/").exists() {
        return;
    }
    
    Command::new("dotnet")
        .arg("build")
        .arg("tests/Test")
        .spawn()
        .expect("dotnet build failed")
        .wait()
        .unwrap();
}
