use thiserror::Error;

#[derive(Error, Debug)]
pub enum LeetCodeHelperError {
    #[error("Detected file name is separated to {0} parts, but required lass than 3 parts")]
    ProblemFileNameLengthError(usize),
    #[error("Extension error. required {0} provided {1}")]
    ExtensionMismatchError(String, String),
    #[error("IO error")]
    IoError(#[source] std::io::Error),
    #[error("Toml error")]
    TomlError(#[source] toml_edit::TomlError),
    #[error("reading file error")]
    DirError(#[source] std::fs::ReadDir::Error),
}
