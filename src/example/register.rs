//! Example of generated register API
//!
//! Take a look at the source of this module to see the macro invocation used.

crate::register! {
    /// Some register
    RegisterName: u16 = 0x1234 {
        /// A field which can be read as a u8
        EXTERN: 0..2 = extern Type;
        /// A field that is newtype over a bool
        NEWTYPE: 3 = struct Newtype(bool);
        /// A field that is an enum
        ENUM: 4..5 = enum Enum {
            False = 0,
            True = 1,
        }
    }
}

crate::field_type! {
    /// A type declared with `field_type!`
    struct Type [u16] (u8);
}
