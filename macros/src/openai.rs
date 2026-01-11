use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

use crate::ai_responder::{AIError, AIResponder, Complexity};

#[derive(Serialize, Deserialize)]
pub struct Auth {
    pub api_key: String,
}

impl Auth {
    pub fn from_env() -> Result<Self> {
        let api_key = std::env::var("OPENAI_API_KEY")
            .map_err(|e| anyhow!("Failed to read OPENAI_API_KEY from environment: {e}"))?;
        Ok(Auth { api_key })
    }
}

pub struct OpenAI {
    pub auth: Auth,
    pub url: String,
}

#[derive(Serialize)]
struct Request {
    model: String,
    instructions: String,
    input: String,
    reasoning: Reasoning,
}

#[derive(Serialize)]
struct Reasoning {
    effort: String,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum Content {
    #[serde(rename = "output_text")]
    OutputText { text: String },
}

#[derive(Deserialize)]
pub struct Response {
    output: Vec<Output>,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Output {
    Reasoning {},
    Message { content: Vec<Content> },
}

impl Response {
    pub fn first_text(&self) -> Option<&str> {
        self.output
            .iter()
            .filter_map(|out| match out {
                Output::Message { content } => Some(content),
                Output::Reasoning {} => None,
            })
            .flatten()
            .map(|item| match item {
                Content::OutputText { text } => text.as_str(),
            })
            .next()
    }
}

impl OpenAI {
    pub fn default() -> Result<OpenAI> {
        let auth = Auth::from_env()?;
        Ok(OpenAI::new(auth, "https://api.openai.com/v1"))
    }

    pub fn new(auth: Auth, url: &str) -> OpenAI {
        OpenAI {
            auth,
            url: url.to_string(),
        }
    }
}

impl From<ureq::Error> for AIError {
    fn from(err: ureq::Error) -> Self {
        AIError::ApiError(err.to_string())
    }
}

impl From<&Complexity> for &str {
    fn from(complexity: &Complexity) -> Self {
        match complexity {
            Complexity::Low => "gpt-5-nano",
            Complexity::Medium => "gpt-5-mini",
            Complexity::High => "gpt-5.2",
        }
    }
}

impl AIResponder for OpenAI {
    fn respond(
        &self,
        complexity: &Complexity,
        instructions: &str,
        input: &str,
    ) -> Result<String, AIError> {
        let request = Request {
            model: <&str>::from(complexity).to_string(),
            instructions: instructions.to_string(),
            input: input.to_string(),
            reasoning: Reasoning {
                // TODO this should probably depend on complexity
                effort: "low".to_string(),
            },
        };

        let response = ureq::post(format!("{}/responses", self.url))
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", self.auth.api_key))
            .send_json(&request)?
            .body_mut()
            .read_json::<Response>()?;

        response
            .first_text()
            .map(std::string::ToString::to_string)
            .ok_or(AIError::ModelOutputError(
                "No text found in response".to_string(),
            ))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use mockito::Server;

    impl Auth {
        pub fn invalid() -> Self {
            Auth {
                api_key: "invalid".into(),
            }
        }
    }

    impl OpenAI {
        pub fn with_url(self, url: String) -> Self {
            OpenAI {
                auth: self.auth,
                url,
            }
        }

        pub fn with_auth(self, auth: Auth) -> Self {
            OpenAI {
                auth,
                url: self.url.clone(),
            }
        }
    }

    #[test]
    fn it_can_generate_response() {
        // Given
        let openai = OpenAI::default().unwrap();
        let complexity = Complexity::Low;
        let instructions = "Repeat the input word";
        let magic_word = "Pike";

        // When
        let response = openai.respond(&complexity, instructions, magic_word);

        // Then
        assert_eq!(response.unwrap(), magic_word);
    }

    #[test]
    fn it_handles_invalid_api_key() {
        // Given
        let openai = OpenAI::default().unwrap().with_auth(Auth::invalid());

        // When
        let result = openai.respond(&Complexity::Low, "instructions", "input");

        // Then
        assert!(matches!(result.unwrap_err(), AIError::ApiError(_)));
    }

    #[test]
    fn it_handles_api_error() {
        // Given
        let mut server = Server::new();
        server.mock("POST", "/responses").with_status(500).create();
        let openai = OpenAI::default().unwrap().with_url(server.url());

        // When
        let result = openai.respond(&Complexity::Low, "instructions", "input");

        // Then
        assert!(matches!(result.unwrap_err(), AIError::ApiError(_)));
    }

    #[test]
    fn it_handles_model_output_error() {
        // Given
        let mut server = Server::new();
        server
            .mock("POST", "/responses")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"output":[]}"#)
            .create();
        let openai = OpenAI::default().unwrap().with_url(server.url());

        // When
        let result = openai.respond(&Complexity::Low, "instructions", "input");

        // Then
        assert!(matches!(result.unwrap_err(), AIError::ModelOutputError(_)));
    }
}
