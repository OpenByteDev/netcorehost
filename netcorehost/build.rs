use platforms::target::*;

fn main() {
    let target = match (
        platforms::TARGET_OS,
        platforms::TARGET_ARCH,
        platforms::TARGET_ENV,
    ) {
        (OS::Windows, Arch::X86, _) => "win-x86",
        (OS::Windows, Arch::X86_64, _) => "win-x64",
        (OS::Windows, Arch::ARM, _) => "win-arm",
        (OS::Windows, Arch::AARCH64, _) => "win-arm64",
        (OS::Linux, Arch::X86_64, Some(Env::Musl)) => "linux-musl-x64",
        (OS::Linux, Arch::ARM, Some(Env::Musl)) => "linux-musl-arm",
        (OS::Linux, Arch::AARCH64, Some(Env::Musl)) => "linux-musl-arm64",
        (OS::Linux, Arch::X86_64, _) => "linux-x64",
        (OS::Linux, Arch::ARM, _) => "linux-arm",
        (OS::Linux, Arch::AARCH64, _) => "linux-arm64",
        (OS::iOS, Arch::X86_64, _) => "osx-x64",
        _ => panic!("platform not supported."),
    };

    println!("cargo:rustc-link-search=runtimes\\{}", target);
    println!("cargo:rustc-link-lib=libnethost");
}
