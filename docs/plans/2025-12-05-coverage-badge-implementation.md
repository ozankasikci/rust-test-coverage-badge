# coverage-badge Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a Rust CLI tool that generates shields.io-style SVG coverage badges.

**Architecture:** Simple CLI with clap for argument parsing, hand-rolled SVG template generation, color selection based on coverage thresholds. No external SVG libraries needed.

**Tech Stack:** Rust, clap (derive), std::fs for file writing

---

## Task 1: Initialize Cargo Project

**Files:**
- Create: `Cargo.toml`
- Create: `src/main.rs`

**Step 1: Initialize the project**

Run:
```bash
cargo init --name coverage-badge
```

**Step 2: Add clap dependency**

Edit `Cargo.toml` to add:
```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
```

**Step 3: Verify it compiles**

Run: `cargo build`
Expected: Compiles successfully

**Step 4: Commit**

```bash
git add Cargo.toml src/main.rs
git commit -m "Initialize coverage-badge project with clap dependency"
```

---

## Task 2: Create Color Module with Tests

**Files:**
- Create: `src/color.rs`
- Modify: `src/main.rs` (add module declaration)

**Step 1: Write the failing tests**

Create `src/color.rs`:
```rust
/// Returns the hex color for a given coverage percentage.
/// - < 50%: red (#e05d44)
/// - 50-79%: yellow (#dfb317)
/// - >= 80%: green (#4c1)
pub fn coverage_color(percentage: f64) -> &'static str {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_low_coverage_is_red() {
        assert_eq!(coverage_color(0.0), "#e05d44");
        assert_eq!(coverage_color(25.0), "#e05d44");
        assert_eq!(coverage_color(49.9), "#e05d44");
    }

    #[test]
    fn test_medium_coverage_is_yellow() {
        assert_eq!(coverage_color(50.0), "#dfb317");
        assert_eq!(coverage_color(65.0), "#dfb317");
        assert_eq!(coverage_color(79.9), "#dfb317");
    }

    #[test]
    fn test_high_coverage_is_green() {
        assert_eq!(coverage_color(80.0), "#4c1");
        assert_eq!(coverage_color(90.0), "#4c1");
        assert_eq!(coverage_color(100.0), "#4c1");
    }
}
```

Add to `src/main.rs`:
```rust
mod color;

fn main() {
    println!("Hello, world!");
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test`
Expected: FAIL with "not yet implemented"

**Step 3: Implement the function**

Replace the `todo!()` in `src/color.rs`:
```rust
pub fn coverage_color(percentage: f64) -> &'static str {
    if percentage < 50.0 {
        "#e05d44"
    } else if percentage < 80.0 {
        "#dfb317"
    } else {
        "#4c1"
    }
}
```

**Step 4: Run tests to verify they pass**

Run: `cargo test`
Expected: All 3 tests PASS

**Step 5: Commit**

```bash
git add src/color.rs src/main.rs
git commit -m "Add color module with coverage threshold logic"
```

---

## Task 3: Create Badge Module with SVG Generation

**Files:**
- Create: `src/badge.rs`
- Modify: `src/main.rs` (add module declaration)

**Step 1: Write the failing test**

Create `src/badge.rs`:
```rust
use crate::color::coverage_color;

/// Generates an SVG badge for the given coverage percentage.
pub fn generate_badge(percentage: f64) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_badge_contains_coverage_label() {
        let svg = generate_badge(85.0);
        assert!(svg.contains("coverage"), "Badge should contain 'coverage' label");
    }

    #[test]
    fn test_badge_contains_percentage() {
        let svg = generate_badge(85.0);
        assert!(svg.contains("85%"), "Badge should contain percentage value");
    }

    #[test]
    fn test_badge_contains_correct_color() {
        let svg = generate_badge(85.0);
        assert!(svg.contains("#4c1"), "85% coverage should use green color");

        let svg_low = generate_badge(30.0);
        assert!(svg_low.contains("#e05d44"), "30% coverage should use red color");
    }

    #[test]
    fn test_badge_is_valid_svg() {
        let svg = generate_badge(50.0);
        assert!(svg.starts_with("<svg"), "Should start with <svg");
        assert!(svg.ends_with("</svg>"), "Should end with </svg>");
    }

    #[test]
    fn test_percentage_formatting_whole_number() {
        let svg = generate_badge(85.0);
        assert!(svg.contains("85%"));
        assert!(!svg.contains("85.0%"));
    }

    #[test]
    fn test_percentage_formatting_decimal() {
        let svg = generate_badge(85.5);
        assert!(svg.contains("85.5%"));
    }
}
```

