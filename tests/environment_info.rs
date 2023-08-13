use netcorehost::{
    hostfxr::{EnvironmentInfo, FrameworkInfo, SdkInfo},
    nethost,
};
use std::{collections::HashMap, path::PathBuf, process::Command};

#[path = "common.rs"]
mod common;

#[test]
#[cfg(feature = "net6_0")]
fn get_dotnet_environment_info() {
    let hostfxr = nethost::load_hostfxr().unwrap();

    let actual_env = hostfxr.get_dotnet_environment_info().unwrap();
    let expected_env = get_expected_environment_info();

    assert_eq!(expected_env.hostfxr_version, actual_env.hostfxr_version);
    assert_eq!(expected_env.sdks, actual_env.sdks);
    assert_eq!(expected_env.frameworks, actual_env.frameworks);
}

fn get_expected_environment_info() -> EnvironmentInfo {
    let output = Command::new("dotnet").arg("--info").output().unwrap();
    assert!(output.status.success());
    let output = String::from_utf8_lossy(&output.stdout);

    let mut sections = Vec::new();
    let mut current_section = None;
    for line in output.lines() {
        if line.is_empty() {
            if let Some(section) = current_section.take() {
                sections.push(section);
            }
            continue;
        }

        match &mut current_section {
            None => current_section = Some((line.trim().trim_end_matches(':'), Vec::new())),
            Some((_header, content)) => {
                content.push(line.trim());
            }
        }
    }

    let host_section_content = sections
        .iter()
        .find(|(header, _content)| *header == "Host")
        .map(|(_header, content)| content)
        .unwrap();
    let host_info = host_section_content
        .iter()
        .map(|line| {
            let (key, value) = line.split_once(':').unwrap();
            (key.trim(), value.trim())
        })
        .collect::<HashMap<_, _>>();
    let hostfxr_version = host_info["Version"].to_string();
    let hostfxr_commit_hash = host_info["Commit"].to_string();

    let sdk_section_content = sections
        .iter()
        .find(|(header, _content)| *header == ".NET SDKs installed")
        .map(|(_header, content)| content)
        .unwrap();
    let sdks = sdk_section_content
        .iter()
        .map(|line| {
            let (version, enclosed_path) = line.split_once(' ').unwrap();
            let path = enclosed_path.trim_start_matches('[').trim_end_matches(']');
            let version = version.to_string();
            let mut path = PathBuf::from(path);
            path.push(&version);
            SdkInfo { version, path }
        })
        .collect::<Vec<_>>();

    let framework_section_content = sections
        .iter()
        .find(|(header, _content)| *header == ".NET runtimes installed")
        .map(|(_header, content)| content)
        .unwrap();
    let frameworks = framework_section_content
        .iter()
        .map(|line| {
            let mut items = line.splitn(3, ' ');
            let name = items.next().unwrap();
            let version = items.next().unwrap();
            let enclosed_path = items.next().unwrap();
            assert_eq!(items.next(), None);

            let name = name.to_string();
            let path = PathBuf::from(enclosed_path.trim_start_matches('[').trim_end_matches(']'));
            let version = version.to_string();
            FrameworkInfo {
                name,
                version,
                path,
            }
        })
        .collect::<Vec<_>>();

    EnvironmentInfo {
        hostfxr_version,
        hostfxr_commit_hash,
        sdks,
        frameworks,
    }
}
