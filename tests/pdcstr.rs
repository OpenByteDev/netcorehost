use std::{fs, process::Command};

#[test]
fn try_build() {
    // as different macros are used depending on the os -> copy the correct contents to the .stderr fil.
    let family = if cfg!(windows) { "windows" } else { "other" };
    fs::copy(
        format!(
            "tests/macro-build-tests/pdcstr-compile-fail.{}.stderr",
            family
        ),
        "tests/macro-build-tests/pdcstr-compile-fail.stderr",
    )
    .unwrap();

    let t = trybuild::TestCases::new();
    t.pass("tests/macro-build-tests/pdcstr-pass.rs");
    t.compile_fail("tests/macro-build-tests/pdcstr-compile-fail.rs");
}

#[test]
fn correct_reexports() {
    // check that macro dependencies are correctly exported and do not need to be manually referenced by the consuming crate.
    let exit_status = Command::new("cargo")
        .arg("build")
        .arg("--target")
        .arg(current_platform::CURRENT_PLATFORM)
        .current_dir("tests/macro-test-crate")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    assert!(exit_status.success());
}
