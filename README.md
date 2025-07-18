# test-manager
Small written in Rust test manager to improve quality of life working in tests/ui

## Usage

### Single File Mode (Default)
```bash
test-manager <source> -n <new_name> -p <path> [OPTIONS]
```

**Required:**
- `<source>` - Source file (e.g., `issue-12345.rs`)
- `-n <new_name>` - New filename (e.g., `custom_attr.rs`)
- `-p <path>` - Subdirectory under `tests/ui` (e.g., `attributes`)

### Multi-File Mode
```bash
test-manager -m <source1> <path1> <new_name1> <source2> <path2> <new_name2> ... [OPTIONS]
```

**Required:**
- `-m, --multi` - Enable multi-file mode
- Triplets of: `<source> <path> <new_name>` for each file to move

**Options (available in both modes):**
- `-s, --stderr` - Remove old `.stderr` file and regenerate with `./x test --bless`
- `-c, --comment <TEXT>` - Add `//! <TEXT>` doc comment at top of file
- `-f, --fmt` - Format file with `rustfmt`
- `-R, --regression` - Extract issue number from filename and add GitHub link comment
- `-g, --git` - Commit moves with git

## Examples

### Single File Mode
```bash
# Basic move
test-manager issue-12345.rs -n custom_attr.rs -p attributes

# Regression test with stderr
test-manager issue-98765.rs -n proc_macro_span.rs -p proc-macro -R -s

# Full workflow with git commit
test-manager issue-54321.rs -n lifetime_bounds.rs -p lifetimes -s -f -c "Test description" -g
```

### Multi-File Mode
```bash
# Move multiple files at once
test-manager -m \
  issue-12345.rs attributes custom_attr.rs \
  issue-67890.rs proc-macro proc_macro_span.rs \
  issue-11111.rs lifetimes lifetime_bounds.rs

# Multi-file with options
test-manager -m \
  issue-12345.rs attributes custom_attr.rs \
  issue-67890.rs proc-macro proc_macro_span.rs \
  -R -s -g
```

## Workflow

The tool operates in phases:

1. **Prepare**: Parse arguments and prepare file operations
2. **Move**: Move all files to their new locations
3. **Git Commit** (if `-g`): Commit the moves before applying other changes
4. **Post-processing**: Apply other operations (comments, formatting, stderr generation)

This ensures git history is clean when using the `-g` flag, as moves are committed separately from content changes.

## File Structure

All source files are expected to be in `tests/ui/issues/` and will be moved to `tests/ui/<path>/`.

## Adding New Features

Each operation is a separate module in `src/modules/`. To add a new feature:

1. Create `src/modules/your_feature.rs` with your function
2. Add it to `src/modules/mod.rs`
3. Add the flag to `Args` struct in `main.rs`
4. Call your function in `main.rs`

Example adding a `--backup` flag:
```rust
// src/modules/backup.rs
pub fn create_backup(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::copy(path, format!("{}.bak", path))?;
    Ok(())
}

// In main.rs Args struct:
#[arg(short = 'b', long)]
backup: bool,

// In main() function:
if args.backup {
    create_backup(&destination_path)?;
}
```
