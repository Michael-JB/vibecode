use proc_macro::TokenStream;

use crate::ai_responder::AIResponder;
use crate::openai::OpenAI;

pub fn impl_vibecode(signature: &str) -> TokenStream {
    let openai = OpenAI::default().expect("Failed to create OpenAI client");

    let response = openai.respond(
        "gpt-5-mini",
        "Implement the given function in Rust. You must ONLY return the implementation code without any explanation.",
        signature,
    );

    println!("Vibecode generated function:\n{:?}", response);

    response
        .expect("No text found in response")
        .parse()
        .expect("Failed to parse vibecoded function")
}
