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
fn completions_bash_include_supported_flags() {
    let mut cmd = Command::cargo_bin("ploc").unwrap();

    cmd.args(["completions", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--include-noise"))
        .stdout(predicate::str::contains("--no-color"))
        .stdout(predicate::str::contains("complete"));
}

#[test]
fn completions_zsh_include_supported_flags() {
    let mut cmd = Command::cargo_bin("ploc").unwrap();

    cmd.args(["completions", "zsh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--include-noise"))
        .stdout(predicate::str::contains("--no-color"))
        .stdout(predicate::str::contains("#compdef ploc"));
}

#[test]
fn completions_fish_include_supported_flags() {
    let mut cmd = Command::cargo_bin("ploc").unwrap();

    cmd.args(["completions", "fish"])
        .assert()
        .success()
        .stdout(predicate::str::contains("-l include-noise"))
        .stdout(predicate::str::contains("-l no-color"))
        .stdout(predicate::str::contains("complete -c ploc"));
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
fn language_counts_align_after_color_codes_are_removed() {
    let temp = assert_fs::TempDir::new().unwrap();
    temp.child("app.ts")
        .write_str("const a = 1;\nconst b = 2;\n")
        .unwrap();
    temp.child("component.svelte")
        .write_str("<script>let count = 0;</script>\n")
        .unwrap();
    temp.child("package.json")
        .write_str("{\"name\":\"demo\"}\n")
        .unwrap();

    let output = Command::cargo_bin("ploc")
        .unwrap()
        .current_dir(temp.path())
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let plain = strip_ansi(&stdout);
    let language_lines = plain
        .lines()
        .filter(|line| {
            line.contains("TypeScript") || line.contains("Svelte") || line.contains("JSON")
        })
        .collect::<Vec<_>>();
    let columns = language_lines
        .iter()
        .map(|line| line.find(|ch: char| ch.is_ascii_digit()).unwrap())
        .collect::<Vec<_>>();

    assert_eq!(columns, vec![columns[0]; columns.len()]);
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

fn strip_ansi(input: &str) -> String {
    let mut stripped = String::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\u{1b}' && chars.peek() == Some(&'[') {
            chars.next();
            for code_ch in chars.by_ref() {
                if code_ch.is_ascii_alphabetic() {
                    break;
                }
            }
        } else {
            stripped.push(ch);
        }
    }

    stripped
}
