use crate::ai_responder::{AIError, AIResponder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Auth {
    pub api_key: String,
}

impl Auth {
    pub fn from_env() -> Result<Self, String> {
        let api_key =
            std::env::var("OPENAI_API_KEY").map_err(|_| "Missing OPENAI_API_KEY".to_string())?;
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
                _ => None,
            })
            .flatten()
            .find_map(|item| match item {
                Content::OutputText { text } => Some(text.as_str()),
            })
    }
}

impl OpenAI {
    pub fn default() -> Result<OpenAI, String> {
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
        AIError::NetworkError(err.to_string())
    }
}

impl AIResponder for OpenAI {
    fn respond(&self, model: &str, instructions: &str, input: &str) -> Result<String, AIError> {
        let request = Request {
            model: model.to_string(),
            instructions: instructions.to_string(),
            input: input.to_string(),
        };

        let response = ureq::post(format!("{}/responses", self.url))
            .header("Content-Type", "application/json")
            .header("Authorization", &format!("Bearer {}", self.auth.api_key))
            .send_json(&request)?
            .body_mut()
            .read_json::<Response>()?;

        response
            .first_text()
            .map(|s| s.to_string())
            .ok_or(AIError::ModelOutputError(
                "No text found in response".to_string(),
            ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_generate_response() {
        // Given
        let openai = OpenAI::default().unwrap();
        let model = "gpt-5-nano";
        let instructions = "Repeat the input word.";
        let magic_word = "Pike";

        // When
        let response = openai.respond(model, instructions, magic_word);

        // Then
        assert_eq!(response.unwrap(), magic_word);
    }
}
