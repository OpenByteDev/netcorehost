use std::process::{Command, Stdio};

pub fn build_example_project(example: &str) {
    let result = Command::new("dotnet")
        .arg("build")
        .current_dir(format!("examples/{}/ExampleProject", example))
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        // .stderr(Stdio::null())
        .spawn()
        .unwrap()
        .wait()
        .expect("dotnet build failed");
    assert!(result.success());
}
