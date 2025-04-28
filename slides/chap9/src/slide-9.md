
# Panic on Error: `unwrap` and `expect`

- `unwrap()` returns the value if `Ok`, else panics.
- `expect("message")` does the same, but **with a custom error message**.

Prefer `expect()` over `unwrap()` for better error diagnostics!

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
```

