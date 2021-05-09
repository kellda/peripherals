//! Types and trait needed for this library

use core::convert::TryInto;
use core::fmt::{self, Debug};
use core::marker::PhantomData;
use core::ops::*;

pub use dynreg::*;
pub use field::*;
pub use field_values::*;
pub use fields::*;
pub use reg::*;
pub use value::*;

mod dynreg;
mod field;
mod field_values;
mod fields;
mod reg;
mod value;

use private::*;
mod private {
    use super::*;

    pub trait Int:
        BitAnd<Output = Self>
        + BitOr<Output = Self>
        + BitXor<Output = Self>
        + Not<Output = Self>
        + Shl<usize, Output = Self>
        + Shr<usize, Output = Self>
        + Eq
        + Default
        + fmt::Binary
        + fmt::LowerHex
        + Copy
    {
        const WIDTH: usize;
    }

    impl Int for u8 {
        const WIDTH: usize = 8;
    }
    impl Int for u16 {
        const WIDTH: usize = 16;
    }
    impl Int for u32 {
        const WIDTH: usize = 32;
    }
    impl Int for u64 {
        const WIDTH: usize = 64;
    }
    impl Int for u128 {
        const WIDTH: usize = 128;
    }

    pub trait Both<T> {
        type Output;
    }

    impl<T: MayToggle> Both<()> for T {
        type Output = ();
    }

    impl<T: MayToggle> Both<Toggle> for T {
        type Output = T::Toggle;
    }

    pub trait Either<T> {
        type Output;
    }

    impl<T: MayToggle> Either<()> for T {
        type Output = T::Toggle;
    }

    impl<T: MayToggle> Either<Toggle> for T {
        type Output = Toggle;
    }
}

/// A trait for peripheral instances
///
/// This trait is implemented by the [`device!`] macro for marker types that indicate peripheral instances.
pub trait Peripheral {
    /// The base address of this peripheral instance
    const BASE: usize;
    /// The name to be displayed in debug
    const NAME: &'static str;
}

/// A trait for the register associated with a value
///
/// This trait is implemented by the [`register!`] macro for marker types that indicate registers associated to a value.
pub trait RegisterValue {
    /// The width of this register (`u8`, `u16`, etc.)
    type Int: Int;
    /// The reset value of this register
    const RESET: Self::Int;
    /// The name to be displayed in debug
    const NAME: &'static str;
}

/// A trait for registers
//
/// This trait is implemented by the [`periph!`] macro for marker types that indicate registers.
pub trait Register {
    /// The width of this register (`u8`, `u16`, etc.)
    type Int: Int;
    /// The marker type for values read from this register
    type Value: RegisterValue<Int = Self::Int>;

    /// The offset from the base address
    const OFFSET: usize;
    /// The name to be displayed in debug
    const NAME: &'static str;
}

/// A marker trait for readable registers
pub trait ReadRegister: Register {}

/// A marker trait for writeable register
pub trait WriteRegister: Register {}

/// A marker type for toggleable fields
#[derive(Debug)]
pub enum Toggle {}

/// Whether the fields or fields values may be toggled
pub trait MayToggle {
    /// `Toggle` if it can be toggled, `()` otherwise
    type Toggle;
}

/// Error returned when converting an interger to a field value fails
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct InvalidValue;
