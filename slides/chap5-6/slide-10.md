
### Example
The docs had an example. There is no point me re stating their example but would be nice to hear peoples peoples thoughts?

An example I thought of was in our Knight-Tour we could potentially have a Position (position on board) struct.

```rust,editable
#[derive(Debug)]
// This by itself is slightly pointless (compared to tuple) 
// ... but will make more sence when we add methods
struct Position {
    x: u8,
    y: u8,
}
```

