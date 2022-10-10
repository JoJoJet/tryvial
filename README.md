# `tryvial`

<!-- cargo-rdme start -->

A small crate for Ok-wrapping and try blocks.
This is compatible with [`Result`], [`Option`], and any type implementing the unstable [`std::ops::Try`] trait.

*This crate does not require nightly Rust.*

## Overview

The titular macro, [`tryvial`], is used to perform Ok-wrapping on the return value of a function.

Before:
```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter your name: ");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name)?;
    println!("Hello, {name}!");
    Ok(()) // this is ugly
}
```

After:
```rust
#[tryvial]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter your name: ");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name)?;
    println!("Hello, {name}!");
}
```

---

The macro [`try_block`] is an implementation of "try blocks" from nightly rust.

```rust
let result: Result<T, E> = try_block! {
   let a = do_one(x)?;
   let b = do_two(a)?;
   b
};
```

---

The macro [`wrap_ok`] simply wraps an expression with the "ok" variant for a given [`Try`] type.

```rust
assert_eq!(Some(42), wrap_ok!(42));
```

[`Try`]: std::ops::Try

<!-- cargo-rdme end -->

License: MIT or Apache-2.0
