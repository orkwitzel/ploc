use assert_cmd::Command;
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
