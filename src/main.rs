use clap::Parser;
use std::fs;
use std::path::Path;
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

/// Validates coverage is within valid range (0-100).
fn validate_coverage(coverage: f64) -> Result<(), String> {
    if coverage < 0.0 || coverage > 100.0 {
        Err(format!("coverage must be between 0 and 100, got: {}", coverage))
    } else {
        Ok(())
    }
}

/// Ensures the output directory exists, creating it if necessary.
fn ensure_output_dir(path: &Path) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| {
                format!("cannot create directory '{}': {}", parent.display(), e)
            })?;
        }
    }
    Ok(())
}

/// Writes the badge to the specified path.
fn write_badge(path: &Path, svg: &str) -> Result<(), String> {
    fs::write(path, svg).map_err(|e| format!("cannot write to '{}': {}", path.display(), e))
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = validate_coverage(cli.coverage) {
        eprintln!("error: {}", e);
        process::exit(1);
    }

    let svg = coverage_badge::generate_badge(cli.coverage);

    if let Err(e) = ensure_output_dir(&cli.output) {
        eprintln!("error: {}", e);
        process::exit(1);
    }

    match write_badge(&cli.output, &svg) {
        Ok(_) => println!("Badge written to {}", cli.output.display()),
        Err(e) => {
            eprintln!("error: {}", e);
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_validate_coverage_valid() {
        assert!(validate_coverage(0.0).is_ok());
        assert!(validate_coverage(50.0).is_ok());
        assert!(validate_coverage(100.0).is_ok());
        assert!(validate_coverage(75.5).is_ok());
    }

    #[test]
    fn test_validate_coverage_invalid() {
        assert!(validate_coverage(-1.0).is_err());
        assert!(validate_coverage(101.0).is_err());
        assert!(validate_coverage(-0.1).is_err());
        assert!(validate_coverage(100.1).is_err());
    }

    #[test]
    fn test_validate_coverage_error_message() {
        let err = validate_coverage(150.0).unwrap_err();
        assert!(err.contains("150"));
        assert!(err.contains("between 0 and 100"));
    }

    #[test]
    fn test_ensure_output_dir_exists() {
        // Current directory should exist, no-op
        let path = PathBuf::from("test.svg");
        assert!(ensure_output_dir(&path).is_ok());
    }

    #[test]
    fn test_ensure_output_dir_creates_missing() {
        let path = PathBuf::from("target/test-ensure-dir/nested/badge.svg");

        // Clean up first if exists
        let _ = std::fs::remove_dir_all("target/test-ensure-dir");

        // Should create the directory
        assert!(ensure_output_dir(&path).is_ok());
        assert!(path.parent().unwrap().exists());

        // Cleanup
        std::fs::remove_dir_all("target/test-ensure-dir").ok();
    }

    #[test]
    fn test_write_badge_success() {
        let path = PathBuf::from("target/test-write-badge.svg");
        let svg = "<svg>test</svg>";
        assert!(write_badge(&path, svg).is_ok());

        // Verify content
        let content = std::fs::read_to_string(&path).unwrap();
        assert_eq!(content, svg);

        // Cleanup
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn test_write_badge_missing_dir() {
        let path = PathBuf::from("nonexistent/dir/badge.svg");
        let svg = "<svg>test</svg>";
        let result = write_badge(&path, svg);
        assert!(result.is_err());
    }
}
