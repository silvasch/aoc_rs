#[derive(Debug, thiserror::Error)]
pub enum AocError {
    #[error("this solution is not implemented yet")]
    NotImplemented,
    #[error("failed to read the input file: '{0}'")]
    InputReadFailed(String),
    #[error("got wrong result: expected {0}, got {1}")]
    TestFailed(i64, i64),
    #[error("failed to compute solution: {0}")]
    SolutionFailed(String),
}
