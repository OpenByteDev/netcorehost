use std::{
    borrow::Cow,
    env,
    error::Error,
    fs::{create_dir_all, File},
    io::{self, Cursor, Read},
    path::Path,
    str::FromStr,
};

use platforms::target::*;
use semver::Version;
use serde::{Deserialize, Serialize};
use zip::ZipArchive;

#[derive(Debug, Serialize, Deserialize)]
struct ResourceIndex<'a> {
    resources: Vec<Resource<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Resource<'a> {
    #[serde(rename = "@id")]
    url: Cow<'a, str>,
    #[serde(rename = "@type")]
    r#type: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PackageInfoIndex<'a> {
    #[serde(rename = "items")]
    pages: Vec<PackageInfoCatalogPage<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PackageInfoCatalogPage<'a> {
    #[serde(rename = "@id")]
    url: Cow<'a, str>,
    lower: Cow<'a, str>,
    upper: Cow<'a, str>,
    items: Vec<PackageInfoCatalogPageEntry<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PackageInfoCatalogPageEntry<'a> {
    #[serde(rename = "catalogEntry")]
    inner: PackageInfoCatalogEntry<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PackageInfoCatalogEntry<'a> {
    #[serde(rename = "@id")]
    url: Cow<'a, str>,
    listed: bool,
    #[serde(rename = "packageContent")]
    content: Cow<'a, str>,
    version: Cow<'a, str>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let out_dir = env::var("OUT_DIR")?;
    let runtimes_dir = Path::new(&out_dir)
        .parent().unwrap()
        .parent().unwrap()
        .parent().unwrap()
        .join("runtimes")
        .join(target);
    if !runtimes_dir.exists() || runtimes_dir.read_dir()?.next().is_none() {
        create_dir_all(&runtimes_dir)?;
        download_nethost(target, &runtimes_dir)?;
    }

    println!("cargo:rustc-link-search={}", runtimes_dir.into_os_string().to_str().unwrap());

    // NOTE: for some reason we need the rustc argument here, but the link attribute in bindings/nethost.rs for unix.
    // For more information see https://github.com/OpenByteDev/netcorehost/issues/2.
    match platforms::TARGET_OS {
        OS::Windows => {
            println!("cargo:rustc-link-lib=libnethost");
        }
        OS::iOS => {
            // untestet
            // println!("cargo:rustc-link-lib=dylib=c++");
            // println!("cargo:rustc-link-lib=static=nethost");
        }
        _ => {
            // println!("cargo:rustc-link-lib=dylib=stdc++");
            // println!("cargo:rustc-link-lib=static=nethost");
        }
    }

    Ok(())
}

fn download_nethost(target: &str, target_path: &Path) -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();

    let index = client
        .get("https://api.nuget.org/v3/index.json")
        .send()
        .expect("Failed to query nuget.org index.")
        .json::<ResourceIndex>()
        .expect("Failed to parse json response from nuget.org2.");
    let registrations_base_url = index
        .resources
        .into_iter()
        .find(|res| res.r#type == "RegistrationsBaseUrl")
        .expect("Unable to find nuget.org query endpoint.")
        .url;

    let package_info = client
        .get(format!(
            "{}runtime.{}.microsoft.netcore.dotnetapphost/index.json",
            registrations_base_url, target
        ))
        .send()
        .expect("Failed to find package on nuget.org.")
        .json::<PackageInfoIndex>()
        .expect("Failed to parse json response from nuget.org.")
        .pages
        .into_iter()
        .max_by_key(|page| Version::from_str(page.upper.as_ref()).unwrap())
        .expect("Unable to find package page.")
        .items
        .into_iter()
        .map(|e| e.inner)
        .filter(|e| e.listed)
        .max_by_key(|e| Version::from_str(e.version.as_ref()).unwrap())
        .unwrap();

    let mut package_content_response = client
        .get(package_info.content.as_ref())
        .send()
        .expect("Failed to download nethost nuget package.");

    let mut buf: Vec<u8> = Vec::new();
    package_content_response.read_to_end(&mut buf)?;

    let reader = Cursor::new(buf);
    let mut archive = ZipArchive::new(reader)?;

    let runtime_dir_path = format!("runtimes/{}/native", target);

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;

        let out_path = match file.enclosed_name() {
            Some(path) => path,
            None => continue,
        };

        if !out_path.starts_with(&runtime_dir_path) {
            continue;
        }

        if let Some(ext) = out_path.extension() {
            if !(ext == "a" || ext == "lib") {
                continue;
            }
        } else {
            continue;
        }

        let mut out_file = File::create(target_path.join(out_path.components().last().unwrap()))?;
        io::copy(&mut file, &mut out_file)?;
    }

    Ok(())
}
