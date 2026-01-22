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
    assert!(
        stderr.contains("Ignorefile"),
        "stderr did not mention Ignorefile"
    );
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

    let mut perms = std::fs::metadata(dir.path())
        .expect("stat temp dir")
        .permissions();
    let original_mode = perms.mode();
    perms.set_mode(0o555);
    std::fs::set_permissions(dir.path(), perms).expect("make temp dir read-only");

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

    let mut perms = std::fs::metadata(dir.path())
        .expect("stat temp dir")
        .permissions();
    perms.set_mode(original_mode);
    std::fs::set_permissions(dir.path(), perms).expect("restore temp dir permissions");

    let after = std::fs::read_to_string(&gitignore_path).expect("read .gitignore after failure");
    assert_eq!(after, original);
}

#[test]
#[cfg(unix)]
fn update_preserves_gitignore_symlink() {
    use std::os::unix::fs::symlink;

    let dir = tempfile::tempdir().expect("create temp dir");

    std::fs::write(dir.path().join("Ignorefile"), IGNOREFILE_PATHS_ONLY).expect("write Ignorefile");

    let target_path = dir.path().join("linked.gitignore");
    std::fs::write(&target_path, "# old\n").expect("write linked.gitignore");

    symlink("linked.gitignore", dir.path().join(".gitignore")).expect("create .gitignore symlink");

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

    let meta = std::fs::symlink_metadata(dir.path().join(".gitignore")).expect("stat .gitignore");
    assert!(meta.file_type().is_symlink(), ".gitignore is not a symlink");

    let generated = std::fs::read_to_string(&target_path).expect("read linked.gitignore");
    assert_eq!(generated, EXPECTED_GITIGNORE_PATHS_ONLY);
}

#[test]
fn update_succeeds_with_empty_ignorefile() {
    let dir = tempfile::tempdir().expect("create temp dir");

    std::fs::write(dir.path().join("Ignorefile"), "").expect("write empty Ignorefile");

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

    let generated =
        std::fs::read_to_string(dir.path().join(".gitignore")).expect("read generated .gitignore");

    let expected = "\
# This .gitignore file is auto-generated. Do not edit!\n\
# Edit Ignorefile instead.\n\
\n\
\n\
### Project specific settings ###\n\
\n";

    assert_eq!(generated, expected);
}
