use thiserror::Error;

#[derive(Error, Debug)]
pub enum AIError {
    #[error("network failure: {0}")]
    NetworkError(String),

    #[error("received invalid output from model: {0}")]
    ModelOutputError(String),
}

pub trait AIResponder {
    fn respond(&self, model: &str, instructions: &str, input: &str) -> Result<String, AIError>;
}
