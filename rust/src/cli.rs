#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Help,
    Version,
    Update,
    Unknown(String),
}

pub fn resolve(args: &[String]) -> Action {
    let is_help = args
        .iter()
        .any(|arg| arg == "help" || arg == "--help" || arg == "-h");
    if is_help {
        return Action::Help;
    }

    let is_version = args.iter().any(|arg| arg == "version" || arg == "--version");
    if is_version {
        return Action::Version;
    }

    match args.first().map(|s| s.as_str()) {
        None => Action::Update,
        Some("update") => Action::Update,
        Some(other) => Action::Unknown(other.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_empty_args_to_update_shortcut() {
        let args: Vec<String> = vec![];
        assert_eq!(resolve(&args), Action::Update);
    }
}

