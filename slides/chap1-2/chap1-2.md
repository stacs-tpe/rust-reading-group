% The Rust Programming Language
% Reading group -- Chapters 1 & 2
% Michael Young

# This week

0. Foreword & Introduction

1.  Getting Started

    1.1. Installation

    1.2. Hello, World!

    1.3. Hello, Cargo!

2. Programming a Guessing Game


# Foreword

- "Rust is great, get us."


# Introduction

- Rust is for everyone
- You can learn Rust
- Book is best read from cover to cover


# Chapter 1. Getting Started

# 1.1. Installation

- Install `rustup` and use it to install everything else
  - Avoid multiple installation hell!

- Debian: I used apt only to install `rustup`


# 1.2. Hello, World!

In `main.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

Compile and run with:

```bash
rustc main.rs
./main
```


# 1.3. Hello, Cargo!

- Rust's build system is called **Cargo**

- Like Python's `pip`, Java's `maven`, or Javascript's `node`

- Looks usable enough
- Create project with `cargo new`
- Metadata and dependencies ("crates") in `Cargo.toml`
- `cargo build` and `cargo run`

- Let's use this for tangible programming environment project!


# Criticism

Did we need to learn this tool this early?

- "Just add more dependencies" mindset is dangerous
- No substitute for proper understanding of algorithms and project structure


# Chapter 2. Programming a Guessing Game

- Teaching by example
- Various interesting features
- Some explained, some only hinted at

# Full code

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

# Imports

- Some commonly-used things are in the *prelude*
- Others we have to import deliberately

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;
```

- `rand`, `std`, `cmp`, `io` are all **libraries**
- `rand` is a library **crate** we must download using `Cargo.toml`
- `Ordering` is a type
- `Rng` is a **trait**, whatever one of those is (Chapter 10)


# Functions

```rust
fn main() {
```
- Simple syntax
- No OO weirdness (public static etc.)
- void by default, return type specified with `-> type` syntax
- `main` is executed first


# Printing

```rust
println!("Guess the number!");
```

- `!` indicates that `println` is a **macro**, whatever one of those is (Chapter 20)

```rust
println!("You guessed: {guess}");
```

- Placeholders built in by default:
  - `println!("I am {age}");`
  - `println!("I am {}", age);`


# Variables

- Declare a new variable with `let`

```rust
let secret_number = 37;
```

- Type inferred from context, but could specify with a **type annotation**:

```rust
let secret_number: u32 = 37;
```

- Immutable by default, but can specify mutability with `mut`:

```rust
let mut guess = String::new();
```

- Nice!


# Ranges

- Nice tight range syntax

```rust
let secret_number = rand::thread_rng().gen_range(1..=100);
```

- Looks like `start..end` for exclusive and `start..=end` for inclusive



# Loop syntax

- `loop` means `while True`

```rust
loop {
    ...
    break;
}
```


# User input

```rust
io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
```

- `read_line` writes the result into `guess`
- `&` represents a **reference**, whatever one of those is (Chapter 4)
- References are immutable by default, so we need `&mut`


# Error handling

```rust
io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
```

- `read_line` returns a `Result` value (either `Ok` or `Err`)
- We can ignore the result, but the compiler will warn us!
- Two ways to handle a `Result`:
  - `expect` (like an assertion, crashes the program if it fails)
  - `match` (write explicit branches for each case)


# Matches

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

- Like switch expressions in modern Java
- One branch for each value of an enum
- Can return a value


# Type inference and parsing

- Some hot stuff with types

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

- `guess` is a String
- `guess.trim()` is a String
- `parse` turns the String into **something**
- Compiler decides what `parse` method to use based on required type in context: `u32`
  - Needs the type annotation to compile
- Actually returns a `Result<u32, Err>` which we then match


# Shadowing

```rust
let mut guess = String::new();
let guess: u32 = ...;
```

- New variable with old name
- Can even have a different type
- "Shadows" the old variable which we can't use any more
- Error prone? Care needed?


# Ordering enum

- Match other enums just like `Result`s
- `Ordering` represents the set {<, =, >}
- `guess.cmp` returns an `Ordering` object, which we branch on

```rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => {
        println!("You win!");
        break;
    }
}
```


# Full code

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```


# WTF are...

- Traits?
- Macros?
- References?
- `use rand::Rng;` which we never used


# Next week?

3.  Common Programming Concepts

4.  Understanding Ownership
