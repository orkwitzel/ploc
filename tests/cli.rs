use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn help_lists_supported_flags() {
    let mut cmd = Command::cargo_bin("ploc").unwrap();

    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--include-noise"))
        .stdout(predicate::str::contains("--no-color"));
}

#[test]
fn version_prints_package_version() {
    let mut cmd = Command::cargo_bin("ploc").unwrap();

    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn scans_only_current_directory() {
    let temp = assert_fs::TempDir::new().unwrap();
    temp.child("parent.rs")
        .write_str("fn outside() {}\n")
        .unwrap();
    let child = temp.child("child");
    child.create_dir_all().unwrap();
    child
        .child("inside.rs")
        .write_str("fn inside() {}\n")
        .unwrap();

    let mut cmd = Command::cargo_bin("ploc").unwrap();
    cmd.current_dir(child.path())
        .arg("--no-color")
        .assert()
        .success()
        .stdout(predicate::str::contains("LOC       1"))
        .stdout(predicate::str::contains("Files     1"));
}

#[test]
fn excludes_noise_directories_by_default() {
    let temp = assert_fs::TempDir::new().unwrap();
    temp.child("src.rs").write_str("fn main() {}\n").unwrap();
    temp.child("target/generated.rs")
        .write_str("fn generated() {}\n")
        .unwrap();

    let mut cmd = Command::cargo_bin("ploc").unwrap();
    cmd.current_dir(temp.path())
        .arg("--no-color")
        .assert()
        .success()
        .stdout(predicate::str::contains("LOC       1"))
        .stdout(predicate::str::contains("Files     1"));
}

#[test]
fn include_noise_counts_noise_directories() {
    let temp = assert_fs::TempDir::new().unwrap();
    temp.child("src.rs").write_str("fn main() {}\n").unwrap();
    temp.child("target/generated.rs")
        .write_str("fn generated() {}\n")
        .unwrap();

    let mut cmd = Command::cargo_bin("ploc").unwrap();
    cmd.current_dir(temp.path())
        .args(["--no-color", "--include-noise"])
        .assert()
        .success()
        .stdout(predicate::str::contains("LOC       2"))
        .stdout(predicate::str::contains("Files     2"));
}

#[test]
fn renders_language_breakdown_sorted_by_loc() {
    let temp = assert_fs::TempDir::new().unwrap();
    temp.child("main.rs")
        .write_str("fn main() {}\nfn helper() {}\n")
        .unwrap();
    temp.child("app.py").write_str("print('hi')\n").unwrap();

    let mut cmd = Command::cargo_bin("ploc").unwrap();
    cmd.current_dir(temp.path())
        .arg("--no-color")
        .assert()
        .success()
        .stdout(predicate::str::contains("Languages 2"))
        .stdout(predicate::str::contains("Rust"))
        .stdout(predicate::str::contains("Python"))
        .stdout(predicate::str::contains("│ █"))
        .stdout(predicate::str::contains("│ Rust"))
        .stdout(predicate::str::contains("│ Python"))
        .stdout(predicate::str::contains("Rust").and(predicate::str::contains("66.7%").not()));
}

#[test]
fn colored_output_matches_bar_and_legend_colors() {
    let temp = assert_fs::TempDir::new().unwrap();
    temp.child("main.rs")
        .write_str("fn main() {}\nfn helper() {}\n")
        .unwrap();
    temp.child("app.py").write_str("print('hi')\n").unwrap();

    let output = Command::cargo_bin("ploc")
        .unwrap()
        .current_dir(temp.path())
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("\u{1b}[38;5;208m█"));
    assert!(stdout.contains("\u{1b}[38;5;208mRust"));
    assert!(stdout.contains("\u{1b}[38;5;39m█"));
    assert!(stdout.contains("\u{1b}[38;5;39mPython"));
}

#[test]
fn no_color_output_has_no_ansi_sequences() {
    let temp = assert_fs::TempDir::new().unwrap();
    temp.child("main.rs").write_str("fn main() {}\n").unwrap();

    let output = Command::cargo_bin("ploc")
        .unwrap()
        .current_dir(temp.path())
        .arg("--no-color")
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(!stdout.contains("\u{1b}["));
}
