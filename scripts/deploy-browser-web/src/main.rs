// rawr
//! Deploy a build-browser-web.py output: the wasm to R2, then the Worker and
//! its assets. From anywhere in the checkout:
//!
//! ```sh
//! rawr run scripts/deploy-browser-web            # deploy
//! rawr run scripts/deploy-browser-web -- --dry-run
//! ```

mod sys;

const OUTPUT: &str = "target/browser-web";
const CONFIG: &str = "examples/browser/wrangler.jsonc";
const BUCKET: &str = "icedcn-wasm";
const WRANGLER: [&str; 3] = ["npx", "--yes", "wrangler@4.69.0"];

fn main() {
    let dry_run = std::env::args().any(|arg| arg == "--dry-run");
    if let Err(error) = sys::enter_workspace() {
        fail(&error);
    }
    if !sys::is_file(CONFIG) {
        fail(&format!("{CONFIG} is missing from the workspace root."));
    }
    // Upload the module the pages fetch, as the build wrote it into the shell.
    let shell = std::fs::read_to_string(format!("{OUTPUT}/index.html"))
        .unwrap_or_else(|_| fail("No web build found; run scripts/build-browser-web.py."));
    let wasm = shell
        .split("fetch(\"/")
        .nth(1)
        .and_then(|rest| rest.split('"').next())
        .filter(|name| name.starts_with("browser-") && name.ends_with(".wasm"))
        .unwrap_or_else(|| fail("The page shell names no browser wasm."));
    let file = format!("{OUTPUT}/{wasm}");
    if !sys::is_file(&file) {
        fail(&format!("{file} is missing; rebuild the web output."));
    }
    let object = format!("{BUCKET}/{wasm}");
    // Hashed names never change, so an upload must land before the pages that load it.
    wrangler(
        &[
            "r2", "object", "put", &object, "--file", &file,
            "--content-type", "application/wasm", "--remote", "--config", CONFIG,
        ],
        dry_run,
    );
    wrangler(&["deploy", "--config", CONFIG], dry_run);
}

fn wrangler(args: &[&str], dry_run: bool) {
    let mut command: Vec<String> = WRANGLER[1..].iter().map(|arg| arg.to_string()).collect();
    command.extend(args.iter().map(|arg| arg.to_string()));
    if dry_run {
        println!("{} {}", WRANGLER[0], command.join(" "));
    } else if let Err(error) = sys::run(WRANGLER[0], &command) {
        fail(&error);
    }
}

fn fail(message: &str) -> ! {
    eprintln!("{message}");
    std::process::exit(1);
}
