use crate::{
    Args, FileOperation,
    modules::{
        comment::add_comment, format::format_test_file, regression::extract_issue_number,
        stderr::generate_stderr,
    },
};

pub fn apply_post_move_operations(
    file_op: &FileOperation,
    args: &Args,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = &file_op.destination_path;

    // Apply regression comment if needed
    if args.regression {
        if let Ok(issue_number) = extract_issue_number(&file_op.source) {
            let regression_comment = format!(
                "Regression test for https://github.com/rust-lang/rust/issues/{}\n",
                issue_number
            );
            add_comment(path, &regression_comment)?;
        }
    }

    // Apply formatting if needed
    if args.fmt {
        format_test_file(path)?;
    }

    // Apply custom comment if provided
    if let Some(comment) = &args.comment {
        add_comment(path, comment)?;
    }

    // Generate stderr if needed
    if args.stderr {
        generate_stderr(path)?;
    }

    Ok(())
}