Add to `src/main.rs`:
```rust
mod badge;
mod color;

fn main() {
    println!("Hello, world!");
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test`
Expected: FAIL with "not yet implemented"

**Step 3: Implement the badge generation**

Replace the contents of `src/badge.rs`:
```rust
use crate::color::coverage_color;

/// Formats the percentage for display.
/// Shows decimal only if it's not a whole number.
fn format_percentage(percentage: f64) -> String {
    if percentage.fract() == 0.0 {
        format!("{}%", percentage as i64)
    } else {
        format!("{:.1}%", percentage)
    }
}

/// Generates an SVG badge for the given coverage percentage.
pub fn generate_badge(percentage: f64) -> String {
    let color = coverage_color(percentage);
    let percentage_text = format_percentage(percentage);

    // Approximate width calculation (shields.io style)
    // Label "coverage" is ~52px, percentage varies
    let label_width = 60;
    let value_width = 10 + (percentage_text.len() as i32 * 7);
    let total_width = label_width + value_width;

    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{total_width}" height="20">
  <linearGradient id="smooth" x2="0" y2="100%">
    <stop offset="0" stop-color="#bbb" stop-opacity=".1"/>
    <stop offset="1" stop-opacity=".1"/>
  </linearGradient>
  <clipPath id="round">
    <rect width="{total_width}" height="20" rx="3" fill="#fff"/>
  </clipPath>
  <g clip-path="url(#round)">
    <rect width="{label_width}" height="20" fill="#555"/>
    <rect x="{label_width}" width="{value_width}" height="20" fill="{color}"/>
    <rect width="{total_width}" height="20" fill="url(#smooth)"/>
  </g>
  <g fill="#fff" text-anchor="middle" font-family="DejaVu Sans,Verdana,Geneva,sans-serif" font-size="11">
    <text x="{label_x}" y="15" fill="#010101" fill-opacity=".3">coverage</text>
    <text x="{label_x}" y="14">coverage</text>
    <text x="{value_x}" y="15" fill="#010101" fill-opacity=".3">{percentage_text}</text>
    <text x="{value_x}" y="14">{percentage_text}</text>
  </g>
</svg>"#,
        total_width = total_width,
        label_width = label_width,
        value_width = value_width,
        color = color,
        label_x = label_width / 2,
        value_x = label_width + (value_width / 2),
        percentage_text = percentage_text
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_badge_contains_coverage_label() {
        let svg = generate_badge(85.0);
        assert!(svg.contains("coverage"), "Badge should contain 'coverage' label");
    }

    #[test]
    fn test_badge_contains_percentage() {
        let svg = generate_badge(85.0);
        assert!(svg.contains("85%"), "Badge should contain percentage value");
    }

    #[test]
    fn test_badge_contains_correct_color() {
        let svg = generate_badge(85.0);
        assert!(svg.contains("#4c1"), "85% coverage should use green color");

        let svg_low = generate_badge(30.0);
        assert!(svg_low.contains("#e05d44"), "30% coverage should use red color");
    }

    #[test]
    fn test_badge_is_valid_svg() {
        let svg = generate_badge(50.0);
        assert!(svg.starts_with("<svg"), "Should start with <svg");
        assert!(svg.ends_with("</svg>"), "Should end with </svg>");
    }

    #[test]
    fn test_percentage_formatting_whole_number() {
        let svg = generate_badge(85.0);
        assert!(svg.contains("85%"));
        assert!(!svg.contains("85.0%"));
    }

    #[test]
    fn test_percentage_formatting_decimal() {
        let svg = generate_badge(85.5);
        assert!(svg.contains("85.5%"));
    }
}
```

