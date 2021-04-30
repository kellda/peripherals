/// Define a peripheral and all its associated registers, fields and values
///
/// It is recommended to have one module per peripheral and thus to invoke this macros in its own
/// module. For an example of the generated api, see the [example module](crate::example::peripheral).
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
///     // access name     offset size  reset value
///        rw     MY_REG @ 0x00:  u16 = 0x1234      {
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
/// There can also be some documentation: a short description of the resister, used on the register
/// struct and the register field of the peripheral struct, and a long description, used on
/// everything related to this register
///
/// ```
/// peripherals::periph!{
///     MyPeripheral;
///     "Short description"
///     /// Detailed description of my register
///     rw MY_REG @ 0x00: u16 = 0x1234 {
///         // fields go here
///     }
/// }
/// ```
///
/// Each field has a name, used (in lowercase) for the access function on read values. It also has
/// a position in the register, and a type. The position is either an inclusive range or a single bit.
///
/// ```
/// # peripherals::periph!{
/// #    MyPeripheral;
/// #    "Short description"
/// #    /// Detailed description of my register
/// #    rw MY_REG @ 0x00: u16 = 0x1234 {
/// // name     position
/// SOME_FIELD: 0..1     = // type
/// # enum A {}
/// SINGLE_BIT: 2        = // type
/// # enum B {}
/// #    }
/// # }
/// ```
///
/// The type can be:
/// - A unit struct over an other type, which must implement `Into` and `TryFrom` for the register
///   type (e.g. `u16`)
/// - An enum over all possible values, which is expected to be exhaustive
/// - A type defined somewere else for more flexibility. This allows to give two names to a field,
///   to share types between registers and to define and use more complex types.
///
/// ```
/// # peripherals::periph!{
/// #    MyPeripheral;
/// #    "Short description"
/// #    /// Detailed description of my register
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
/// ALIAS: 8 = extern SingleBit;
/// #    }
/// # }
/// ```
///
/// Everything can of course also have doc comments

