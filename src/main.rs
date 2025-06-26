mod modules;
use modules::*;

use clap::Parser;

use crate::modules::{
    comment::add_comment, format::format_test_file, r#move::rmove, stderr::generate_stderr,
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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let current_path = format!("tests/ui/{}", args.source);
    let destination_path = format!("tests/ui/{}/{}", args.path, args.new_name);

    rmove(&current_path, &destination_path)?;

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