**Step 4: Run tests to verify they pass**

Run: `cargo test`
Expected: All tests PASS

**Step 5: Commit**

```bash
git add src/badge.rs src/main.rs
git commit -m "Add badge module with SVG generation"
```

---

## Task 4: Create lib.rs for Public API

**Files:**
- Create: `src/lib.rs`
- Modify: `src/main.rs`

**Step 1: Create lib.rs**

Create `src/lib.rs`:
```rust
mod badge;
mod color;

pub use badge::generate_badge;
pub use color::coverage_color;
```

**Step 2: Update main.rs to use the library**

Replace `src/main.rs`:
```rust
fn main() {
    println!("Hello, world!");
}
```

**Step 3: Verify tests still pass**

Run: `cargo test`
Expected: All tests PASS

**Step 4: Commit**

```bash
git add src/lib.rs src/main.rs
git commit -m "Add lib.rs with public API exports"
```

---

## Task 5: Implement CLI Argument Parsing

**Files:**
- Modify: `src/main.rs`

**Step 1: Implement CLI with clap**

Replace `src/main.rs`:
```rust
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::process;

/// Generate a shields.io-style coverage badge as an SVG file.
#[derive(Parser)]
#[command(name = "coverage-badge")]
#[command(about = "Generate a coverage badge SVG", long_about = None)]
struct Cli {
    /// Coverage percentage (0-100)
    #[arg(short, long)]
    coverage: f64,

    /// Output path for the SVG file
    #[arg(short, long)]
    output: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    // Validate coverage range
    if cli.coverage < 0.0 || cli.coverage > 100.0 {
        eprintln!("error: coverage must be between 0 and 100, got: {}", cli.coverage);
        process::exit(1);
    }

    // Generate the badge
    let svg = coverage_badge::generate_badge(cli.coverage);

    // Write to file
    if let Some(parent) = cli.output.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            eprintln!("error: cannot write to '{}': directory does not exist", cli.output.display());
            process::exit(1);
        }
    }

    match fs::write(&cli.output, &svg) {
        Ok(_) => println!("Badge written to {}", cli.output.display()),
        Err(e) => {
            eprintln!("error: cannot write to '{}': {}", cli.output.display(), e);
            process::exit(1);
        }
    }
}
```

**Step 2: Test the CLI manually**

Run: `cargo run -- --coverage 85 --output test-badge.svg`
Expected: "Badge written to test-badge.svg"

Run: `cargo run -- --coverage 150 --output test.svg`
Expected: "error: coverage must be between 0 and 100, got: 150" and exit code 1

**Step 3: Clean up test file**

Run: `rm -f test-badge.svg`

**Step 4: Commit**

```bash
git add src/main.rs
git commit -m "Implement CLI argument parsing and file output"
```

---

## Task 6: Add Integration Tests

**Files:**
- Create: `tests/integration.rs`

**Step 1: Write integration tests**

