
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

