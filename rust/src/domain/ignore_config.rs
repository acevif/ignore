#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IgnoreConfig {
    pub gitignore_io: Vec<String>,
    pub github: Vec<String>,
    pub paths_ignore: Vec<String>,
}
