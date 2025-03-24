
### Other neat features when declaring
- Can add pre known attributes (i.e dont need to init them).
    - If param has the same name as attribute dont can use ```attribute``` over ```attribute : attribute``` notation.
- Can reference other structures attributes using the ```..structure_name``` syntax.
    - Is this sort of like inheritance / overriding? 
    - If we have types like u64 or bool that have copy trait would just copy over!

```rust,editable
let student2 = CS_Student {
    name: String::from("julia"),
    email: String::from("julia@st-andrews.ac.uk"),
    ..student1  // take definitions for other attributes from student1
}
```


