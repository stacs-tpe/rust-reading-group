
### Matching with ```Option<T>```

The same idea as before but now the arms are ```None``` and ```Some(T)``` as these are the variants of Option.

This is useful for handling when we have the something there and when there isnt, for example we couldnt add `Option<i8>` to `i8` directly because of the ```null``` case so we handle it like so.

```rust,editable
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
```


