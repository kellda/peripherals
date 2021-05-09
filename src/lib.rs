//! # peripherals
//!
//! Define and access your microcontroller peripherals
//!
//! ## Features
//!
//! - A macro to generate code for peripherals, registers, and register fields
//! - Zero-sized structs to represent peripherals and registers
//! - Enums or newtype struct to represent fields values
//! - Combine values to write to a regiser with `|` (binary or)
//! - Read/write access at the regiser level
//! - Strong typing ensure you don't mix up registers
//! - Generic over the peripheral instance
//!
//! ## Usage
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
//!
//! ## Operators
//!
//! Most types implements the `|` (bit or), `&` (bit and) and `^` (xor) operators. Operations that
//! returns that same type as their left operand can be written in the assign form, i.e. `|=`, `&=`
//! and `^=`. The operands can be of the type written in the table below, or anything that converts
//! to that type.
//!
//! Operation                          | Resulting type  | Description
//! ---------------------------------- | --------------- | ----------------------------------
//! [`Fields`]      \| [`Fields`]      | [`Fields`]      | Fields in either or both operands
//! [`Fields`]       & [`Fields`]      | [`Fields`]      | Fields in both operands
//! [`Fields`]       ^ [`Fields`]      | [`Fields`]      | Fields in only one of the operands
//! [`FieldValues`] \| [`FieldValues`] | [`FieldValues`] | Field values in either operand, with the value of the right operand if the field is specified in both
//! [`FieldValues`]  & [`Fields`]      | [`FieldValues`] | Field values of fields in both operands
//! [`FieldValues`]  ^ [`Fields`]      | [`FieldValues`] | Left operand with values of fields in both operands toggled (inverted)
//! [`Value`]       \| [`FieldValues`] | [`Value`]       | Left operand with the values of the fields in the right operand
//! [`Value`]        & [`Fields`]      | [`FieldValues`] | Values of fields in right operand
//! [`Value`]        ^ [`Fields`]      | [`Value`]       | Left operand with values of the fields in right operand toggled (inverted)
//!
//! ## Attribute in macros
//!
//! By default, attributes exand on generated types and fields, and not expand on generated impls.
//! This can be changed by inserting `type:`, `field:`, `impl:` or `all:` to specify on what the
//! attribute should be present. See the [`attributes`] module for examples.
//!
//! This allows to use e.g. `cfg` and `repr` attribute by writting `#[all: cfg(...)]` and `#[type: repr(...)]`.
// Idealy `derive`, `non_exhaustive`, `must_use` and `repr` would expand only on types, `doc` on
// types and fields, and any other attributes on everything. This however makes macros much more
// complex

#![no_std]
#![warn(missing_docs)]
#![warn(clippy::missing_inline_in_public_items)]
#![warn(clippy::missing_const_for_fn)]

#[doc(hidden)]
pub use paste::paste;
pub use utils::*;

mod utils;

mod macros;

#[cfg(any(doc, test))]
pub mod attributes;
#[cfg(any(doc, test))]
pub mod example;
