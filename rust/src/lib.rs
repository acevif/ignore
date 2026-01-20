pub mod adapters;
pub mod domain;
pub mod ports;
pub mod usecases;

pub use adapters::{ReqwestGitignoreSource, SerdeNorwayIgnorefileParser};
pub use domain::IgnoreConfig;
pub use ports::{parse_ignorefile, IgnorefileParseError, IgnorefileParser};
pub use usecases::{generate_gitignore, GenerateGitignoreError};
