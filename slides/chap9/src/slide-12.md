
# Where Can You Use `?`

- Only in functions that return a `Result` (or compatible type).
- Compiler will complain otherwise.


This is really useful and means that you can write code like:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

```

