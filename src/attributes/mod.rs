//! Example of attribute in macros
//!
//! By default, attributes exand on generated types and fields, and not expand on generated impls.
//! This shows how it can be changed.
//!
//! The macro invocations can be seen in the source code. This module exists only in documentation
//! and tests

// To check that everithing is explicit in the macro
#![no_implicit_prelude]
#![allow(missing_docs, non_upper_case_globals)]

pub mod device;
pub mod field_type;
pub mod periph;
pub mod register;
