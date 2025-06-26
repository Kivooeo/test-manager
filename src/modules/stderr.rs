use std::fs;
use std::path::Path;
use std::process::Command;

// This is second module
// And it is needed because sometimes
// Tests are failed, and have some stderr file
// (yes, sometimes many)
// (it actually does not can work with multiple stderr)
// But what this actually will do?
// removing old stderr file
// and runs `x test --bless tests/ui/subdir/y.rs`
// which will automatically create a new stderr file
// for our new test

pub fn generate_stderr(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Remove old stderr file if it exists
    let stderr_path = format!("{}.stderr", path.trim_end_matches(".rs"));
    if Path::new(&stderr_path).exists() {
        fs::remove_file(&stderr_path)?;
    }

    // Run `cargo test --bless <path>`
    let status = Command::new("./x")
        .args(&["test", "--bless", path])
        .status()?;

    if !status.success() {
        return Err(format!("Failed to run cargo test --bless {}", path).into());
    }

    Ok(())
}
