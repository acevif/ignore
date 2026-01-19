use std::process::Command;

const IGNOREFILE_PATHS_ONLY: &str = r#"
paths-ignore:
  - ".codex/"
  - ".claude/"
  - "mise.local.toml"
"#;

const EXPECTED_GITIGNORE_PATHS_ONLY: &str = "\
# This .gitignore file is auto-generated. Do not edit!\n\
# Edit Ignorefile instead.\n\
\n\
\n\
### Project specific settings ###\n\
\n\
.codex/\n\
.claude/\n\
mise.local.toml\n";

#[test]
fn update_generates_gitignore_from_paths_ignore_only() {
    let dir = tempfile::tempdir().expect("create temp dir");

    std::fs::write(dir.path().join("Ignorefile"), IGNOREFILE_PATHS_ONLY).expect("write Ignorefile");

    let output = Command::new(env!("CARGO_BIN_EXE_ignore"))
        .arg("update")
        .current_dir(dir.path())
        .output()
        .expect("run ignore update");

    assert!(
        output.status.success(),
        "status={}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    assert!(output.stdout.is_empty(), "stdout is not empty");
    assert!(output.stderr.is_empty(), "stderr is not empty");

    let generated =
        std::fs::read_to_string(dir.path().join(".gitignore")).expect("read generated .gitignore");

    assert_eq!(generated, EXPECTED_GITIGNORE_PATHS_ONLY);
}

#[test]
fn update_runs_when_called_without_args() {
    let dir = tempfile::tempdir().expect("create temp dir");

    std::fs::write(dir.path().join("Ignorefile"), IGNOREFILE_PATHS_ONLY).expect("write Ignorefile");

    let output = Command::new(env!("CARGO_BIN_EXE_ignore"))
        .current_dir(dir.path())
        .output()
        .expect("run ignore");

    assert!(
        output.status.success(),
        "status={}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    let generated =
        std::fs::read_to_string(dir.path().join(".gitignore")).expect("read generated .gitignore");

    assert_eq!(generated, EXPECTED_GITIGNORE_PATHS_ONLY);
}

#[test]
fn update_fails_when_ignorefile_missing() {
    let dir = tempfile::tempdir().expect("create temp dir");

    let output = Command::new(env!("CARGO_BIN_EXE_ignore"))
        .arg("update")
        .current_dir(dir.path())
        .output()
        .expect("run ignore update");

    assert!(
        !output.status.success(),
        "expected failure but got status={}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    assert!(!dir.path().join(".gitignore").exists());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Ignorefile"), "stderr did not mention Ignorefile");
}

#[test]
#[cfg(unix)]
fn update_does_not_clobber_existing_gitignore_when_write_fails() {
    use std::os::unix::fs::PermissionsExt;

    let dir = tempfile::tempdir().expect("create temp dir");

    std::fs::write(dir.path().join("Ignorefile"), IGNOREFILE_PATHS_ONLY).expect("write Ignorefile");

    let gitignore_path = dir.path().join(".gitignore");
    let original = "# keep\n";
    std::fs::write(&gitignore_path, original).expect("write existing .gitignore");

    let mut perms = std::fs::metadata(&gitignore_path)
        .expect("stat existing .gitignore")
        .permissions();
    perms.set_mode(0o444);
    std::fs::set_permissions(&gitignore_path, perms).expect("make .gitignore read-only");

    assert!(
        std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&gitignore_path)
            .is_err(),
        "precondition: expected .gitignore to be unwritable"
    );

    let output = Command::new(env!("CARGO_BIN_EXE_ignore"))
        .arg("update")
        .current_dir(dir.path())
        .output()
        .expect("run ignore update");

    assert!(
        !output.status.success(),
        "expected failure but got status={}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    let after = std::fs::read_to_string(&gitignore_path).expect("read .gitignore after failure");
    assert_eq!(after, original);
}
