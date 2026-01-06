use proc_macro::TokenStream;

use crate::ai_responder::AIResponder;
use crate::openai::OpenAI;

pub fn impl_vibecode(signature: &str, prompt: Option<&str>) -> TokenStream {
    let openai = OpenAI::default().expect("Failed to create OpenAI client");

    let input = match prompt {
        Some(p) => format!(
            "Function signature:\n{}\n\nAdditional information:\n{}",
            signature, p
        ),
        None => format!("Function signature:\n{}", signature),
    };

    let response = openai.respond(
        "gpt-5-nano",
        "Implement the given function in Rust. You must ONLY return the implementation code without any explanation.",
        &input,
    );

    println!("Vibecoded function:\n{:?}", response);

    response
        .expect("No text found in response")
        .parse()
        .expect("Failed to parse vibecoded function")
}
