
### ```Option``` Enum

Note - I have seen this in quite a lot of rust code so I think this is quite important

Its basically a type where it could be something (i.e not ```null```) or ```null```.

I suppose an example of this would be popping from a stack (to be honest this would probally throw an error before I imagine)?

Why do we need this ? ->
<b> RUST DOESNT HAVE A NULL TYPE ...</b> - What are peoples thoughts on this? 

Option is defined as -
```rust,editable
enum Option<T> {   // we dont need to bring into scope explicitly
    None,       // We can call these directly if we want - if so can we not just think of None as null?
    Some(T), // T is a generic type parameter (we find out about this later) - T can hold any type
}
```

