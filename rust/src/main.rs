mod cli;

use ignore_rs::{generate_gitignore, parse_ignorefile, ReqwestGitignoreSource, SerdeNorwayIgnorefileParser};

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

    // TODO: Write `.gitignore` atomically (temp file next to the target, write+fsync, then
    // replace/rename). This is the common best practice (e.g. Git's lockfile + rename workflow),
    // but it swaps the inode and therefore breaks hard links. There is no general way to update
    // an inode "in place" atomically, so either accept/declare that hard links are not supported,
    // or choose a different (non-atomic) strategy with locking.
    //
    // Implementation candidates: `tempfile::NamedTempFile::new_in(".")` + `persist`, or an atomic
    // write crate (keep MSRV/rustc 1.70 constraints in mind).
    std::fs::write(".gitignore", generated)
        .map_err(|e| format!("failed to write .gitignore: {e}"))?;
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
