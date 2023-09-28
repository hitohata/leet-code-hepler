use thiserror::Error;

#[derive(Error, Debug)]
pub enum LeetCodeHelperError {
    #[error("Detected file name is separated to {0} parts, but required lass than 3 parts")]
    ProblemFileNameLengthError(usize),
}
