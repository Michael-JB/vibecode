use proc_macro::TokenStream;
use syn::ExprClosure;

use crate::ai_responder::{AIResponder, Complexity};
use crate::openai::OpenAI;
use anyhow::Result;

pub fn generate_closure(complexity: &Complexity, prompt: &str) -> Result<ExprClosure> {
    let openai = OpenAI::default()?;

    let response = openai.respond(
        complexity,
        "Write a Rust closure for the given task. You must ONLY return the closure without any explanation or wrapping code.",
        prompt,
    )?;

    eprintln!("--- Vibecoded closure ---");
    eprintln!("{}", response);

    let lexed_closure: TokenStream = response
        .parse()
        .map_err(|e| anyhow::anyhow!("Failed to lex vibecoded closure to a token stream: {}", e))?;

    syn::parse(lexed_closure)
        .map_err(|e| anyhow::anyhow!("Failed to parse vibecoded closure: {}", e))
}
