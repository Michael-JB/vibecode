use crate::ai_responder::{AIResponder, Complexity};
use crate::openai::OpenAI;
use anyhow::Result;

pub fn populate_function(
    complexity: &Complexity,
    signature: &str,
    prompt: Option<&str>,
) -> Result<String> {
    let openai = OpenAI::default()?;

    let input = match prompt {
        Some(p) => format!(
            "Function signature:\n{}\n\nAdditional information:\n{}",
            signature, p
        ),
        None => format!("Function signature:\n{}", signature),
    };

    let response = openai.respond(
        complexity,
        "Implement the given function in Rust. You must ONLY return the implementation code without any explanation.",
        &input,
    )?;

    eprintln!("--- Vibecoded function ---");
    eprintln!("{}", response);

    Ok(response)
}
