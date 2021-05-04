//! # peripherals
//!
//! Define and access your microcontroller peripherals
//!
//! # Features
//!
//! - A macro to generate code for peripherals, registers, and register fields
//! - Zero-sized structs to represent peripherals and registers
//! - Enums or newtype struct to represent fields values
//! - Combine values to write to a regiser with `|` (binary or)
//! - Read/write access at the regiser level
//! - Strong typing ensure you don't mix up registers
//! - Generic over the peripheral instance
//!
//! # Usage
//!
//! Peripherals are defined with the [`periph!`] macro. Registers and fields can be defined in the
//! same macro invocation or separately with the [`register!`] and [`field_type!`] macros.
//! Microcontrollers / devices are then defined with the [`device!`] macro.
//!
//! These macros generate types to represent fields and their values, and marker types for
//! registers and peripherals. These types can be seen in the [`example`] module.
//!
//! Registers are accessed with the [`Reg`] struct. [`Value`]s are used to read and write them.
//!
//! To use the generated device struct, create it from `()` as part of your initialisation routine.
//! There sould be only one instance of the device (and the right one) in your whole program.

#![no_std]
#![warn(missing_docs)]
#![warn(clippy::missing_inline_in_public_items)]
#![warn(clippy::missing_const_for_fn)]
#![feature(const_fn)]

#[doc(hidden)]
pub use paste::paste;
pub use utils::*;

mod utils;

mod macros {
    mod device;
    mod field_type;
    mod periph;
    mod register;
}

#[cfg(any(doc, test))]
pub mod example;
