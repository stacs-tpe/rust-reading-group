
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

