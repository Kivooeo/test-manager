use std::process::Command;

pub fn git_commit_moves(operations: &[crate::FileOperation]) -> Result<(), String> {
    // Add all files to git
    for op in operations {
        let add_status = Command::new("git")
            .args(["add", &op.current_path, &op.destination_path])
            .status()
            .map_err(|e| format!("Failed to execute git add: {}", e))?;

        if !add_status.success() {
            return Err(format!("Git add failed for {}", op.destination_path));
        }
    }

    // Single commit for all moves
    let message = if operations.len() == 1 {
        format!(
            "moved {} -> {}",
            operations[0].current_path, operations[0].destination_path
        )
    } else {
        format!("moved {} tests to organized locations", operations.len())
    };

    let status = Command::new("git")
        .args(["commit", "-m", &message])
        .status()
        .map_err(|e| format!("Failed to execute git: {}", e))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("Git commit failed with status: {}", status))
    }
}
