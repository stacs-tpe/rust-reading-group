# Week X: Error Handling in Rust

<i>Slide template taken from Sylvain Wallez' [Rust intro](https://github.com/swallez/introduction-to-rust?tab=readme-ov-file). This is rendered through [Mdbook](https://rust-lang.github.io/mdBook/) (which is how the Rust book is created)</i>.

![](media/rust-logo-blk.svg)

</div>

-----------------

# This Week's Content

- **Error Handling in Rust**
  - Unrecoverable vs Recoverable Errors
  - Using `panic!`
  - Using `Result<T, E>`
  - `unwrap` and `expect`
  - Error propagation with `?`
  - Guidelines for when to panic

<center>

![](media/ferris.gif)

</center>

-----------------

# Error Handling Philosophy

- Rust forces you to **acknowledge** the possibility of errors.
- Program must handle errors **before** it will compile.
- Two kinds of errors:
  - **Unrecoverable** → use `panic!`
  - **Recoverable** → use `Result<T, E>`

-----------------

# Unrecoverable Errors with `panic!`

- Use `panic!` macro when program cannot continue.
- Panics:
  - Print a failure message
  - Unwind and clean up the stack
  - Quit the program
- Can configure to **abort** instead of unwind for smaller builds:

```toml
[profile.release]
panic = 'abort'
```

An example of calling the `panic!` macro is

```rust
fn main() {
    panic!("crash and burn");
}
```

When you compile the program you get something like:

```bash
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/panic`
thread 'main' panicked at src/main.rs:2:5:
crash and burn
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

-----------------

# Example: Out-of-Bounds Access

```rust
fn main() {
    let v = vec![1, 2, 3];
    v[99];
}
```

Result:

```bash
thread 'main' panicked at src/main.rs:4:6:
index out of bounds: the len is 3 but the index is 99
```

Use `RUST_BACKTRACE=1` to get full backtrace info.

```bash
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at src/main.rs:4:6:
index out of bounds: the len is 3 but the index is 99
stack backtrace:
   0: rust_begin_unwind
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/std/src/panicking.rs:662:5
   1: core::panicking::panic_fmt
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/core/src/panicking.rs:74:14
   2: core::panicking::panic_bounds_check
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/core/src/panicking.rs:276:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/core/src/slice/index.rs:302:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/core/src/slice/index.rs:16:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/alloc/src/vec/mod.rs:2920:9
   6: panic::main
             at ./src/main.rs:4:6
   7: core::ops::function::FnOnce::call_once
             at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

```

-----------------

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

-----------------

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

-----------------

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

-----------------

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

-----------------

# Error Propagation with `?` Operator

Without `?`:

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```

- `?` automatically returns the error if it happens, no need for `match`.

-----------------

# Chaining with `?`

You can chain calls elegantly:

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
```

- Similar to **optional chaining** in Swift/JavaScript.

-----------------

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

-----------------

# To Panic or Not to Panic?

Use `panic!` when:

- You are in a **bad state**.
- Code **relies** on correct assumptions.
- Error is **unexpected**.
- Cannot easily encode the requirement in types.

Otherwise: propagate errors with `Result`.

-----------------

# Input Validation with New Types

Encapsulate validation logic inside constructors:

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```

-----------------

# Summary

- Rust encourages **proactive** error handling.
- `panic!` is for catastrophic errors.
- `Result` is for recoverable problems.
- `?` makes error propagation **easy** and **clean**.
- Thoughtful error design leads to more reliable programs.
