use std::process::Command;

/// Makes a git commit with a message indicating a file move from old_path to new_path.
/// Returns Result<(), String> indicating success or error message.
pub fn git_commit_move(old_path: &str, new_path: &str) -> Result<(), String> {
    let message = format!("moved {} -> {}", old_path, new_path);
    let status = Command::new("git")
        .args(["commit", "-m", &message])
        .status()
        .map_err(|e| format!("Failed to execute git: {}", e))?;

    // Add both old and new paths to the git index
    let add_status = Command::new("git")
        .args(["add", old_path, new_path])
        .status()
        .map_err(|e| format!("Failed to execute git add: {}", e))?;

    if !add_status.success() {
        return Err(format!("Git add failed with status: {}", add_status));
    }

    if status.success() {
        Ok(())
    } else {
        Err(format!("Git commit failed with status: {}", status))
    }
}
