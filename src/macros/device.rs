/// Define a microcontroller and which peripherals it has
///
/// It is recommended to have one module per device and thus to invoke this macros in its own
/// module. For an example of the generated types, see the [example module](crate::example::device).
///
/// # Usage
///
/// The macro begins with the device name.
///
/// ```
/// peripherals::device!{
///     /// Optional documentation
///     MyMicrocontroller;
/// }
/// ```
///
/// Then each peripheral is described as follow:
///
/// ```
/// # peripherals::periph! {
/// #   MyPeripheral;
/// #   rw MY_REG @ 0: u16 = 0 {}
/// # }
/// peripherals::device!{
///     MyMicrocontroller;
///     // name     base address   peripheral type
///        PERIPH @ 0x1234         : MyPeripheral;
/// }
/// ```
///
/// - The name of the peripheral is generaly written in uppercase. It is used to name the marker
///   type for this instance as well as the field (in lowercase) in the device struct.
/// - The base address (here `0x1234`) is the address of the register at offset `0` of the
///   peripheral. That is, each register will be at (base address + register offset).
/// - The peripheral type is a struct defined with the [`periph!`] macro. You can either import
///   each used peripheral or use absolute or relative paths.

#[macro_export]
macro_rules! device {
    (
        $(#[$($device_attr:tt)*])*
        $device:ident;
        $($(#[$($periph_attr:tt)*])*
        $periph:ident @ $base:literal : $type:ty;)*
    ) => {
        $crate::periph_attr_inner! { @type { $([$($device_attr)*])* } {} {
            device_inner: @struct $device {$( $(#[$($periph_attr)*])* $periph $type; )*} {}
        }}

        $($crate::periph_attr_inner! { @type { $([$($periph_attr)*])* } {} {
        periph_attr_inner: @expand
            #[derive(Debug)]
            pub enum $periph {}
        }})*

        $($crate::periph_attr_inner! { @impl { $([$($periph_attr)*])* } {} {
        periph_attr_inner: @expand
            impl $crate::Peripheral for $periph {
                const BASE: usize = $base;
                const NAME: &'static str = stringify!($periph);
            }
        }})*
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! device_inner {
    (@struct $device:ident {} {$(#[$device_attr:meta])*
        $($periph:ident $type:ty; $(#[$attr:meta])*)*
    }) => { $crate::paste! {
        $(#[$device_attr])*
        #[derive(Debug)]
        pub struct $device {$(
            $(#[$attr])*
            pub [<$periph:lower>]: $type<$periph>,
        )*}
    }};
    (@struct $(#[$attr:meta])* $device:ident {} {$($rest:tt)*} ) => {
        $crate::device_inner!(@struct $device {} { $($rest)* $(#[$attr])* } );
    };
    (@struct
        $(#[$prev:meta])* $device:ident
        { $(#[$($attr:tt)*])* $periph:ident $type:ty; $($rest:tt)* }
        { $($parsed:tt)* }
    ) => {
        $crate::periph_attr_inner! { @field { $([$($attr)*])* } {} {
            device_inner: @struct $device { $($rest)* } { $($parsed)* $(#[$prev])* $periph $type; }
        }}
    };
}
