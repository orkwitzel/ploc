use std::{fs, process::Command};

#[test]
fn install_script_is_valid_posix_shell() {
    let status = Command::new("sh")
        .args(["-n", "install.sh"])
        .status()
        .expect("sh should be available");

    assert!(status.success());
}

#[test]
fn install_script_uses_public_release_assets() {
    let script = fs::read_to_string("install.sh").unwrap();

    assert!(script.contains("orkwitzel/ploc"));
    assert!(script.contains("releases/latest/download"));
    assert!(script.contains("ploc-linux-x86_64.tar.gz"));
    assert!(script.contains("ploc-macos-x86_64.tar.gz"));
    assert!(script.contains("ploc-macos-aarch64.tar.gz"));
}

#[test]
fn man_page_documents_core_flags() {
    let man_page = fs::read_to_string("share/man/man1/ploc.1").unwrap();

    assert!(man_page.contains(".TH PLOC 1"));
    assert!(man_page.contains("\\-\\-include\\-noise"));
    assert!(man_page.contains("\\-\\-no\\-color"));
    assert!(man_page.contains("current working directory"));
}
