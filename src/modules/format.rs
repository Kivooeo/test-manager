use std::process::Command;

// Third step
// Sometimes we shall format a test file,
// Because sometimes it was created before rust 1.0
// And have incredible awful formatting
// So sometimes we actaully want run some `rustfmt` on it
//
// But _sometimes_ awful formatting is intend,
// So please check this

pub fn format_test_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Run rustfmt on the file
    let status = Command::new("rustfmt").arg(path).status()?;

    if !status.success() {
        return Err("rustfmt failed".into());
    }

    Ok(())
}
