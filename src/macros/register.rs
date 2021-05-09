/// Define a register model that can be shared accross peripherals
///
/// For an example of the generated types, see the [example module](crate::example::register).
///
/// # Usage
///
/// The macro takes the name of the register, its underlying type (u8, u16, etc.) and its reset
/// value. The reset value is the value the register has right after a reset or a boot, `0x1234` in
/// the example below.
///
/// This is followed by the fields list. It takes the field name, the position of the field and the
/// type that represents the field. The position is either an inclusive range of bits or a single
/// bit. Fields defined with a single bit are toggleable.
///
/// The field type can be one of:
///
/// - `struct`: A unit struct over an other type, which must implement `::core::convert::Into` and `TryFrom` for the
///   register type (e.g. `u16`)
/// - `enum`: An enum over all possible values, which is expected to be exhaustive
/// - `extern`: An existing type that can be converted to and from the register type. This allows
///   to define and use more complex types. Note that you can't use the same type twice.
///
/// ```
/// peripherals::register! {
///     RegisterName: u16 = 0x1234 {
///         EXTERN: 0..2 = extern Type;
///         NEWTYPE: 3 = struct Newtype(bool);
///         ENUM: 4..5 = enum Enum {
///             False = 0,
///             True = 1,
///         }
///     }
/// }
///
/// peripherals::field_type! {
///     struct Type [u16] (u8);
/// }
/// ```

