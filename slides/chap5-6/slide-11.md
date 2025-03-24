
### Methods
Intuitively Methods are basically the same as class methods in most OO languages.
We create them by using ```impl``` (can have multiple per struct).
No such thing as getters and setters in Rust.
We use ```object.method()``` notation (dont need -> as rust automatically matches signature of method!).
<i>Associated functions</i> = methods that dont reference struct (i.e dont have self in params). Often use when we want to create new instance of struct.
```rust,editable
impl Position {
    // Check if the position is within the board boundaries
    fn valid(&self) -> bool {   // Using &self as we just want copy of attributes (not ownership)
        (0..WIDTH).contains(&self.x) && (0..WIDTH).contains(&self.y)
    }
```
