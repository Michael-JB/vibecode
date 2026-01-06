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

You can also generate and evaluate code inline:

```rust
use vibecode::viberun;

let result = viberun!("Calculate the factorial of a number", 5);

assert_eq!(result, 120);
```

## TODOs

- [x] Support attribute macro for function signatures
- [x] Take user prompt as input to attribute macro
- [x] Add function macro that takes user prompt as input
- [ ] Support injection of OpenAI-compatible API client
