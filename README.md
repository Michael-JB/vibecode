# vibecode

Tired of fast, reproducible and secure builds? Introducing compile-time vibe
coding!

```rust
use vibecode::vibecode;

#[vibecode]
fn sum(xs: Vec<i32>) -> i32 {}

let result = sum(vec![27, 14, 42, -4, 32]);

assert_eq!(result, 111); // That's numberwang!
```

## Tell me more!

You can also give more context via an explicit prompt, and the complexity you
configure determines the model used to generate the code:

```rust
use vibecode::vibecode;

#[vibecode(prompt = "Return sorted in descending order", complexity = "medium")]
fn prime_numbers_below_limit(limit: u64) -> Vec<u64> {}

let result = prime_numbers_below_limit(20);

assert_eq!(result, vec![19, 17, 13, 11, 7, 5, 3, 2]);
```

You can also generate and evaluate code inline with `viberun!`:

```rust
use vibecode::viberun;

let my_number = 5;

let result = viberun!("Calculate the factorial of a number", my_number);

assert_eq!(result, 120);
```

## TODOs

- [x] Support attribute macro for function signatures
- [x] Take user prompt as input to attribute macro
- [x] Add function macro that takes user prompt as input
- [x] Support configurable complexity in attribute macro
- [ ] Support configurable complexity in function macro
- [ ] Support injection of OpenAI-compatible API client
