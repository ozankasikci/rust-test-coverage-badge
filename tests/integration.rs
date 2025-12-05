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

    assert_ne!(code, 0, "Should exit with non-zero code");
    // Clap exits with code 2 when it can't parse arguments
    // Note: -5 is interpreted as a flag, not a negative number
    assert!(stderr.contains("unexpected argument") || stderr.contains("coverage must be between 0 and 100"), "Should show error");
}

#[test]
fn test_rejects_coverage_over_100() {
    let (_, stderr, code) = run_cli(&["--coverage", "150", "--output", "test.svg"]);

    assert_eq!(code, 1, "Should exit with code 1");
    assert!(stderr.contains("coverage must be between 0 and 100"), "Should show error");
}

#[test]
fn test_creates_missing_directory() {
    let output_path = "target/test-auto-create/nested/badge.svg";

    // Clean up first if exists
    let _ = fs::remove_dir_all("target/test-auto-create");

    let (stdout, _, code) = run_cli(&["--coverage", "50", "--output", output_path]);

    assert_eq!(code, 0, "Should exit with code 0");
    assert!(stdout.contains("Badge written to"), "Should print success message");
    assert!(fs::metadata(output_path).is_ok(), "File should exist");

    // Cleanup
    fs::remove_dir_all("target/test-auto-create").ok();
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
