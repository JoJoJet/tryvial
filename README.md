# `tryvial`

<!-- cargo-rdme start -->

A small crate for Ok-wrapping and try blocks.
This is compatible with [`Result`](https://doc.rust-lang.org/stable/core/result/enum.Result.html), [`Option`](https://doc.rust-lang.org/stable/core/option/enum.Option.html),
and any type implementing the unstable [`std::ops::Try`](https://doc.rust-lang.org/std/ops/trait.Try.html) trait.

*This crate does not require nightly Rust.*

## Overview

The titular macro, `tryvial`, is used to perform Ok-wrapping on the return value of a function.

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

The macro [`try_block`](https://docs.rs/tryvial/latest/tryvial/macro.try_block.html) is an implementation of "try blocks" from nightly rust.

```rust
let result: Result<T, E> = try_block! {
   let a = do_one(x)?;
   let b = do_two(a)?;
   b
};
```

---

The macro [`wrap_ok`](https://docs.rs/tryvial/latest/tryvial/macro.wrap_ok.html) simply wraps an expression with the "ok" variant for a given `Try` type.

```rust
assert_eq!(Some(42), wrap_ok!(42));
```

<!-- cargo-rdme end -->

## License

MIT or Apache-2.0
