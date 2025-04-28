
# Matching on `Result`

```rust
use std::fs::File;

fn main() {
    let greeting_file = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
}
```

- If `hello.txt` doesn't exist, program will panic.

