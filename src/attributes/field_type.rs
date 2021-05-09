//! An example of where attributes get expanded in [`field_type!`] macros
//!
//! Take a look at the source of this module to see the macro invocation used.

crate::field_type! {
    #[doc = "Nothing: only on enum"]
    enum Enum [u8] {
        #[doc = "Only on enum variant (`cfg` will likely don't work), prefixes not supported"]
        Variant = 0,
    }
}

crate::field_type! {
    #[type: doc = "`type`: only on enum"]
    enum EnumType [u8] {}
}

crate::field_type! {
    #[field: doc = "`field`: on nothing"]
    enum EnumField [u8] {}
}

crate::field_type! {
    #[impl: doc = "`impl`: only on impls  (obvoiusly not derived ones)"]
    enum EnumImpl [u8] {}
}

crate::field_type! {
    #[all: doc = "`all`: on enum and impls (obvoiusly not derived ones)"]
    enum EnumAll [u8] {}
}

crate::field_type! {
    #[doc = "Nothing: only on struct"]
    struct Struct [u8] (u8);
}

crate::field_type! {
    #[type: doc = "`type`: only on struct"]
    struct StructType [u8] (u8);
}

crate::field_type! {
    #[field: doc = "`field`: on nothing"]
    struct StructField [u8] (u8);
}

crate::field_type! {
    #[impl: doc = "`impl`: only on impls (obvoiusly not derived ones)"]
    struct StructImpl [u8] (u8);
}

crate::field_type! {
    #[all: doc = "`all`: on struct and impls (obvoiusly not derived ones)"]
    struct StructAll [u8] (u8);
}
