
## Enums
These are used when defining types to <i>enumerate</i> through its different variants.

These are in other prog languages right? (python / java ect...)

We call then through ```::``` i.e we namespace it up its id.
<i>example</i>
```rust,editable
// From book
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;      // double colon for specific kind or single colon for any of that type
```

