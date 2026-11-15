use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const CHANNEL: &str = "nightly-2026-07-11";
const TEMPLATE: &str = include_str!("../templates/rust-toolchain.toml");

fn directory() -> PathBuf {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let path = env::temp_dir().join(format!(
        "icedcn-toolchain-{}-{}-{}",
        std::process::id(),
        timestamp.as_nanos(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir(&path).unwrap();
    fs::write(path.join("Cargo.toml"), "[workspace]\n").unwrap();
    path
}

fn run(directory: &Path, update: bool) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_icedcn"));
    command.arg("toolchain").arg(directory);
    if update {
        command.arg("--update");
    }
    command.output().unwrap()
}

#[test]
fn missing_file_is_created_in_the_default_directory() {
    let directory = directory();
    let output = Command::new(env!("CARGO_BIN_EXE_icedcn"))
        .arg("toolchain")
        .current_dir(&directory)
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        fs::read_to_string(directory.join("rust-toolchain.toml")).unwrap(),
        TEMPLATE
    );
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn a_directory_without_a_manifest_is_not_modified() {
    let directory = directory();
    fs::remove_file(directory.join("Cargo.toml")).unwrap();
    let output = run(&directory, false);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("Cargo.toml"));
    assert_eq!(fs::read_dir(&directory).unwrap().count(), 0);
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn matching_channel_leaves_the_file_byte_identical_even_with_update() {
    let directory = directory();
    let path = directory.join("rust-toolchain.toml");
    let source = format!(
        "# Keep this layout\r\n[toolchain]\r\nchannel = '{CHANNEL}' # pinned\r\ncomponents = ['rustfmt']\r\n"
    );
    fs::write(&path, &source).unwrap();
    for update in [false, true] {
        let output = run(&directory, update);
        assert!(output.status.success(), "{output:?}");
        assert_eq!(fs::read_to_string(&path).unwrap(), source);
    }
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn conflicting_channel_reports_both_versions_without_writing() {
    let directory = directory();
    let path = directory.join("rust-toolchain.toml");
    let source = "[toolchain]\nchannel = 'stable'\n";
    fs::write(&path, source).unwrap();
    let output = run(&directory, false);
    assert!(!output.status.success());
    let error = String::from_utf8_lossy(&output.stderr);
    for expected in ["stable", CHANNEL, "--update", "unchanged"] {
        assert!(error.contains(expected), "{error}");
    }
    assert_eq!(fs::read_to_string(&path).unwrap(), source);
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn update_changes_only_the_channel_and_preserves_configuration_and_comments() {
    let directory = directory();
    let path = directory.join("rust-toolchain.toml");
    let source = "# Project configuration\n[toolchain]\nchannel = \"stable\" # Keep this comment\nprofile = 'minimal'\ncomponents = ['rustfmt', 'clippy']\ntargets = ['wasm32-unknown-unknown']\n";
    fs::write(&path, source).unwrap();
    let output = run(&directory, true);
    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        fs::read_to_string(&path).unwrap(),
        source.replace("\"stable\"", &format!("\"{CHANNEL}\""))
    );
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn malformed_or_missing_channel_is_never_overwritten() {
    let directory = directory();
    let path = directory.join("rust-toolchain.toml");
    for source in [
        "[toolchain\n",
        "[toolchain]\nchannel = 7\n",
        "[toolchain]\npath = '/custom/compiler'\n",
    ] {
        fs::write(&path, source).unwrap();
        for update in [false, true] {
            let output = run(&directory, update);
            assert!(!output.status.success(), "{output:?}");
            assert_eq!(fs::read_to_string(&path).unwrap(), source);
        }
    }
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn legacy_toolchain_blocks_creation_and_updates() {
    let directory = directory();
    let legacy = directory.join("rust-toolchain");
    let path = directory.join("rust-toolchain.toml");
    fs::write(&legacy, "stable\n").unwrap();
    for exists in [false, true] {
        if exists {
            fs::write(&path, TEMPLATE).unwrap();
        }
        for update in [false, true] {
            let output = run(&directory, update);
            assert!(!output.status.success());
            assert!(String::from_utf8_lossy(&output.stderr).contains("legacy"));
            assert_eq!(path.exists(), exists);
            assert_eq!(fs::read_to_string(&legacy).unwrap(), "stable\n");
            if exists {
                assert_eq!(fs::read_to_string(&path).unwrap(), TEMPLATE);
            }
        }
    }
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn directory_at_the_toolchain_path_is_rejected() {
    let directory = directory();
    let path = directory.join("rust-toolchain.toml");
    fs::create_dir(&path).unwrap();
    assert!(!run(&directory, true).status.success());
    assert!(path.is_dir());
    fs::remove_dir_all(directory).unwrap();
}

#[cfg(unix)]
#[test]
fn toolchain_symlinks_are_rejected_even_when_dangling() {
    use std::os::unix::fs::symlink;

    let directory = directory();
    let target = directory.join("shared.toml");
    let path = directory.join("rust-toolchain.toml");
    symlink(&target, &path).unwrap();
    assert!(!run(&directory, true).status.success());
    assert!(!target.exists());
    fs::write(&target, "[toolchain]\nchannel = 'stable'\n").unwrap();
    assert!(!run(&directory, true).status.success());
    assert_eq!(
        fs::read_to_string(&target).unwrap(),
        "[toolchain]\nchannel = 'stable'\n"
    );
    assert!(
        fs::symlink_metadata(&path)
            .unwrap()
            .file_type()
            .is_symlink()
    );
    fs::remove_dir_all(directory).unwrap();
}
