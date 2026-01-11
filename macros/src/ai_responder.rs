use darling::FromMeta;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AIError {
    #[error("API usage error: {0}")]
    ApiError(String),

    #[error("received invalid output from model: {0}")]
    ModelOutputError(String),
}

#[derive(Debug, FromMeta)]
pub enum Complexity {
    Low,
    Medium,
    High,
}

pub trait AIResponder {
    fn respond(
        &self,
        complexity: &Complexity,
        instructions: &str,
        input: &str,
    ) -> Result<String, AIError>;
}