#[macro_export]
macro_rules! periph {
    (
        $(#[$periph_attr:meta])*
        $periph:ident;
        $($($reg_desc:literal)? $(#[$reg_attr:meta])*
        $rw:ident $reg:ident @ $offset:literal : $type:ty = $reset:literal $fields:tt)*
    ) => { $crate::paste! {
        $(#[$periph_attr])*
        #[derive(Debug)]
        pub struct $periph<P> {$(
            $(#[doc = $reg_desc] #[doc = ""])?
            $(#[$reg_attr])*
            pub [<$reg:lower>]: $reg<P>,
        )*}

        $(
        $(#[doc = $reg_desc] #[doc = ""])?
        $(#[$reg_attr])*
        #[derive(Debug)]
        #[allow(non_camel_case_types)]
        pub struct $reg<P>(::core::marker::PhantomData<P>);

        impl<P: $crate::Peripheral> $reg<P> {
            #[doc = "Offset of `" $reg "` in the register block"]
            pub const OFFSET: usize = $offset;

            $crate::periph!(@impl_reg base $reg $type);
            $crate::periph!(@impl_reg $rw [<$reg Value>] [<$reg Bits>]);
        }

        $crate::periph!(@impl_value $(#[$reg_attr])*
            $reg [<$reg Value>] [<$reg Bits>]: $type = $reset
            $fields
        );
        $crate::periph!(@impl_bits $(#[$reg_attr])* $reg [<$reg Bits>]: $type);

        $crate::periph!(@fields [<$reg Bits>]: $type $fields);
        )*
    }};

    (@impl_reg base $reg:ident $type:ty) => {
        /// Raw pointer to the register
        #[inline]
        pub fn ptr(&self) -> *const $type {
            (P::BASE + Self::OFFSET) as *const $type
        }

        /// Mutable raw pointer to the register
        #[inline]
        pub fn ptr_mut(&mut self) -> *mut $type {
            (P::BASE + Self::OFFSET) as *mut $type
        }

        // /// Erase peripheral information
        // #[inline]
        // pub fn into_dyn(self) -> &'static mut DynReg<R> {
        //     unsafe { &mut *(P::BASE as *mut _) }
        // }
    };
    (@impl_reg rw $value:ident $bits:ident) => {
        $crate::periph!(@impl_reg r $value $bits);
        $crate::periph!(@impl_reg w $value $bits);

        /// Modify the given fields
        #[inline]
        pub fn modify<B: ::core::convert::Into<$bits>>(&mut self, bits: B) {
            self.write(self.read() | bits.into());
        }
    };
    (@impl_reg r $value:ident $bits:ident) => {
        /// Read the current value of this register
        #[inline]
        pub fn read(&self) -> $value {
            unsafe { $value(self.ptr().read_volatile()) }
        }

        /// Test the given fields
        #[inline]
        pub fn test<B: ::core::convert::Into<$bits>>(&self, bits: B) -> bool {
            self.read().test(bits)
        }
    };
    (@impl_reg w $value:ident $bits:ident) => {
        /// Write a value to this register
        #[inline]
        pub fn write(&mut self, value: $value) {
            unsafe { self.ptr_mut().write_volatile(value.0) };
        }

        /// Reset this register
        #[inline]
        pub fn reset(&mut self) {
            self.write($value::RESET);
        }
    };

    (@impl_value $(#[$attr:meta])* $reg:ident $value:ident $bits:ident: $type:ty = $reset:literal {$(
        $(#[$field_attr:meta])*
        $field:ident : $start:literal $(.. $end:literal)? = $kind:ident $field_type:ident $({ $($enum:tt)* })? $(($($struct:tt)*))? $(;)?
    )*}) => { $crate::paste! {
        #[doc = "A value read from or to write to [`" $reg "`]."]
        #[doc = ""]
        $(#[$attr])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[allow(non_camel_case_types)]
        pub struct $value($type);

        impl $value {
            #[doc = "Reset value of `" $reg "`."]
            pub const RESET: $value = $value($reset);

            /// Get the raw value
            #[inline]
            pub fn raw_value(self) -> $type {
                self.0
            }

            /// Build from a raw value
            #[inline]
            pub unsafe fn from_raw(value: $type) -> $value {
                $value(value)
            }

            /// Test the given fields
            #[inline]
            pub fn test<B: ::core::convert::Into<[<$reg Bits>]>>(self, bits: B) -> bool {
                let bits = bits.into();
                self.0 & bits.mask == bits.bits
            }

            $($(#[$field_attr])*
            #[inline]
            pub fn [<$field:lower>](self) -> $field_type {
                let value = (self.0 & $field_type::MASK) >> $field_type::OFFSET;
                ::core::convert::TryInto::<$field_type>::try_into(value).unwrap()
            })*
        }}

        impl ::core::default::Default for $value {
            /// Get the default / reset value
            #[inline]
            fn default() -> $value {
                $value::RESET
            }
        }

        impl<T: ::core::convert::Into<$bits>> ::core::ops::BitOr<T> for $value {
            type Output = $value;

            fn bitor(self, other: T) -> $value {
                let other = other.into();
                $value(self.0 & other.mask | other.bits)
            }
        }

        impl<T: ::core::convert::Into<$bits>> ::core::ops::BitOrAssign<T> for $value {
            fn bitor_assign(&mut self, other: T) {
                let other = other.into();
                self.0 = self.0 & other.mask | other.bits;
            }
        }
    };

    (@impl_bits $(#[$attr:meta])* $reg:ident $bits:ident: $type:ty) => {
        $crate::paste! {
            #[doc = "A bit pattern to modify or to test the value of [`" $reg "`]."]
            #[doc = ""]
            $(#[$attr])*
            #[derive(Clone, Copy, Debug, PartialEq, Eq)]
            #[allow(non_camel_case_types)]
            pub struct $bits {
                bits: $type,
                mask: $type,
            }
        }

        impl $bits {
            /// Get the bits to set
            #[inline]
            pub fn bits(self) -> $type {
                self.bits
            }

            /// Get the mask
            #[inline]
            pub fn mask(self) -> $type {
                self.mask
            }

            /// Build from raw bits and mask
            #[inline]
            pub unsafe fn from_raw(bits: $type, mask: $type) -> $bits {
                $bits { bits, mask }
            }
        }

        impl ::core::default::Default for $bits {
            /// A bit pattern that doesn't change anything
            #[inline]
            fn default() -> $bits {
                $bits {
                    bits: 0,
                    mask: !0,
                }
            }
        }

        impl<T: ::core::convert::Into<$bits>> ::core::ops::BitOr<T> for $bits {
            type Output = $bits;

            fn bitor(self, other: T) -> $bits {
                let other = other.into();
                $bits {
                    bits: self.bits & other.mask | other.bits,
                    mask: self.mask & other.mask,
                }
            }
        }

        impl<T: ::core::convert::Into<$bits>> ::core::ops::BitOrAssign<T> for $bits {
            fn bitor_assign(&mut self, other: T) {
                let other = other.into();
                self.bits = self.bits & other.mask | other.bits;
                self.mask = self.mask & other.mask;
            }
        }
    };

    (@fields $bits:ident: $type:ty {$(
        $(#[$field_attr:meta])*
        $field:ident : $start:literal $(.. $end:literal)? = $kind:ident $field_type:ident $({ $($enum:tt)* })? $(($($struct:tt)*))? $(;)?
    )*}) => {
        $($crate::periph!(
            @field_type $bits $type: $(#[$field_attr])*
            $start $(.. $end)? = $kind $field_type $({ $($enum)* })? $(($($struct)*))?
        );)*
    };
    (@field_type $bits:ident $type:ty :
        $(#[$attr:meta])*
        $start:literal = enum $name:ident {
            $(#[$variant1_attr:meta])*
            $variant1:ident = $value1:literal,
            $(#[$variant2_attr:meta])*
            $variant2:ident = $value2:literal $(,)?
        }
    ) => {
        $(#[$attr])*
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        pub enum $name {
            $(#[$variant1_attr])*
            $variant1 = $value1,
            $(#[$variant2_attr])*
            $variant2 = $value2,
        }

        $crate::periph! {
            @impl_field $bits $name value $type: $start;
            value as $type;
            match value {
                $value1 => ::core::result::Result::Ok($name::$variant1),
                $value2 => ::core::result::Result::Ok($name::$variant2),
                _ => ::core::result::Result::Err($crate::InvalidValue),
            };
        }

        impl ::core::ops::Not for $name {
            type Output = $name;

            fn not(self) -> $name {
                match self {
                    $name::$variant1 => $name::$variant2,
                    $name::$variant2 => $name::$variant1,
                }
            }
        }
    };
    (@field_type $bits:ident $type:ty :
        $(#[$attr:meta])*
        $start:literal $(.. $end:literal)? = enum $name:ident {$(
            $(#[$variant_attr:meta])*
            $variant:ident = $value:literal
        ),*$(,)?}
    ) => {
        $(#[$attr])*
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        pub enum $name {$(
            $(#[$variant_attr])*
            $variant = $value
        ),*}

        $crate::periph! {
            @impl_field $bits $name value $type: $start $(.. $end)?;
            value as $type;
            match value {
                $($value => ::core::result::Result::Ok($name::$variant),)*
                _ => ::core::result::Result::Err($crate::InvalidValue),
            };
        }
    };
    (@field_type $bits:ident $type:ty :
        $(#[$attr:meta])*
        $start:literal = struct $name:ident (bool)
    ) => {
        $(#[$attr])*
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        pub struct $name(pub bool);

        $crate::periph! {
            @impl_field $bits $name value $type: $start;
            ::core::convert::Into::<$type>::into(value.0);
            ::core::result::Result::Ok($name(value != 0));
        }

        impl ::core::ops::Not for $name {
            type Output = $name;

            fn not(self) -> $name {
                $name(!self.0)
            }
        }
    };
    (@field_type $bits:ident $type:ty :
        $(#[$attr:meta])*
        $start:literal $(.. $end:literal)? = struct $name:ident ($inner:ty)
    ) => {
        $(#[$attr])*
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        pub struct $name(pub $inner);

        $crate::periph! {
            @impl_field $bits $name value $type: $start $(.. $end)?;
            ::core::convert::Into::<$type>::into(value.0);
            ::core::convert::TryInto::<$inner>::try_into(value).map($name).map_err(|_| $crate::InvalidValue);
        }
    };
    (@field_type $bits:ident $type:ty :
        $(#[$attr:meta])* $start:literal $(.. $end:literal)? = extern $name:ident
    ) => {
        const _: () = {
            let bit_len = ::core::mem::size_of::<$type>() * 8;
            // $end if provided, $start otherwise
            let end = $start $( - $start + $end)? + 1;
            // Compute the mask
            let mask: $type = !0 << (bit_len - end) >> (bit_len - end + $start) << $start;
            // Check MASK and OFFSET of the type. Causes overflow if they are not correct
            let _ = ($name::MASK == mask) as usize - 1;
            let _ = ($name::OFFSET == $start) as usize - 1;
        };
    };

    (@impl_field $bits:ident $name:ident $value:ident $type:ty: $start:literal $(.. $end:literal)?;
        $to_type:expr; $from_type:expr;
    ) => {
        impl $name {
            const MASK: $type = {
                let bit_len = ::core::mem::size_of::<$type>() * 8;
                // $end if provided, $start otherwise
                let end = $start $( - $start + $end)? + 1;
                // Compute the mask
                !0 << (bit_len - end) >> (bit_len - end + $start) << $start
            };
            const OFFSET: $type = $start;
        }

        impl<T: ::core::convert::Into<$bits>> ::core::ops::BitOr<T> for $name {
            type Output = $bits;

            fn bitor(self, other: T) -> $bits {
                let lhs = ::core::convert::Into::<$bits>::into(self);
                let rhs = other.into();
                $bits {
                    bits: lhs.bits & rhs.mask | rhs.bits,
                    mask: lhs.mask & rhs.mask,
                }
            }
        }

        impl ::core::convert::From<$name> for $bits {
            #[inline]
            fn from($value: $name) -> $bits {
                $bits {
                    bits: $to_type << $name::OFFSET,
                    mask: !$name::MASK,
                }
            }
        }

        impl ::core::convert::TryFrom<$type> for $name {
            type Error = $crate::InvalidValue;

            #[inline]
            fn try_from($value: $type) -> ::core::result::Result<$name, $crate::InvalidValue> {
                $from_type
            }
        }
    }
}
