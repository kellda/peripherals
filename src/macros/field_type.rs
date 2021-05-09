/// A helper hacro to define a type that can be used in field value.
///
/// It can define either an enum or a newtype struct. It also implement `into` and `try_from` for
/// the given interger types. For an example of the generated types, see the
/// [example module](crate::example::field_type).
///
/// # Usage
///
/// ```
/// use core::convert::{TryFrom, TryInto};
///
/// // Define an enum
/// peripherals::field_type! {
///     enum Mode [u8, u16] {
///         A = 0,
///         B = 1,
///         C = 2,
///         D = 3,
///     }
/// }
///
/// // It implements into and try_from for given interger types (here u8 and u16)
/// assert_eq!(Into::<u8>::into(Mode::A), 0);
/// assert_eq!(u16::from(Mode::B), 1);
/// assert_eq!(TryInto::<Mode>::try_into(2_u8), Ok(Mode::C));
/// assert_eq!(Mode::try_from(3_u16), Ok(Mode::D));
///
/// // Define a newtype struct
/// peripherals::field_type! {
///     struct Data [u16] (u8);
/// }
///
/// // It implements into and try_from for given interger types (here u16)
/// assert_eq!(u16::from(Data(10)), 10);
/// assert_eq!(Data::try_from(20), Ok(Data(20)));
/// ```
///
/// It also implements `Not` for enum with two fields and newtypes over bool. Fields with these
/// types can be toggled.
///
/// ```
/// use core::convert::{TryFrom, TryInto};
///
/// // Define an enum with two variants
/// peripherals::field_type! {
///     enum State [] {
///         Low = 0,
///         High = 1,
///     }
/// }
///
/// // It implements Not
/// assert_eq!(!State::Low, State::High);
/// assert_eq!(!State::High, State::Low);
///
/// // Define a newtype struct over a bool
/// peripherals::field_type! {
///     struct Status [] (bool);
/// }
///
/// // It implements Not
/// assert_eq!(!Status(true), Status(false));
/// assert_eq!(!Status(false), Status(true));
/// ```

#[macro_export]
macro_rules! field_type {
    ($(#[$($attr:tt)*])* enum $name:ident $int:tt {
        $(#[$variant1_attr:meta])*
        $variant1:ident = $value1:literal,
        $(#[$variant2_attr:meta])*
        $variant2:ident = $value2:literal $(,)?
    }) => {
        $crate::periph_attr_inner! { @type { $([$($attr)*])* } {} {
        periph_attr_inner: @expand
            #[derive(Copy, Clone, Debug, PartialEq, Eq)]
            pub enum $name {
                $(#[$variant1_attr])*
                $variant1 = $value1,
                $(#[$variant2_attr])*
                $variant2 = $value2,
            }
        }}

        $crate::periph_attr_inner! { @impl { $([$($attr)*])* } {} {
        periph_attr_inner: @expand
            impl ::core::ops::Not for $name {
                type Output = $name;

                #[inline]
                fn not(self) -> $name {
                    match self {
                        $name::$variant1 => $name::$variant2,
                        $name::$variant2 => $name::$variant1,
                    }
                }
            }
        }}

        $crate::periph_attr_inner! { @impl { $([$($attr)*])* } {} {
        field_type_inner: @outer $name value $int:
            value as _;
            match value {
                $value1 => ::core::result::Result::Ok($name::$variant1),
                $value2 => ::core::result::Result::Ok($name::$variant2),
                _ => ::core::result::Result::Err($crate::InvalidValue),
            };
        }}
    };
    ($(#[$($attr:tt)*])* enum $name:ident $int:tt {$(
        $(#[$variant_attr:meta])*
        $variant:ident = $value:literal
    ),*$(,)?}) => {
        $crate::periph_attr_inner! { @type { $([$($attr)*])* } {} {
        periph_attr_inner: @expand
            #[derive(Copy, Clone, Debug, PartialEq, Eq)]
            pub enum $name {$(
                $(#[$variant_attr])*
                $variant = $value
            ),*}
        }}

        $crate::periph_attr_inner! { @impl { $([$($attr)*])* } {} {
        field_type_inner: @outer $name value $int:
            value as _;
            match value {
                $($value => ::core::result::Result::Ok($name::$variant),)*
                _ => ::core::result::Result::Err($crate::InvalidValue),
            };
        }}
    };
    ($(#[$($attr:tt)*])* struct $name:ident $int:tt (bool);) => {
        $crate::periph_attr_inner! { @type { $([$($attr)*])* } {} {
        periph_attr_inner: @expand
            #[derive(Copy, Clone, Debug, PartialEq, Eq)]
            pub struct $name(pub bool);
        }}

        $crate::periph_attr_inner! { @impl { $([$($attr)*])* } {} {
        periph_attr_inner: @expand
            impl ::core::ops::Not for $name {
                type Output = $name;

                #[inline]
                fn not(self) -> $name {
                    $name(!self.0)
                }
            }
        }}

        $crate::periph_attr_inner! { @impl { $([$($attr)*])* } {} {
        field_type_inner: @outer $name value $int:
            ::core::convert::Into::into(value.0);
            ::core::result::Result::Ok($name(value != 0));
        }}
    };
    ($(#[$($attr:tt)*])* struct $name:ident $int:tt ($inner:ty);) => {
        $crate::periph_attr_inner! { @type { $([$($attr)*])* } {} {
        periph_attr_inner: @expand
            #[derive(Copy, Clone, Debug, PartialEq, Eq)]
            pub struct $name(pub $inner);
        }}

        $crate::periph_attr_inner! { @impl { $([$($attr)*])* } {} {
        field_type_inner: @outer $name value $int:
            ::core::convert::Into::into(value.0);
            ::core::convert::TryInto::try_into(value).map($name).map_err(|_| $crate::InvalidValue);
        }}
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! field_type_inner {
    (@outer $(#[$attr:meta])* $name:ident $value:ident [$($int:ty),*]: $to_type:expr; $from_type:expr;) => {
        $crate::field_type_inner!(@inner {$(#[$attr])*} $name $value [$($int),*]: $to_type; $from_type;);
    };
    (@inner $attr:tt $name:ident $value:ident [$($int:ty),*]: $to_type:expr; $from_type:expr;) => {
        $( $crate::field_type_inner!(@impl $attr $name $value $int: $to_type; $from_type;); )*
    };
    (@impl { $(#[$attr:meta])* } $name:ident $value:ident $int:ty: $to_type:expr; $from_type:expr; ) => {
        $(#[$attr])*
        impl ::core::convert::From<$name> for $int {
            #[inline]
            fn from($value: $name) -> $int {
                $to_type
            }
        }

        $(#[$attr])*
        impl ::core::convert::TryFrom<$int> for $name {
            type Error = $crate::InvalidValue;

            #[inline]
            fn try_from($value: $int) -> ::core::result::Result<$name, $crate::InvalidValue> {
                $from_type
            }
        }
    };
}
