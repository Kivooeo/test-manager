use std::fs;
use std::io;

/// Adds a doc comment to the top of a file if one doesn't already exist.
///
/// This helps future developers understand test intent, as tests without
/// clear documentation can become dead weight over time.
pub fn add_comment(path: &str, comment_line: &str) -> io::Result<()> {
    let content = fs::read_to_string(path)?;

    // Check if the first line is already a doc comment
    let first_line_is_doc_comment = content
        .lines()
        .next()
        .map_or(false, |line| line.trim_start().starts_with("//!"));

    if !first_line_is_doc_comment {
        let new_content = format!("//! {}\n{}", comment_line, content);
        fs::write(path, new_content)?;
    }

    Ok(())
}
