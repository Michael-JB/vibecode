use syn::ExprClosure;

use crate::ai_responder::AIResponder;
use crate::openai::OpenAI;

pub fn impl_viberun(prompt: &str) -> ExprClosure {
    let openai = OpenAI::default().expect("Failed to create OpenAI client");

    let response = openai.respond(
        "gpt-5-nano",
        "Write a Rust closure for the given task. You must ONLY return the closure without any explanation or wrapping code.",
        prompt,
    );

    println!("Vibecoded closure: {:?}", response);

    syn::parse(
        response
            .expect("No text found in response")
            .parse()
            .expect("Failed to convert vibecoded closure to a token stream"),
    )
    .expect("Failed to parse vibecoded closure")
}
