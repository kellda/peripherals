//! Example of generated peripheral API
//!
//! Take a look at the source of this module to see the macro invocation used.

crate::periph! {
    /// The peripheral
    Peripheral;
    "A read-write register"
    /// Detailed description of `CONFIG`
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
    "A read-only register"
    /// Detailed description of `STATUS`
    r STATUS @ 0x02: u16 = 0x0000 {
        /// Some status for the peripheral
        STATUS: 0..1 = enum Status {
            A = 0,
            B = 1,
            C = 2,
            D = 3,
        }
        /// Some flag
        FLAG: 2 = struct StatFlag(bool);
    }
    "A write-only register"
    /// Detailed description of `BUFFER`
    w BUFFER @ 0x04: u16 = 0x0000 {
        /// Data to use with the peripheral
        DATA: 0..7 = struct Data(u8);
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	use ::core::assert_eq;
	
	enum PERIPH {}
	unsafe impl crate::Peripheral for PERIPH {
		const BASE: usize = 0;
	}
	
	#[test]
	fn registers() {
		use ::core::mem::size_of;
		assert_eq!(size_of::<CONFIG<PERIPH>>(), 0);
		assert_eq!(size_of::<STATUS<PERIPH>>(), 0);
		assert_eq!(size_of::<BUFFER<PERIPH>>(), 0);
		assert_eq!(CONFIG::<PERIPH>::OFFSET, 0);
		assert_eq!(STATUS::<PERIPH>::OFFSET, 2);
		assert_eq!(BUFFER::<PERIPH>::OFFSET, 4);
	}
	
	#[test]
	fn fields() {
		assert_eq!(Mode::OFFSET, 0);
		assert_eq!(Mode::MASK, 0x03);
		assert_eq!(CfgFlag::OFFSET, 2);
		assert_eq!(CfgFlag::MASK, 0x04);
		assert_eq!(Status::OFFSET, 0);
		assert_eq!(Status::MASK, 0x03);
		assert_eq!(StatFlag::OFFSET, 2);
		assert_eq!(StatFlag::MASK, 0x04);
		assert_eq!(Data::OFFSET, 0);
		assert_eq!(Data::MASK, 0xff);
	}
}