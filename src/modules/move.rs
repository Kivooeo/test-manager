use std::fs;
use std::path::Path;

// This simple module will move file from
// `tests/ui/x.rs` to `tests/ui/subdir/y.rs`
//
// this is pretty simple first move where we start
// renaming and moving it into right place

pub fn rmove(from: &str, to: &str) -> Result<(), Box<dyn std::error::Error>> {
    let from_path = Path::new(from);
    let to_path = Path::new(to);

    // Create parent directories for the destination if they don't exist
    if let Some(parent) = to_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Move (rename) the file
    fs::rename(from_path, to_path)?;

    Ok(())
}
