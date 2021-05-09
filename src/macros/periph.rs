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
        $(#[$($periph_attr:tt)*])*
        $periph:ident;
        $($(#[$($reg_attr:tt)*])* $rw:ident $reg:ident @ $offset:literal : $int:ty = $desc1:tt $desc2:tt)*
    ) => {
        $crate::periph_attr_inner! { @type { $([$($periph_attr)*])* } {} {
            periph_inner: @struct $periph {$( $(#[$($reg_attr)*])* $reg )*} {}
        }}

        $crate::paste! { $crate::periph_attr_inner! { @impl { $([$($periph_attr)*])* } {} {
        periph_attr_inner: @expand
            impl<P: $crate::Peripheral> $periph<P> {
                /// Erase peripheral information
                ///
                /// This allows to choose at runtime which instance of a peripheral to use.
                #[inline]
                pub fn into_dyn(self) -> &'static mut [<Dyn $periph>] {
                    unsafe { &mut *(P::BASE as *mut _) }
                }
            }
        }}}

        $($crate::periph_inner!( $(#[$($reg_attr)*])* $rw $reg @ $offset : $int = $desc1 $desc2); )*

        $crate::periph_attr_inner! { @type { $([$($periph_attr)*])* } {} {
            periph_inner: @struct $periph dyn {$( $(#[$($reg_attr)*])* $reg )*} {}
        }}
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! periph_inner {
    ($(#[$($attr:tt)*])* $rw:ident $reg:ident @ $offset:literal : $int:ty = $desc:ty ; ) => {
        $crate::periph_attr_inner! { @type { $([$($attr)*])* } {} {
        periph_attr_inner: @expand
            #[derive(Debug)]
            pub enum $reg {}
        }}

        $crate::periph_attr_inner! { @impl { $([$($attr)*])* } {} {
        periph_attr_inner: @expand
            impl $crate::Register for $reg {
                type Int = $int;
                type Value = $desc;

                const OFFSET: usize = $offset;
                const NAME: &'static str = stringify!($reg);
            }
        }}

        $crate::periph_attr_inner! { @impl { $([$($attr)*])* } {} { periph_inner: @impl $rw $reg }}
    };
    ($(#[$($attr:tt)*])* $rw:ident $reg:ident @ $offset:literal : $int:ty = $reset:literal $desc:tt) => {
        $crate::register!($(#[$($attr)*])* $reg: $int = $reset $desc);

        $crate::periph_attr_inner! { @impl { $([$($attr)*])* } {} {
        periph_attr_inner: @expand
            impl $crate::Register for $reg {
                type Int = $int;
                type Value = $reg;

                const OFFSET: usize = $offset;
                const NAME: &'static str = stringify!($reg);
            }
        }}

        $crate::periph_attr_inner! { @impl { $([$($attr)*])* } {} { periph_inner: @impl $rw $reg }}
    };

    (@impl $(#[$attr:meta])* rw $reg:ident) => {
        impl $crate::ReadRegister for $reg {}
        impl $crate::WriteRegister for $reg {}
    };
    (@impl $(#[$attr:meta])* r $reg:ident) => {
        impl $crate::ReadRegister for $reg {}
    };
    (@impl $(#[$attr:meta])* w $reg:ident) => {
        impl $crate::WriteRegister for $reg {}
    };

    (@struct $periph:ident {} {$(#[$periph_attr:meta])*
        $($reg:ident $(#[$attr:meta])*)*
    }) => { $crate::paste! {
        $(#[$periph_attr])*
        #[derive(Debug)]
        pub struct $periph<P: $crate::Peripheral> {$(
            $(#[$attr])*
            pub [<$reg:lower>]: $crate::Reg<$reg, P>,
        )*}
    }};
    (@struct $periph:ident dyn {} {$(#[$periph_attr:meta])*
        $($reg:ident $(#[$attr:meta])*)*
    }) => { $crate::paste! {
        $(#[$periph_attr])*
        #[derive(Debug)]
        pub struct [<Dyn $periph>] {$(
            $(#[$attr])*
            pub [<$reg:lower>]: $crate::DynReg<$reg>,
        )*}
    }};
    (@struct $(#[$attr:meta])* $periph:ident $($type:ident)? {} {$($rest:tt)*} ) => {
        $crate::periph_inner!(@struct $periph $($type)? {} { $($rest)* $(#[$attr])* } );
    };
    (@struct
        $(#[$prev:meta])* $periph:ident $($type:ident)?
        { $(#[$($attr:tt)*])* $reg:ident $($rest:tt)* }
        { $($parsed:tt)* }
    ) => {
        $crate::periph_attr_inner! { @field { $([$($attr)*])* } {} {
            periph_inner: @struct $periph $($type)? { $($rest)* } { $($parsed)* $(#[$prev])* $reg }
        }}
    };
}
