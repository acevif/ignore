#[test]
fn prints_version() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ignore"))
        .arg("--version")
        .output()
        .expect("run ignore --version");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        format!("ignore {}\n", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn prints_version_with_subcommand() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ignore"))
        .arg("version")
        .output()
        .expect("run ignore version");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        format!("ignore {}\n", env!("CARGO_PKG_VERSION"))
    );
}
