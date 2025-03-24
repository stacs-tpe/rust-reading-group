
### Option examples

Rust will infer the type T. 
```rust,editable
    let some_number = Some(5);      // infers i32
    let some_char = Some('e');      // infers char

    let absent_number: Option<i32> = None;  // of type Option<i32> as we dont know what type some would be
```
But we need to note ```Option<T> != T```! meaning we cant do normal type operations with Option if we have defined T with Some(value) (eg cannot add `Option<i8>` to `i8`). To fix this we must covert ```Option<T>``` to ```T``` by ensuring its not null. This means if we see a type that isnt `Option<T>` we know it cant be null! 

<img alt="" class="bh gt pa c" width="498" height="301" loading="eager" role="presentation" src="https://miro.medium.com/v2/resize:fit:996/1*EyBu3xyslFozyfa_UUlK0w.gif">


