mod modules;
use modules::*;

use clap::Parser;
use std::io::{self, Write};
use std::process::Command;

use crate::modules::regression::extract_issue_number;
use crate::modules::{
    comment::add_comment, commit::git_commit_move, format::format_test_file, r#move::rmove,
    stderr::generate_stderr,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Source test file (e.g., custom_attribute.rs)
    source: String,

    /// New test file name (e.g., custom_attributes_error.rs)
    #[arg(short = 'n')]
    new_name: String,

    /// Subdirectory under tests/ui (e.g., attributes)
    #[arg(short = 'p', long)]
    path: String,

    /// Remove associated .stderr file and run test
    #[arg(short = 's', long)]
    stderr: bool,

    /// Doc comment to add at the top of the test file
    #[arg(short = 'c', long)]
    comment: Option<String>,

    /// Format the test file with rustfmt
    #[arg(short = 'f', long = "fmt")]
    fmt: bool,

    /// Regression test: extract issue number from filename and add GitHub link
    #[arg(short = 'R', long)]
    regression: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let current_path = format!("tests/ui/issues/{}", args.source);
    let destination_path = format!("tests/ui/{}/{}", args.path, args.new_name);
    dbg!(&current_path);
    dbg!(&destination_path);
    rmove(&current_path, &destination_path)?;

    if args.regression {
        let issue_number = extract_issue_number(&args.source)?;
        let regression_comment = format!(
            "Regression test for https://github.com/rust-lang/rust/issues/{}\n",
            issue_number
        );
        add_comment(&destination_path, &regression_comment)?;
    }

    if args.fmt {
        format_test_file(&destination_path)?;
    }

    if args.comment.is_some() {
        add_comment(&destination_path, &args.comment.unwrap())?;
    }

    if args.stderr {
        generate_stderr(&destination_path)?;
    }

    Ok(())
}
