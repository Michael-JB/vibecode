# vibecode

Compile-time vibe coding!

```rust
use vibecode::vibecode;

#[vibecode]
fn sum(xs: Vec<i32>) -> i32 {}

let result = sum(vec![27, 14, 42, -4, 32]);

assert_eq!(result, 111); // That's numberwang!
```

You can also give more context via an explicit prompt:

```rust
use vibecode::vibecode;

#[vibecode(prompt = "Sort in descending order")]
fn special_sort(xs: Vec<u64>) -> Vec<u64> {}

let result = special_sort(vec![1, 4, 2, 3, 1]);

assert_eq!(result, vec![4, 3, 2, 1, 1]);
```

The complexity you configure determines the model used to generate the code:

```rust
use vibecode::vibecode;

#[vibecode(complexity = "medium")]
fn prime_numbers_below_limit(limit: u64) -> Vec<u64> {}

let result = prime_numbers_below_limit(20);

assert_eq!(result, vec![2, 3, 5, 7, 11, 13, 17, 19]);
```

You can also generate and evaluate code inline with `viberun!`:

```rust
use vibecode::viberun;

let result = viberun!("Calculate the factorial of a number", 5);

assert_eq!(result, 120);
```

## TODOs

- [x] Support attribute macro for function signatures
- [x] Take user prompt as input to attribute macro
- [x] Add function macro that takes user prompt as input
- [x] Support configurable complexity in attribute macro
- [ ] Support configurable complexity in function macro
- [ ] Support injection of OpenAI-compatible API client
