# vibecode

[![Docs](https://docs.rs/vibecode/badge.svg)](https://docs.rs/vibecode/)
[![Crates.io Version](https://img.shields.io/crates/v/vibecode)](https://crates.io/crates/vibecode)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/vibecode)](https://crates.io/crates/vibecode)

Tired of fast, reproducible and secure builds? Try compile-time vibe coding.

```rust
use vibecode::vibecode;

#[vibecode]
fn absolute_sum(xs: Vec<i32>) -> u64 {}

assert_eq!(absolute_sum(vec![-41, 42, 28]), 111); // That's numberwang!
```

## Tell me more!

Add vibecode to your project:

```bash
cargo add vibecode
```

At present, vibecode only supports OpenAI; you'll need to set the
`OPENAI_API_KEY` environment variable to compile your code.

### vibecode

The `vibecode` attribute macro generates the body of the annotated function at
compile time using an LLM.

You can provide more context via an explicit prompt. The complexity you
configure determines the model `vibecode` uses to generate the code:

```rust
use vibecode::vibecode;

#[vibecode(prompt = "Return sorted in descending order", complexity = "medium")]
fn prime_numbers_below(limit: u64) -> Vec<u64> {}

assert_eq!(prime_numbers_below(12), vec![11, 7, 5, 3, 2]);
```

### viberun!

You can also generate and evaluate code inline with `viberun!`:

```rust
use vibecode::viberun;

let my_number = 5;

let result = viberun!("Calculate the factorial of a number", my_number);

assert_eq!(result, 120);
```

## License

[MIT License](LICENSE)