#[macro_export]
macro_rules! register {
    ($(#[$($attr:tt)*])* $reg:ident: $type:ty = $reset:literal {$($fields:tt)*}) => {
        $crate::periph_attr_inner! { @type { $([$($attr)*])* } {} {
        periph_attr_inner: @expand
            #[derive(Debug)]
            pub enum $reg {}
        }}

        $crate::periph_attr_inner! { @impl { $([$($attr)*])* } {} {
        periph_attr_inner: @expand
            impl $reg {
                $crate::register_inner!(@reg $reg $type: $($fields)*);
            }
        }}

        $crate::periph_attr_inner! { @impl { $([$($attr)*])* } {} {
        periph_attr_inner: @expand
            impl $crate::RegisterValue for $reg {
                type Int = $type;
                const RESET: $type = $reset;
                const NAME: &'static str = stringify!($reg);
            }
        }}

        $crate::periph_attr_inner! { @impl { $([$($attr)*])* } {} { register_inner: @mucher $reg $type: $($fields)* }}
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! register_inner {
    (@mucher $(#[$impl_attr:meta])* $reg:ident $type:ty: ) => {};
    (@mucher
        $(#[$impl_attr:meta])* $reg:ident $type:ty: $(#[$($attr:tt)*])*
        $field:ident: $start:literal $(.. $end:literal)? = enum $name:ident $desc:tt $($rest:tt)*
    ) => {
        $crate::field_type!($(#[$($attr)*])* enum $name [$type] $desc);
        $crate::register_inner!(@impl $(#[$impl_attr])* $reg, $type, $field, $name, $start $($end)?);
        $crate::register_inner!(@mucher $(#[$impl_attr])* $reg $type: $($rest)*);
    };
    (@mucher
        $(#[$impl_attr:meta])* $reg:ident $type:ty: $(#[$($attr:tt)*])*
        $field:ident: $start:literal $(.. $end:literal)? = struct $name:ident $desc:tt; $($rest:tt)*
    ) => {
        $crate::field_type!($(#[$($attr)*])* struct $name [$type] $desc;);
        $crate::register_inner!(@impl $(#[$impl_attr])* $reg, $type, $field, $name, $start $($end)?);
        $crate::register_inner!(@mucher $(#[$impl_attr])* $reg $type: $($rest)*);
    };
    (@mucher
        $(#[$impl_attr:meta])* $reg:ident $type:ty: $(#[$($attr:tt)*])*
        $field:ident: $start:literal $(.. $end:literal)? = extern $name:ty; $($rest:tt)*
    ) => {
        $crate::register_inner!(@impl $(#[$impl_attr])* $reg, $type, $field, $name, $start $($end)?);
        $crate::register_inner!(@mucher $(#[$impl_attr])* $reg $type: $($rest)*);
    };

    (@impl $(#[$attr:meta])* $reg:ident, $type:ty, $field:ident, $name:ty, $start:literal $end:literal) => {
        $(#[$attr])*
        impl ::core::convert::From<$name> for $crate::FieldValues<$reg> {
            #[inline]
            fn from(value: $name) -> $crate::FieldValues<$reg> {
                unsafe { $crate::FieldValues::from_raw(::core::convert::Into::<$type>::into(value) << $start, $reg::$field.mask()) }
            }
        }

        $(#[$attr])*
        impl $crate::MayToggle for $name {
            type Toggle = ();
        }

        $(#[$attr])*
        impl<T: ::core::convert::Into<$crate::FieldValues<$reg>>> ::core::ops::BitOr<T> for $name
        where
            T: $crate::MayToggle,
        {
            type Output = $crate::FieldValues<$reg>;

            #[inline]
            fn bitor(self, other: T) -> Self::Output {
                ::core::convert::Into::<$crate::FieldValues<$reg>>::into(self) | other
            }
        }

        $(#[$attr])*
        impl<T: ::core::convert::Into<$crate::Fields<$reg>>> ::core::ops::BitAnd<T> for $name
        where
            T: $crate::MayToggle,
        {
            type Output = $crate::FieldValues<$reg, T::Toggle>;

            #[inline]
            fn bitand(self, other: T) -> Self::Output {
                ::core::convert::Into::<$crate::FieldValues<$reg>>::into(self) & other
            }
        }

        $(#[$attr])*
        impl<T: ::core::convert::Into<$crate::Fields<$reg>>> ::core::ops::BitXor<T> for $name
        where
            T: $crate::MayToggle<Toggle = $crate::Toggle>,
        {
            type Output = $crate::FieldValues<$reg>;

            #[inline]
            fn bitxor(self, other: T) -> Self::Output {
                ::core::convert::Into::<$crate::FieldValues<$reg>>::into(self) ^ other
            }
        }
    };

    (@impl  $(#[$attr:meta])* $reg:ident, $type:ty, $field:ident, $name:ty, $start:literal) => {
        $(#[$attr])*
        impl ::core::convert::From<$name> for $crate::FieldValues<$reg, $crate::Toggle> {
            #[inline]
            fn from(value: $name) -> $crate::FieldValues<$reg, $crate::Toggle> {
                unsafe { $crate::FieldValues::from_raw(::core::convert::Into::<$type>::into(value) << $start, $reg::$field.mask()) }
            }
        }

        $(#[$attr])*
        impl $crate::MayToggle for $name {
            type Toggle = $crate::Toggle;
        }

        $(#[$attr])*
        impl<T: ::core::convert::Into<$crate::FieldValues<$reg>>> ::core::ops::BitOr<T> for $name
        where
            T: $crate::MayToggle,
        {
            type Output = $crate::FieldValues<$reg, T::Toggle>;

            #[inline]
            fn bitor(self, other: T) -> Self::Output {
                ::core::convert::Into::<$crate::FieldValues<$reg, $crate::Toggle>>::into(self) | other
            }
        }

        $(#[$attr])*
        impl<T: ::core::convert::Into<$crate::Fields<$reg>>> ::core::ops::BitAnd<T> for $name
        where
            T: $crate::MayToggle,
        {
            type Output = $crate::FieldValues<$reg, $crate::Toggle>;

            #[inline]
            fn bitand(self, other: T) -> Self::Output {
                ::core::convert::Into::<$crate::FieldValues<$reg, $crate::Toggle>>::into(self) & other
            }
        }

        $(#[$attr])*
        impl<T: ::core::convert::Into<$crate::Fields<$reg>>> ::core::ops::BitXor<T> for $name
        where
            T: $crate::MayToggle,
        {
            type Output = $crate::FieldValues<$reg, $crate::Toggle>;

            #[inline]
            fn bitxor(self, other: T) -> Self::Output {
                ::core::convert::Into::<$crate::FieldValues<$reg, $crate::Toggle>>::into(self) ^ other
            }
        }
    };

    (@reg $reg:ident $type:ty: ) => {};
    (@reg $reg:ident $type:ty: $(#[$($attr:tt)*])*
        $field:ident: $start:literal $(.. $end:literal)? = enum $name:ident $desc:tt $($rest:tt)*
    ) => {
        $crate::periph_attr_inner! { @field { $([$($attr)*])* } {} {
        periph_attr_inner: @expand
            pub const $field: $crate::Field<$reg, $name, $type> = unsafe { $crate::Field::from_raw({
                let front = ::core::mem::size_of::<$type>() * 8 $(- $end + $start)? - 1;
                // Compute the mask
                !0 >> front << $start
            }, $start) };
        }}
        $crate::register_inner!(@reg $reg $type: $($rest)*);
    };
    (@reg $reg:ident $type:ty: $(#[$($attr:tt)*])*
        $field:ident: $start:literal $(.. $end:literal)? = struct $name:ident $desc:tt; $($rest:tt)*
    ) => {
        $crate::periph_attr_inner! { @field { $([$($attr)*])* } {} {
        periph_attr_inner: @expand
            pub const $field: $crate::Field<$reg, $name, $type> = unsafe { $crate::Field::from_raw({
                let front = ::core::mem::size_of::<$type>() * 8 $(- $end + $start)? - 1;
                // Compute the mask
                !0 >> front << $start
            }, $start) };
        }}
        $crate::register_inner!(@reg $reg $type: $($rest)*);
    };
    (@reg $reg:ident $type:ty: $(#[$($attr:tt)*])*
        $field:ident: $start:literal $(.. $end:literal)? = extern $name:ty; $($rest:tt)*
    ) => {
        $crate::periph_attr_inner! { @field { $([$($attr)*])* } {} {
        periph_attr_inner: @expand
            pub const $field: $crate::Field<$reg, $name, $type> = unsafe { $crate::Field::from_raw({
                let front = ::core::mem::size_of::<$type>() * 8 $(- $end + $start)? - 1;
                // Compute the mask
                !0 >> front << $start
            }, $start) };
        }}
        $crate::register_inner!(@reg $reg $type: $($rest)*);
    };
}
