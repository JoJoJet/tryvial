//! # Examples
//!
//! Before:
//! ```
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let x = some_fallible_operation()?;
//!     println!("Hello, {x}!");
//!     Ok(()) // this is ugly
//! }
//! # fn some_fallible_operation() -> Result<&'static str, &'static str> { Ok("World") }
//! ```
//!
//! After:
//! ```
//! # use tryvial::tryvial;
//! #[tryvial]
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let x = some_fallible_operation()?;
//!     println!("Hello, {x}!");
//! }
//! # fn some_fallible_operation() -> Result<&'static str, &'static str> { Ok("World") }
//! ```

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
/// # assert_eq!(y, Ok(3));
/// ```
#[macro_export]
macro_rules! try_block {
    { $($token:tt)* } => {
        (|| $crate::wrap_ok!({
            $($token)*
        }))()
    }
}
