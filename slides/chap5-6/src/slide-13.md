
### When we use enums over structs?
<ul>
<li>
Say we dont know all attributes in advance only know type.
</li>
<li>
Are able to have different types for variants (cant do this with a struct).
</li>
</ul>

```rust,editable
// their example
enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
```
But from my understanding anything that can be done with an enum can be replicated with multiple structs (obviously a bit longer to express).
We can also use methods with enums (with ```imp```)??? 

