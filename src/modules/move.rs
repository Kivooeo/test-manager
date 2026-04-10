use std::fs;

// This simple module will move file from
// `tests/ui/x.rs` to `tests/ui/subdir/y.rs`
//
// this is pretty simple first move where we start
// renaming and moving it into right place

pub fn rmove(
    from: &std::path::Path,
    to: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create parent directories for the destination if they don't exist
    if let Some(parent) = to.parent() {
        fs::create_dir_all(parent)?;
    }

    // Move (rename) the file
    fs::rename(from, to)?;

    Ok(())
}
