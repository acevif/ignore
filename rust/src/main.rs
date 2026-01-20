mod cli;

use ignore_rs::{
    generate_gitignore, parse_ignorefile, ReqwestGitignoreSource, SerdeNorwayIgnorefileParser,
};
use std::io::Write;
use std::path::{Path, PathBuf};

fn resolve_symlink_target(mut path: PathBuf) -> Result<PathBuf, String> {
    for _ in 0..8 {
        let meta = match std::fs::symlink_metadata(&path) {
            Ok(meta) => meta,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(path),
            Err(e) => return Err(format!("failed to stat {}: {e}", path.display())),
        };

        if !meta.file_type().is_symlink() {
            return Ok(path);
        }

        let link = std::fs::read_link(&path)
            .map_err(|e| format!("failed to read symlink {}: {e}", path.display()))?;

        let base = path
            .parent()
            .ok_or_else(|| format!("symlink has no parent directory: {}", path.display()))?;

        path = if link.is_absolute() {
            link
        } else {
            base.join(link)
        };
    }

    Err("symlink chain too deep (possible loop)".to_string())
}

fn write_file_atomically(path: &Path, contents: &str) -> Result<(), String> {
    let target = resolve_symlink_target(path.to_path_buf())?;

    let parent = target
        .parent()
        .ok_or_else(|| format!("target path has no parent: {}", target.display()))?;

    let mut temp = tempfile::NamedTempFile::new_in(parent)
        .map_err(|e| format!("failed to create temp file in {}: {e}", parent.display()))?;

    temp.as_file_mut()
        .write_all(contents.as_bytes())
        .map_err(|e| format!("failed to write temp file: {e}"))?;
    temp.as_file()
        .sync_all()
        .map_err(|e| format!("failed to fsync temp file: {e}"))?;

    temp.persist(&target)
        .map_err(|e| format!("failed to replace {}: {}", target.display(), e.error))?;

    #[cfg(unix)]
    {
        if let Ok(dir) = std::fs::File::open(parent) {
            let _ = dir.sync_all();
        }
    }

    Ok(())
}

async fn run_update() -> Result<(), String> {
    let ignorefile_path = std::path::Path::new("Ignorefile");
    let ignorefile = std::fs::File::open(ignorefile_path)
        .map_err(|e| format!("failed to read Ignorefile: {e}"))?;

    let parser = SerdeNorwayIgnorefileParser::default();
    let config = parse_ignorefile(&parser, ignorefile).map_err(|e| e.to_string())?;

    let source = ReqwestGitignoreSource::new();
    let generated = generate_gitignore(&config, &source)
        .await
        .map_err(|e| e.to_string())?;

    // NOTE: Atomic replacement swaps the inode and therefore breaks hard links. This is an
    // acceptable trade-off, but we preserve symlinks by writing to the resolved target path.
    write_file_atomically(std::path::Path::new(".gitignore"), &generated)?;
    Ok(())
}

#[tokio::main]
async fn main() {
    const HELP: &str = "Usage: ignore [<command>]\n\nCommands:\n  help     Show this help message\n  update   Generate .gitignore from Ignorefile\n  version  Show version information\n\nOptions:\n  -h, --help  Show this help message\n  --version   Show version information\n";

    let args: Vec<String> = std::env::args().skip(1).collect();

    match cli::resolve(&args) {
        cli::Action::Help => {
            print!("{HELP}");
        }
        cli::Action::Version => {
            println!("ignore {}", env!("CARGO_PKG_VERSION"));
        }
        cli::Action::Update => {
            if let Err(error) = run_update().await {
                eprintln!("{error}");
                std::process::exit(1);
            }
        }
        cli::Action::Unknown(command) => {
            eprintln!("unknown command: {command}\n\n{HELP}");
            std::process::exit(1);
        }
    }
}
