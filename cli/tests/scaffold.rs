use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use toml::Value;

fn directory() -> PathBuf {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let path = env::temp_dir().join(format!(
        "icedcn-cli-{}-{}-{}",
        std::process::id(),
        timestamp.as_nanos(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir(&path).unwrap();
    path
}

#[test]
fn new_generates_a_standalone_git_application() {
    let parent = directory();
    let destination = parent.join("my-app");
    let output = Command::new(env!("CARGO_BIN_EXE_icedcn"))
        .arg("new")
        .arg(&destination)
        .output()
        .unwrap();
    assert!(output.status.success(), "{:?}", output);

    let manifest = fs::read_to_string(destination.join("Cargo.toml")).unwrap();
    let manifest: Value = toml::from_str(&manifest).unwrap();
    assert_eq!(manifest["package"]["name"].as_str(), Some("my-app"));
    assert!(manifest.get("workspace").is_none());
    let dependencies = manifest["dependencies"].as_table().unwrap();
    assert_eq!(
        dependencies["icedcn"]["git"].as_str(),
        Some("https://github.com/airstrike/icedcn")
    );
    assert_eq!(dependencies["icedcn"]["branch"].as_str(), Some("master"));
    for dependency in dependencies.values().filter_map(Value::as_table) {
        assert!(!dependency.contains_key("path"));
        assert!(!dependency.contains_key("workspace"));
    }
    for name in ["iced", "iced_core", "iced_widget", "iced_runtime"] {
        assert_eq!(
            manifest["patch"]["crates-io"][name]["git"].as_str(),
            Some("https://github.com/airstrike/iced.git")
        );
    }
    assert_eq!(
        manifest["patch"]["https://github.com/hecrj/cosmic-text.git"]["cosmic-text"]["git"]
            .as_str(),
        Some("https://github.com/airstrike/cosmic-text")
    );
    let toolchain = fs::read_to_string(destination.join("rust-toolchain.toml")).unwrap();
    let toolchain: Value = toml::from_str(&toolchain).unwrap();
    assert_eq!(
        toolchain["toolchain"]["channel"].as_str(),
        Some("nightly-2026-07-11")
    );
    for file in ["src/main.rs", "src/theme.rs", ".gitignore"] {
        let contents = fs::read_to_string(destination.join(file)).unwrap();
        assert!(!contents.contains("__NAME__"));
    }
    assert!(!destination.join("Cargo.lock").exists());
    fs::remove_dir_all(parent).unwrap();
}

#[test]
fn new_refuses_existing_directories_without_modifying_them() {
    let parent = directory();
    let destination = parent.join("my-app");
    fs::create_dir(&destination).unwrap();
    for populated in [false, true] {
        if populated {
            fs::write(destination.join("keep.txt"), "keep me").unwrap();
        }
        let output = Command::new(env!("CARGO_BIN_EXE_icedcn"))
            .arg("new")
            .arg(&destination)
            .output()
            .unwrap();
        assert!(!output.status.success());
        assert_eq!(
            fs::read_dir(&destination).unwrap().count(),
            usize::from(populated)
        );
        if populated {
            assert_eq!(
                fs::read_to_string(destination.join("keep.txt")).unwrap(),
                "keep me"
            );
        }
    }
    fs::remove_dir_all(parent).unwrap();
}

#[test]
fn new_rejects_invalid_names_before_writing() {
    let parent = directory();
    for name in ["invalid name", "123-app", "injection\"", "test"] {
        let destination = parent.join(name);
        let output = Command::new(env!("CARGO_BIN_EXE_icedcn"))
            .arg("new")
            .arg(&destination)
            .output()
            .unwrap();
        assert!(!output.status.success());
        assert!(!destination.exists());
    }
    fs::remove_dir_all(parent).unwrap();
}

#[test]
fn help_and_version_work_without_a_project() {
    for argument in ["--help", "--version"] {
        let output = Command::new(env!("CARGO_BIN_EXE_icedcn"))
            .arg(argument)
            .output()
            .unwrap();
        assert!(output.status.success());
        assert!(String::from_utf8_lossy(&output.stdout).contains("icedcn"));
    }
}
