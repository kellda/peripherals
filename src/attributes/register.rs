//! An example of where attributes get expanded in [`register!`] macros
//!
//! Take a look at the source of this module to see the macro invocation used.

crate::register! {
    #[doc = "Nothing: only on register structs"]
    Register: u8 = 0x00 {
        #[doc = "Nothing: on associated constant (not on `Extern` because it is `extern`)"]
        Extern: 0..2 = extern Extern;
        #[doc = "Nothing: on `Newtype` and associated constant"]
        Newtype: 3 = struct Newtype(bool);
        #[doc = "Nothing: on `Enum` and associated constant"]
        Enum: 4..5 = enum Enum {}
    }
}

crate::register! {
    #[type: doc = "`type`: only on register structs"]
    RegisterType: u8 = 0x00 {
        #[type: doc = "`type`: on nothing (not on `ExternType` because it is `extern`)"]
        ExternType: 0..2 = extern ExternType;
        #[type: doc = "`type`: only on `NewtypeType`"]
        NewtypeType: 3 = struct NewtypeType(bool);
        #[type: doc = "`type`: only on `EnumType`"]
        EnumType: 4..5 = enum EnumType {}
    }
}

crate::register! {
    #[field: doc = "`field`: on nothing"]
    RegisterField: u8 = 0x00 {
        #[field: doc = "`field`: on associated constant"]
        ExternField: 0..2 = extern ExternField;
        #[field: doc = "`field`: on associated constant"]
        NewtypeField: 3 = struct NewtypeField(bool);
        #[field: doc = "`field`: on associated constant"]
        EnumField: 4..5 = enum EnumField {}
    }
}

crate::register! {
    #[impl: doc = "`impl`: only on impls"]
    RegisterImpl: u8 = 0x00 {
        #[impl: doc = "`impl`: only on impls"]
        ExternImpl: 0..2 = extern ExternImpl;
        #[impl: doc = "`impl`: only on impls"]
        NewtypeImpl: 3 = struct NewtypeImpl(bool);
        #[impl: doc = "`impl`: only on impls"]
        EnumImpl: 4..5 = enum EnumImpl {}
    }
}

crate::register! {
    #[all: doc = "`all`: on everything"]
    RegisterAll: u8 = 0x00 {
        #[all: doc = "`all`: on everything"]
        ExternAll: 0..2 = extern ExternAll;
        #[all: doc = "`all`: on everything"]
        NewtypeAll: 3 = struct NewtypeAll(bool);
        #[all: doc = "`all`: on everything"]
        EnumAll: 4..5 = enum EnumAll {}
    }
}

crate::field_type! {
    /// A type defined with `field_type!`
    struct Extern [u8] (u8);
}

crate::field_type! {
    /// A type defined with `field_type!`
    struct ExternType [u8] (u8);
}

crate::field_type! {
    /// A type defined with `field_type!`
    struct ExternField [u8] (u8);
}

crate::field_type! {
    /// A type defined with `field_type!`
    struct ExternImpl [u8] (u8);
}

crate::field_type! {
    /// A type defined with `field_type!`
    struct ExternAll [u8] (u8);
}
