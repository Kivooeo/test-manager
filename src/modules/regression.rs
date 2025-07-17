/// Extracts issue number from filename like "issue-12345.rs" -> "12345"
pub fn extract_issue_number(filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    let basename = std::path::Path::new(filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or("Invalid filename")?;

    if let Some(stripped) = basename.strip_prefix("issue-") {
        // Validate that the remainder is a number
        if stripped.chars().all(|c| c.is_ascii_digit()) {
            Ok(stripped.to_string())
        } else {
            Err(format!("Invalid issue number format in filename: {}", filename).into())
        }
    } else {
        Err(format!("Filename does not match issue-XXXXX pattern: {}", filename).into())
    }
}
