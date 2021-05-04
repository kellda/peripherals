//! A more complex example that shares fields and registers
//!
//! Take a look at the source of this module to see the macro invocation used.

use ::core::convert::{From, Into, TryFrom, TryInto};

/// Map fields from `BigRegister` to those of `SmallRegister`
pub struct Small<T>(pub T);

impl<T: Into<u8>> From<Small<T>> for u8 {
    fn from(value: Small<T>) -> u8 {
        value.0.into()
    }
}

impl<T> TryFrom<u8> for Small<T>
where
    u8: TryInto<T>,
{
    type Error = <u8 as TryInto<T>>::Error;

    fn try_from(value: u8) -> ::core::result::Result<Small<T>, Self::Error> {
        value.try_into().map(Small)
    }
}

crate::register! {
    /// Some 8-bit register
    SmallRegister: u8 = 0x00 {
        /// The mode field
        MODE: 0..1 = extern Small<super::field_type::Mode>;
        /// The status field
        STATUS: 2 = extern Small<super::field_type::Status>;
        /// The state field
        STATE: 3 = extern Small<super::field_type::State>;
        /// The data field
        DATA: 4..7 = extern Small<super::field_type::Data>;
    }
}

crate::register! {
    /// Some 16-bit register
    BigRegister: u16 = 0x00 {
        MODE: 0..1 = extern super::field_type::Mode;
        STATUS: 2 = extern super::field_type::Status;
        STATE: 3 = extern super::field_type::State;
        DATA: 8..15 = extern super::field_type::Data;
    }
}

crate::periph! {
    /// The peripheral
    ComplexPeripheral;
    /// A `SmallRegister`
    rw SMALL1 @ 0x00: u8 = SmallRegister;
    /// An other `SmallRegister`
    rw SMALL2 @ 0x01: u8 = SmallRegister;
    /// A `BigRegister`
    rw BIG1 @ 0x02: u16 = BigRegister;
    /// An other `BigRegister`
    rw BIG2 @ 0x04: u16 = BigRegister;
}
