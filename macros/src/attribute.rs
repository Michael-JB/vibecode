use crate::ai_responder::{AIResponder, Complexity};
use crate::openai::OpenAI;
use proc_macro::TokenStream;

pub fn impl_vibecode(
    complexity: &Complexity,
    signature: &str,
    prompt: Option<&str>,
) -> TokenStream {
    let openai = OpenAI::default().expect("Failed to create OpenAI client");

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
    );

    println!("Vibecoded function: {:?}", response);

    response
        .expect("No text found in response")
        .parse()
        .expect("Failed to parse vibecoded closure to a token stream")
}
