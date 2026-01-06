# vibecode

Compile-time vibe coding!

```rust
use vibecode::vibecode;

#[vibecode]
fn sum(xs: Vec<i32>) -> i32 {}

let result = sum(vec![27, 14, 42, -4, 32]);

assert_eq!(result, 111); // That's numberwang!
```

## TODOs

- [x] Support attribute macro for function signatures
- [ ] Take user prompt as input to attribute macro
- [ ] Add function macro that takes user prompt as input
- [ ] Support injection of OpenAI-compatible API client
