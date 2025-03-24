
### ```Match``` Control flow construct

A bit like a more powerful version of Switch in Python / Java. 

```rust,editable

// Example they gave - helps us tell what variant of the type it is
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,       // These are the arms (have talked about this already)
        Coin::Nickel => 5,
        Coin::Dime => 10,       // for multiline return statements on an arm use => {... u8} notation
        Coin::Quarter => 25,
    }
}
```

