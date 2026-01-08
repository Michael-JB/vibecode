use anyhow::{Result, anyhow};
use proc_macro::TokenStream;
use syn::{ExprClosure, ItemFn};

use crate::ai_responder::{AIResponder, Complexity};
use crate::openai::OpenAI;

pub fn populate_function(
    complexity: &Complexity,
    signature: &str,
    prompt: Option<&str>,
) -> Result<ItemFn> {
    let openai = OpenAI::default()?;

    let input = match prompt {
        Some(p) => format!("Function signature:\n{signature}\n\nAdditional information:\n{p}",),
        None => format!("Function signature:\n{signature}"),
    };

    let response = openai.respond(
        complexity,
        "Implement the given function in Rust. You must ONLY return the implementation code without any explanation.",
        &input,
    )?;

    eprintln!("--- Vibecoded function ---");
    eprintln!("{response}");

    // We lex & parse the vibecoded function to catch and wrap any syntax errors
    let lexed: TokenStream = response
        .parse()
        .map_err(|e| anyhow!("Failed to lex vibecoded function: {e}"))?;
    let parsed: ItemFn =
        syn::parse(lexed).map_err(|e| anyhow!("Failed to parse vibecoded function: {e}"))?;
    Ok(parsed)
}

pub fn generate_closure(complexity: &Complexity, prompt: &str) -> Result<ExprClosure> {
    let openai = OpenAI::default()?;

    let response = openai.respond(
        complexity,
        "Write a Rust closure for the given task. You must ONLY return the closure without any explanation or wrapping code.",
        prompt,
    )?;

    eprintln!("--- Vibecoded closure ---");
    eprintln!("{response}");

    syn::parse(
        response
            .parse()
            .map_err(|e| anyhow!("Failed to lex vibecoded closure: {e}"))?,
    )
    .map_err(|e| anyhow!("Failed to parse vibecoded closure: {e}"))
}
