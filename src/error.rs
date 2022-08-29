use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    InvalidSize(String),
    #[error("Wrong size: {given} given should be {expected}")]
    WrongSize { given: usize, expected: usize },
}
