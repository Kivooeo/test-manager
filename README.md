# test-manager
Small written in Rust test manager to improve quality of life working in tests/ui

## Usage

```bash
test-manager <source> -n <new_name> -p <path> [OPTIONS]
```

**Required:**
- `<source>` - Source file (e.g., `issue-12345.rs`)
- `-n <new_name>` - New filename (e.g., `custom_attr.rs`)
- `-p <path>` - Subdirectory under `tests/ui` (e.g., `attributes`)

**Optional:**
- `-s, --stderr` - Remove old `.stderr` file and regenerate with `./x test --bless`
- `-c, --comment <TEXT>` - Add `//! <TEXT>` doc comment at top of file
- `-f, --fmt` - Format file with `rustfmt`
- `-R, --regression` - Extract issue number from filename and add GitHub link comment

## Examples

```bash
# Basic move
test-manager issue-12345.rs -n custom_attr.rs -p attributes

# Regression test with stderr
test-manager issue-98765.rs -n proc_macro_span.rs -p proc-macro -R -s

# Full workflow
test-manager issue-54321.rs -n lifetime_bounds.rs -p lifetimes -s -f -c "Test description"
```

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
