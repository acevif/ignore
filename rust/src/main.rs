fn main() {
    const HELP: &str = "Usage: ignore <command>\n\nCommands:\n  help     Show this help message\n  version  Show version information\n\nOptions:\n  -h, --help     Show this help message\n  -V, --version  Show version information\n";

    let args: Vec<String> = std::env::args().skip(1).collect();
    let is_help = args
        .iter()
        .any(|arg| arg == "help" || arg == "--help" || arg == "-h");
    let is_version = args
        .iter()
        .any(|arg| arg == "version" || arg == "--version" || arg == "-V");

    if is_help {
        print!("{HELP}");
        return;
    }

    if is_version {
        println!("ignore {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    println!("Hello, world!");
}
