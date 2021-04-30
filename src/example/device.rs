//! Example of generated device API
//!
//! Take a look at the source of this module to see the macro invocation used.

crate::device! {
    /// The microcontroller
    Microcontroller;
    /// Some peripheral
    PERIPH1 @ 0x0010: super::peripheral::Peripheral;
    /// An other instance of the same peripheral
    PERIPH2 @ 0x0020: super::peripheral::Peripheral;
}
