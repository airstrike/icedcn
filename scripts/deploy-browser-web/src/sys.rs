//! Process and filesystem calls rawr's interpreter doesn't run yet
//! (rawr #2082, #646); Cargo compiles this module natively.

use std::path::Path;
use std::process::Command;

/// Run `program` with `args` from the current directory, inheriting its
/// output, and fail unless it exits successfully.
pub fn run(program: &str, args: &[String]) -> Result<(), String> {
    let status = Command::new(program)
        .args(args)
        .status()
        .map_err(|error| format!("could not run {program}: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("{program} {} failed: {status}", args.join(" ")))
    }
}

/// Whether `path` names an existing file.
pub fn is_file(path: &str) -> bool {
    std::fs::metadata(path).is_ok_and(|metadata| metadata.is_file())
}

/// Change to the icedcn workspace root, two levels above this crate, so
/// paths resolve wherever the script was started.
pub fn enter_workspace() -> Result<(), String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    std::env::set_current_dir(&root)
        .map_err(|error| format!("could not enter {}: {error}", root.display()))
}
