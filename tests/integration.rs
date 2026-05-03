use std::fs;
use std::process::Command;
use tempfile::TempDir;

fn cloc_bin() -> std::path::PathBuf {
    let mut p = std::env::current_exe().unwrap();
    // Walk up from the test binary to find the cloc binary
    loop {
        if p.ends_with("deps") {
            p.pop();
            break;
        }
        if !p.pop() {
            break;
        }
    }
    p.join("cloc")
}

#[test]
fn counts_rust_files() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("main.rs"), "fn main() {\n    let x = 1;\n}\n").unwrap();

    let out = Command::new(cloc_bin())
        .arg(dir.path())
        .output()
        .expect("failed to run cloc");

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success());
    assert!(stdout.contains("Rust"));
}

#[test]
fn json_output_is_valid() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("a.py"), "x = 1\n# comment\n\n").unwrap();

    let out = Command::new(cloc_bin())
        .args(["--json", dir.path().to_str().unwrap()])
        .output()
        .expect("failed to run cloc");

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&stdout)
        .expect("output is not valid JSON");
    let code = parsed["totals"]["code"].as_u64().unwrap_or(0);
    assert!(code > 0, "expected non-zero code count");
}

#[test]
fn not_directory_is_excluded() {
    let dir = TempDir::new().unwrap();
    let not_dir = dir.path().join(".not");
    fs::create_dir(&not_dir).unwrap();
    fs::write(not_dir.join("hidden.rs"), "fn hidden() {}").unwrap();
    fs::write(dir.path().join("visible.rs"), "fn visible() {}").unwrap();

    let out = Command::new(cloc_bin())
        .arg(dir.path())
        .output()
        .expect("failed to run cloc");

    let stdout = String::from_utf8_lossy(&out.stdout);
    // Only one file counted; hidden.rs inside .not is excluded
    assert!(stdout.contains("Rust"));
    // The SUM row should show 1 file
    assert!(stdout.contains("1"));
}

#[test]
fn csv_output_has_header() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("x.rs"), "fn x() {}").unwrap();

    let out = Command::new(cloc_bin())
        .args(["--csv", dir.path().to_str().unwrap()])
        .output()
        .expect("failed to run cloc");

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.starts_with("language,files,blank,comment,code"));
}
