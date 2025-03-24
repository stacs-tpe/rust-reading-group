
### Using structs
We create a variable and call the struct but adding concrete values after each name and can call by using attribute notation in other languages like java.

```rust,editable
let student1 = CS_Student {
    // Doesnt need to be in order stated
    name: String::from("Jake"),
    email: String::from("ja250@st-andrews.ac.uk"),
    coffee_addict: true,
    total_commits: 400,
}
// grab attributes like so
let jakes_email = student1.email;

// Test - why would this NOT work?
student1.email = String::from("other0@st-andrews.ac.uk");
```

