
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

