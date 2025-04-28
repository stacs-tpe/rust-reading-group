
# Recoverable Errors with `Result`

- Recoverable errors use the `Result<T, E>` enum:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

- Example:

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");
}
```

