
### Example

In our knights tour we had -

```rust,editable
match visited.iter().position(|a| a == &(x, y)) {
    Some(i) => print!("{:>3}", i),
    None => print!("  .")
};
```
Given what we have learnt we could use if let instead!
```rust,editable

if let Some(i) = visited.iter().position(|a| a == &(x, y)) {
    print!("{:>3}", i);
} else {
    print!("  .");
}
```

