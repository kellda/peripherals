//! Example of generated field type API
//!
//! Take a look at the source of this module to see the macro invocation used.

crate::field_type! {
    /// An enum
    enum Mode [u8, u16] {
        A = 0,
        B = 1,
        C = 2,
        D = 3,
    }
}

crate::field_type! {
    /// A newtype struct
    struct Data [u8, u16] (u8);
}

crate::field_type! {
    /// An enum with two variants. It implements `Not`
    enum State [u8, u16] {
        Low = 0,
        High = 1,
    }
}

crate::field_type! {
    /// A newtype struct over a bool. It implements `Not`
    struct Status [u8, u16] (bool);
}
