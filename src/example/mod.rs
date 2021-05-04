//! Example of generated types
//!
//! The macro invocations can be seen in the source code. This module exists only in documentation
//! and tests
//!
//! To actually use the generated device struct, it must be cased from `()`. This should idealy be
//! done in the runtime support crate (`-rt` crate), or at the very beginning of your main function.
//! There sould be only one instance of the device (and the right one) in your whole program.
//!
//! ```
//! // In some library
//! peripherals::device! {
//!     YourDevice;
//!     // peripherals elided
//! }
//!
//! // In a `-rt` crate or at the very beginning of your program:
//! let peripherals: YourDevice = unsafe { core::mem::transmute(()) };
//! ```

// To check that everithing is explicit in the macro
#![no_implicit_prelude]
#![allow(missing_docs)]

pub mod complex;
pub mod device;
pub mod field_type;
pub mod periph;
pub mod register;
