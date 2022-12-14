//! A small crate for Ok-wrapping and try blocks.
//! This is compatible with [`Result`](::core::result::Result), [`Option`](::core::option::Option),
//! and any type implementing the unstable [`std::ops::Try`](https://doc.rust-lang.org/std/ops/trait.Try.html) trait.
//!
//! *This crate does not require nightly Rust.*
//!
//! # Overview
//!
//! The macro `try_fn` is used to perform Ok-wrapping on the return value of a function.
//!
//! Before:
//! ```
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     println!("Enter your name: ");
//!     let mut name = String::new();
//!     std::io::stdin().read_line(&mut name)?;
//!     println!("Hello, {name}!");
//!     Ok(()) // this is ugly
//! }
//! ```
//!
//! After:
//! ```
//! # use tryvial::try_fn;
//! #[try_fn]
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     println!("Enter your name: ");
//!     let mut name = String::new();
//!     std::io::stdin().read_line(&mut name)?;
//!     println!("Hello, {name}!");
//! }
//! ```
//!
//! ---
//!
//! The macro [`try_block`](crate::try_block) is an implementation of "try blocks" from nightly rust.
//!
//! ```
//! # use tryvial::try_block;
//! # type T = (); type E = ();
//! # fn do_one((): T) -> Result<T, E> { Ok(()) }
//! # fn do_two((): T) -> Result<T, E> { Ok(()) }
//! # let x = ();
//! let result: Result<T, E> = try_block! {
//!    let a = do_one(x)?;
//!    let b = do_two(a)?;
//!    b
//! };
//! ```
//!
//! ---
//!
//! The macro [`wrap_ok`](crate::wrap_ok) simply wraps an expression with the "ok" variant for a given `Try` type.
//!
//! ```
//! # use tryvial::wrap_ok;
//! assert_eq!(Some(42), wrap_ok!(42));
//! ```

#![no_std]

#[cfg(feature = "proc-macro")]
pub use tryvial_proc::{try_fn, tryvial};

/// Performs "Ok-wrapping" on the result of an expression.
/// This is compatible with [`Result`], [`Option`], [`ControlFlow`], and any type that
/// implements the unstable [`std::ops::Try`] trait.
///
/// The destination type must be specified with a type ascription somewhere.
#[macro_export]
macro_rules! wrap_ok {
    ($e:expr) => {
        ::core::iter::empty().try_fold($e, |_, __x: ::core::convert::Infallible| match __x {})
    };
}

/// Macro for the receiving end of a `?` operation.
///
/// ```
/// # use tryvial::try_block;
/// // Note: this fails without explicitly specifying the error type.
/// let y: Result<_, std::num::ParseIntError> = try_block! {
///     "1".parse::<i32>()? + "2".parse::<i32>()?
/// };
/// assert_eq!(y, Ok(3));
/// ```
#[macro_export]
macro_rules! try_block {
    { $($token:tt)* } => {
        (|| $crate::wrap_ok!({
            $($token)*
        }))()
    }
}

#[cfg(test)]
extern crate alloc;

#[cfg(test)]
mod tests {
    use super::*;
    use core::ops::ControlFlow;

    /// This is a doc comment.
    #[try_fn]
    /// And another one.
    pub fn with_doc_comments() -> ControlFlow<usize> {
        ControlFlow::Break(11)?;
    }

    #[test]
    fn test_with_doc() {
        assert!(matches!(with_doc_comments(), ControlFlow::Break(11)));
    }

    #[try_fn]
    unsafe fn generic_fn<T, U: Clone>(x: T, y: &U) -> ControlFlow<U>
    where
        T: PartialEq<U>,
    {
        if x == *y {
            ControlFlow::Break(y.clone())?;
        }
    }

    #[test]
    fn test_generic_fn() {
        use alloc::borrow::ToOwned;
        match unsafe { generic_fn("Hello, world", &"Hello, world".to_owned()) } {
            ControlFlow::Break(s) => assert_eq!(s, "Hello, world"),
            ControlFlow::Continue(()) => unreachable!(),
        }
    }

    struct MyStruct(u32);

    impl core::convert::TryFrom<&str> for MyStruct {
        type Error = core::num::ParseIntError;
        #[try_fn]
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            Self(value.parse()?)
        }
    }

    #[test]
    fn test_parse() {
        assert!(matches!("34".try_into(), Ok(MyStruct(34))));
    }
}
