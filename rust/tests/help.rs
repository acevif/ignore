const EXPECTED_HELP: &str = "Usage: ignore [<command>]\n\nCommands:\n  help     Show this help message\n  version  Show version information\n\nOptions:\n  -h, --help  Show this help message\n  --version   Show version information\n";

#[test]
fn prints_help() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ignore"))
        .arg("help")
        .output()
        .expect("run ignore help");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), EXPECTED_HELP);
}

#[test]
fn prints_help_with_long_option() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ignore"))
        .arg("--help")
        .output()
        .expect("run ignore --help");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), EXPECTED_HELP);
}

#[test]
fn prints_help_with_short_option() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ignore"))
        .arg("-h")
        .output()
        .expect("run ignore -h");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), EXPECTED_HELP);
}
