mod modules;

use clap::Parser;

use crate::modules::multiple::prepare_multi_operations;
use crate::modules::post_move::apply_post_move_operations;
use crate::modules::single::prepare_single_operation;
use crate::modules::{commit::git_commit_moves, r#move::rmove};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Multi-file mode: alternating source, path, newname triplets
    #[arg(short = 'm', long)]
    multi: bool,

    /// Arguments - interpretation depends on multi flag
    /// Default mode: source -n newname -p path
    /// Multi mode: source1 path1 newname1 source2 path2 newname2 ...
    args: Vec<String>,

    /// New test file name (default mode only)
    #[arg(short = 'n', conflicts_with = "multi")]
    new_name: Option<String>,

    /// Subdirectory under tests/ui (default mode only)
    #[arg(short = 'p', long, conflicts_with = "multi")]
    path: Option<String>,

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

    /// Commit moves with git
    #[arg(short = 'g', long)]
    git: bool,
}

#[derive(Debug)]
struct FileOperation {
    source: String,           // Original source filename (e.g., "issue-12345.rs")
    destination: String,      // New destination filename (e.g., "custom_attr.rs")
    current_path: String,     // Full current path
    destination_path: String, // Full destination path
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Phase 1: Prepare all file operations
    let operations = if args.multi {
        prepare_multi_operations(&args)?
    } else {
        prepare_single_operation(&args)?
    };

    // Phase 2: Move all files
    for op in &operations {
        rmove(&op.current_path, &op.destination_path)?;
    }

    // Phase 3: Git commit if requested
    // This is -g flag and it's needed
    // To commit moves before changing files
    if args.git {
        git_commit_moves(&operations)?;
    }

    // Phase 4: Apply other operations to each moved file
    for op in &operations {
        apply_post_move_operations(op, &args)?; // Pass the whole FileOperation
    }

    Ok(())
}
