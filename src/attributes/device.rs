//! An example of where attributes get expanded in [`device!`] macros
//!
//! Take a look at the source of this module to see the macro invocation used.

crate::device! {
    #[doc = "Nothing: only on device struct"]
    Microcontroller;
    #[doc = "Nothing: on `Periph` and `periph` field"]
    Periph @ 0x0000: super::periph::Peripheral;
}

crate::device! {
    #[type: doc = "`type`: only on device struct"]
    MicrocontrollerType;
    #[type: doc = "`type`: on `PeriphType`"]
    PeriphType @ 0x0000: super::periph::PeripheralType;
}

crate::device! {
    #[field: doc = "`field`: on nothing"]
    MicrocontrollerField;
    #[field: doc = "`field`: on `periphfield` field"]
    PeriphField @ 0x0000: super::periph::PeripheralField;
}

crate::device! {
    #[impl: doc = "`impl`: only on impls"]
    MicrocontrollerImpl;
    #[impl: doc = "`impl`: only on impls"]
    PeriphImpl @ 0x0000: super::periph::PeripheralImpl;
}

crate::device! {
    #[all: doc = "`all`: on everything"]
    MicrocontrollerAll;
    #[all: doc = "`all`: on everything"]
    PeriphAll @ 0x0000: super::periph::PeripheralAll;
}
