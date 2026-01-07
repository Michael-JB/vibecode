use syn::ExprClosure;

use crate::ai_responder::{AIResponder, Complexity};
use crate::openai::OpenAI;

pub fn impl_viberun(complexity: &Complexity, prompt: &str) -> ExprClosure {
    let openai = OpenAI::default().expect("Failed to create OpenAI client");

    let response = openai.respond(
        complexity,
        "Write a Rust closure for the given task. You must ONLY return the closure without any explanation or wrapping code.",
        prompt,
    );

    println!("Vibecoded closure: {:?}", response);

    syn::parse(
        response
            .expect("No text found in response")
            .parse()
            .expect("Failed to parse vibecoded closure to a token stream"),
    )
    .expect("Failed to parse vibecoded closure")
}
