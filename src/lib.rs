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
//! - See the example module for an example of usage and generated API
//!
//! # Usage
//!
//! - Define peripherals with the [`periph!`] macro
//! - Define microcontrollers / devices with the [`device!`] macro
//! - Take a look at the [`example`] module to see what the generated API looks like
//!
//! To use the generated device struct, create it from `()` as part of your initialisation routine.
//! There sould be only one instance of the device (and the right one) in your whole program.

#![no_std]
#![warn(missing_docs)]

/// A peripheral instance, that has a given address
pub unsafe trait Peripheral {
    /// The base address of this peripheral instance
    const BASE: usize;
}

/// Error returned when converting an interger to a field value fails
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct InvalidValue;

#[doc(hidden)]
pub use paste::paste;

#[cfg(any(doc, test))]
pub mod example;
#[cfg(test)]
mod test;
mod peripheral;
mod device;
