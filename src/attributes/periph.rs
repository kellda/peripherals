//! An example of where attributes get expanded in [`periph!`] macros
//!
//! Take a look at the source of this module to see the macro invocation used.

crate::periph! {
    #[doc = "Nothing: only on peripheral struct"]
    Peripheral;
    #[doc = "Nothing: on `Register` and field of peripheral"]
    rw Register @ 0x00: u8 = 0x00 {
        #[doc = "Nothing: on `Newtype` and associated constant"]
        Newtype: 3 = struct Newtype(bool);
        #[doc = "Nothing: on `Enum` and associated constant"]
        Enum: 4..5 = enum Enum {}
    }
}

crate::periph! {
    #[type: doc = "`type`: only on peripheral struct"]
    PeripheralType;
    #[type: doc = "`type`: only on register structs"]
    rw RegisterType @ 0x00: u8 = 0x00 {
        #[type: doc = "`type`: only on `NewtypeType`"]
        NewtypeType: 3 = struct NewtypeType(bool);
        #[type: doc = "`type`: only on `EnumType`"]
        EnumType: 4..5 = enum EnumType {}
    }
}

crate::periph! {
    #[field: doc = "`field`: on nothing"]
    PeripheralField;
    #[field: doc = "`field`: on peripheral field"]
    rw RegisterField @ 0x00: u8 = 0x00 {
        #[field: doc = "`field`: on associated constant"]
        NewtypeField: 3 = struct NewtypeField(bool);
        #[field: doc = "`field`: on associated constant"]
        EnumField: 4..5 = enum EnumField {}
    }
}

crate::periph! {
    #[impl: doc = "`impl`: only on impls"]
    PeripheralImpl;
    #[impl: doc = "`impl`: only on impls"]
    rw RegisterImpl @ 0x00: u8 = 0x00 {
        #[impl: doc = "`impl`: only on impls"]
        NewtypeImpl: 3 = struct NewtypeImpl(bool);
        #[impl: doc = "`impl`: only on impls"]
        EnumImpl: 4..5 = enum EnumImpl {}
    }
}

crate::periph! {
    #[all: doc = "`all`: on everything"]
    PeripheralAll;
    #[all: doc = "`all`: on everything"]
    rw RegisterAll @ 0x00: u8 = 0x00 {
        #[all: doc = "`all`: on everything"]
        NewtypeAll: 3 = struct NewtypeAll(bool);
        #[all: doc = "`all`: on everything"]
        EnumAll: 4..5 = enum EnumAll {}
    }
}
