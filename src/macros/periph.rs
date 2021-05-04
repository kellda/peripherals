/// Define a peripheral and all its associated registers, fields and values
///
/// It is recommended to have one module per peripheral and thus to invoke this macros in its own
/// module. For an example of the generated types, see the [example module](crate::example::periph).
///
/// # Usage
///
/// The macro begins with the peripheral name. Note that you can't actually define an empty peripheral.
///
/// ```
/// peripherals::periph!{
///     /// Optional documentation
///     MyPeripheral;
/// #   rw MY_REG @ 0: u16 = 0 {}
/// }
/// ```
///
/// Then each register is described as follow:
///
/// ```
/// peripherals::periph!{
///     MyPeripheral;
///     // access  name    offset size  reset value
///          rw   MY_REG @  0x00: u16 =   0x1234    {
///
///         // fields go here
///     }
/// }
/// ```
///
/// - The access can be `r`, `w` or `rw` for read-only, write-only or read-write.
/// - The name of the register is generaly written in uppercase. It is used to name all types
///   related to this register, as well as the field (in lowercase) in the peripheral struct.
/// - The offset (here `0x00`) is the offset of the register in the register block / peripheral.
///   This allows the code to be generic over the peripheral instance.
/// - The size describes the width of the register (8, 16, 32, or more bits) as well as the type
///   used for all accesses.
/// - The reset value (here `0x1234`) is the "default" value of the register, i.e. the one after a
///   reset of the microcontroller.
///
/// The field description is the same a for the [`register!`] macro, but leading `+` are not needed.
///
/// ```
/// # peripherals::periph!{
/// #    MyPeripheral;
/// #    rw MY_REG @ 0x00: u16 = 0x1234 {
/// A_SRTUCT: 0..4 = struct Struct(u8);
/// BOOL_STRUCT: 5 = struct Bool(bool);
/// AN_ENUM: 6..7 = enum Enum {
///      A = 0,
///      B = 1,
///      C = 2,
///      D = 3,
/// }
/// SINGLE_BIT_ENUM: 8 = enum SingleBit {
///      Off = 0,
///      On = 1,
/// }
/// #    }
/// # }
/// ```

#[macro_export]
macro_rules! periph {
    (
        $(#[$periph_attr:meta])*
        $periph:ident;
        $($(#[$reg_attr:meta])* $rw:ident $reg:ident @ $offset:literal : $int:ty = $desc1:tt $desc2:tt)*
    ) => { $crate::paste! {
        $(#[$periph_attr])*
        #[derive(Debug)]
        pub struct $periph<P> {$(
            $(#[$reg_attr])*
            pub [<$reg:lower>]: $crate::Reg<$reg, P>,
        )*}

        impl<P: $crate::Peripheral> $periph<P> {
            /// Erase peripheral information
            ///
            /// This allows to choose at runtime which instance of a peripheral to use.
            #[inline]
            pub fn into_dyn(self) -> &'static mut [<Dyn $periph>] {
                unsafe { &mut *(P::BASE as *mut _) }
            }
        }

        $($crate::periph_inner!( $(#[$reg_attr])* $rw $reg @ $offset : $int = $desc1 $desc2); )*

        $(#[$periph_attr])*
        #[derive(Debug)]
        pub struct [<Dyn $periph>] {$(
            $(#[$reg_attr])*
            pub [<$reg:lower>]: $crate::DynReg<$reg>,
        )*}
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! periph_inner {
    ($(#[$attr:meta])* $rw:ident $reg:ident @ $offset:literal : $int:ty = $desc:ty ;) => {
        $(#[$attr])*
        #[derive(Debug)]
        pub enum $reg {}

        impl $crate::Register for $reg {
            type Int = $int;
            type Value = $desc;

            const OFFSET: usize = $offset;
        }

        $crate::periph_inner!(@impl $rw $reg);
    };
    ($(#[$attr:meta])* $rw:ident $reg:ident @ $offset:literal : $int:ty = $reset:literal $desc:tt) => {
        $crate::register!($(#[$attr])* $reg: $int = $reset $desc);

        impl $crate::Register for $reg {
            type Int = $int;
            type Value = $reg;

            const OFFSET: usize = $offset;
        }

        $crate::periph_inner!(@impl $rw $reg);
    };

    (@impl rw $reg:ident) => {
        impl $crate::ReadRegister for $reg {}
        impl $crate::WriteRegister for $reg {}
    };
    (@impl r $reg:ident) => {
        impl $crate::ReadRegister for $reg {}
    };
    (@impl w $reg:ident) => {
        impl $crate::WriteRegister for $reg {}
    };
}
