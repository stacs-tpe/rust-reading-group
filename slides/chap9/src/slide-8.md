
# Handling Specific Kinds of Errors
An error variant struct can represent more than one kind of error. The struct has a method called `kind` that returns a `io::ErrorKind` that you can then match on.

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => panic!("Problem opening the file: {other_error:?}"),
        },
    };
}
```

