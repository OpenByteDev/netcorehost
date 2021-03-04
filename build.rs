use std::{env, fs, path::PathBuf};

fn main() {
    if !cfg!(feature = "nethost_bin") {
        return;
    }

    let dll_src = if cfg!(all(target_os = "windows", target_arch = "x86_32")) {
        Some("src\\runtimes\\win-x86\\native\\nethost.dll")
    } else if cfg!(all(target_os = "windows", target_arch = "x86_64")) {
        Some("src\\runtimes\\win-x64\\native\\nethost.dll")
    } else {
        None
    };
    if dll_src.is_none() {
        return;
    }
    let dll_src = dll_src.unwrap();
    let target_path = find_cargo_target_dir();

    let dll_dest_path = target_path.join("nethost.dll");
    fs::copy(dll_src, dll_dest_path).unwrap();

    // println!("cargo:rustc-link-lib=dylib=nethost");
    // println!("cargo:rustc-link-search=native={}", "E:\\Code\\.NET\\Setsuju​\\netcorehost-rs\\src\\runtimes\\win-x64\\native");
}

// from https://github.com/Rust-SDL2/rust-sdl2/blob/84eff3ce1c8dcd34d62f5cc5d44842ba4119e46e/sdl2-sys/build.rs
fn find_cargo_target_dir() -> PathBuf {
    // Infer the top level cargo target dir from the OUT_DIR by searching
    // upwards until we get to $CARGO_TARGET_DIR/build/ (which is always one
    // level up from the deepest directory containing our package name)
    let pkg_name = env::var("CARGO_PKG_NAME").unwrap();
    let mut out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    loop {
        {
            let final_path_segment = out_dir.file_name().unwrap();
            if final_path_segment.to_string_lossy().contains(&pkg_name) {
                break;
            }
        }
        if !out_dir.pop() {
            panic!("Malformed build path: {}", out_dir.to_string_lossy());
        }
    }
    out_dir.pop();
    out_dir.pop();
    out_dir
}
