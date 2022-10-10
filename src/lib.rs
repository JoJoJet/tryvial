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

