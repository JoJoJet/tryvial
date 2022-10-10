//! A small crate for Ok-wrapping and try blocks.
//! This is compatible with [`Result`], [`Option`], and any type implementing the unstable [`std::ops::Try`] trait.
//!
//! *This crate does not require nightly Rust.*
//!
//! # Overview
//!
//! The titular macro, [`tryvial`], is used to perform Ok-wrapping on the return value of a function.
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
//! # use tryvial::tryvial;
//! #[tryvial]
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
//! The macro [`try_block`] is an implementation of "try blocks" from nightly rust.
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
//! The macro [`wrap_ok`] simply wraps an expression with the "ok" variant for a given [`Try`] type.
//!
//! ```
//! # use tryvial::wrap_ok;
//! assert_eq!(Some(42), wrap_ok!(42));
//! ```
//!
//! [`Try`]: std::ops::Try

pub use tryvial_proc::tryvial;

/// Performs "Ok-wrapping" on the result of an expression.
/// This is compatible with [`Result`], [`Option`], [`ControlFlow`], and any type that
/// implements the unstable [`std::ops::Try`] trait.
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