Create `tests/integration.rs`:
```rust
use std::fs;
use std::process::Command;

fn run_cli(args: &[&str]) -> (String, String, i32) {
    let output = Command::new("cargo")
        .args(["run", "--"])
        .args(args)
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let code = output.status.code().unwrap_or(-1);

    (stdout, stderr, code)
}

#[test]
fn test_generates_badge_file() {
    let output_path = "target/test-badge-integration.svg";

    let (stdout, _, code) = run_cli(&["--coverage", "75", "--output", output_path]);

    assert_eq!(code, 0, "Should exit with code 0");
    assert!(stdout.contains("Badge written to"), "Should print success message");
    assert!(fs::metadata(output_path).is_ok(), "File should exist");

    let contents = fs::read_to_string(output_path).unwrap();
    assert!(contents.contains("<svg"), "Should be valid SVG");
    assert!(contents.contains("75%"), "Should contain percentage");

    fs::remove_file(output_path).ok();
}

#[test]
fn test_rejects_negative_coverage() {
    let (_, stderr, code) = run_cli(&["--coverage", "-5", "--output", "test.svg"]);

    assert_eq!(code, 1, "Should exit with code 1");
    assert!(stderr.contains("coverage must be between 0 and 100"), "Should show error");
}

#[test]
fn test_rejects_coverage_over_100() {
    let (_, stderr, code) = run_cli(&["--coverage", "150", "--output", "test.svg"]);

    assert_eq!(code, 1, "Should exit with code 1");
    assert!(stderr.contains("coverage must be between 0 and 100"), "Should show error");
}

#[test]
fn test_rejects_missing_directory() {
    let (_, stderr, code) = run_cli(&["--coverage", "50", "--output", "nonexistent/dir/badge.svg"]);

    assert_eq!(code, 1, "Should exit with code 1");
    assert!(stderr.contains("directory does not exist"), "Should show directory error");
}

#[test]
fn test_missing_coverage_arg() {
    let (_, stderr, code) = run_cli(&["--output", "test.svg"]);

    assert_ne!(code, 0, "Should exit with non-zero code");
    assert!(stderr.contains("--coverage"), "Should mention missing arg");
}

#[test]
fn test_missing_output_arg() {
    let (_, stderr, code) = run_cli(&["--coverage", "50"]);

    assert_ne!(code, 0, "Should exit with non-zero code");
    assert!(stderr.contains("--output"), "Should mention missing arg");
}
```

**Step 2: Run integration tests**

Run: `cargo test --test integration`
Expected: All 6 tests PASS

**Step 3: Commit**

```bash
git add tests/integration.rs
git commit -m "Add integration tests for CLI"
```

---

## Task 7: Add README

**Files:**
- Create: `README.md`

**Step 1: Create README**

Create `README.md`:
```markdown
# coverage-badge

A Rust CLI tool that generates shields.io-style SVG badges for test coverage.

## Installation

```bash
cargo install --path .
```

## Usage

```bash
coverage-badge --coverage <PERCENTAGE> --output <PATH>
```

### Arguments

- `--coverage, -c` - Coverage percentage (0-100, decimals allowed)
- `--output, -o` - Output path for the SVG file

### Examples

```bash
# Generate a badge for 85% coverage
coverage-badge -c 85 -o assets/coverage.svg

# With decimal precision
coverage-badge --coverage 72.5 --output .github/badges/coverage.svg
```

### In your README

```markdown
![Coverage](assets/coverage.svg)
```

## Color Thresholds

| Coverage | Color |
|----------|-------|
| < 50% | Red |
| 50-79% | Yellow |
| ≥ 80% | Green |

## License

MIT
```

**Step 2: Commit**

```bash
git add README.md
git commit -m "Add README with usage instructions"
```

---

## Task 8: Final Verification

**Step 1: Run all tests**

Run: `cargo test`
Expected: All tests PASS

**Step 2: Build release binary**

Run: `cargo build --release`
Expected: Compiles successfully

**Step 3: Test release binary**

Run: `./target/release/coverage-badge -c 95 -o example-badge.svg`
Expected: "Badge written to example-badge.svg"

**Step 4: Verify the generated badge**

Run: `cat example-badge.svg`
Expected: Valid SVG with "coverage" label and "95%"

**Step 5: Clean up and commit example**

```bash
mkdir -p assets
mv example-badge.svg assets/coverage.svg
git add assets/coverage.svg
git commit -m "Add example coverage badge"
```
