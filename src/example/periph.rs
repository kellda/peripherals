//! Example of generated peripheral API
//!
//! Take a look at the source of this module to see the macro invocation used.

crate::periph! {
    /// The peripheral
    Peripheral;
    /// A read-write register
    rw CONFIG @ 0x00: u16 = 0x0000 {
        /// Some mode for the peripheral
        MODE: 0..1 = enum Mode {
            A = 0,
            B = 1,
            C = 2,
            D = 3,
        }
        /// Some flag
        FLAG: 2 = enum CfgFlag {
            False = 0,
            True = 1,
        }
    }
    /// A read-only register
    r STATUS @ 0x02: u16 = 0x0000 {
        /// Some status for the peripheral
        STAT: 0..1 = enum Status {
            A = 0,
            B = 1,
            C = 2,
            D = 3,
        }
        /// Some flag
        FLAG: 2 = struct StatFlag(bool);
    }
    /// A write-only register
    w BUFFER @ 0x04: u16 = 0x0000 {
        /// Data to use with the peripheral
        DATA: 0..7 = struct Data(u8);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{Peripheral, *};
    use ::core::assert_eq;

    enum PERIPH {}
    impl Peripheral for PERIPH {
        const BASE: usize = 0;
        const NAME: &'static str = "PERIPH";
    }

    #[test]
    fn registers() {
        use ::core::mem::size_of;
        assert_eq!(size_of::<Reg<CONFIG, PERIPH>>(), 0);
        assert_eq!(size_of::<Reg<STATUS, PERIPH>>(), 0);
        assert_eq!(size_of::<Reg<BUFFER, PERIPH>>(), 0);
        assert_eq!(CONFIG::OFFSET, 0);
        assert_eq!(STATUS::OFFSET, 2);
        assert_eq!(BUFFER::OFFSET, 4);
    }

    #[test]
    fn fields() {
        assert_eq!(CONFIG::MODE.mask(), 0x03);
        assert_eq!(CONFIG::MODE.offset(), 0);
        assert_eq!(CONFIG::FLAG.mask(), 0x04);
        assert_eq!(CONFIG::FLAG.offset(), 2);
        assert_eq!(STATUS::STAT.mask(), 0x03);
        assert_eq!(STATUS::STAT.offset(), 0);
        assert_eq!(STATUS::FLAG.mask(), 0x04);
        assert_eq!(STATUS::FLAG.offset(), 2);
        assert_eq!(BUFFER::DATA.mask(), 0xff);
        assert_eq!(BUFFER::DATA.offset(), 0);
    }
}
