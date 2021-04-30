/// Define a microcontroller and which peripherals it has
///
/// It is recommended to have one module per device and thus to invoke this macros in its own
/// module. For an example of the generated api, see the [example module](crate::example::device).
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
///   peripheral. That is, each register will be at (_base address_ + _register offset_).
/// - The peripheral type is a struct defined with the [`periph!`] macro. You can either import
///   each used peripheral or use absolute or relative paths.

#[macro_export]
macro_rules! device {
    (
        $(#[$device_attr:meta])*
        $device:ident;
        $($(#[$periph_attr:meta])*
        $periph:ident @ $base:literal : $type:ty;)*
    ) => {
        $crate::paste!{
            $(#[$device_attr])*
            #[derive(Debug)]
            pub struct $device {$(
                $(#[$periph_attr])*
                pub [<$periph:lower>]: $type<$periph>,
            )*}
        }

        $($(#[$periph_attr])*
        #[derive(Debug)]
        #[allow(non_camel_case_types)]
        pub enum $periph {}

        unsafe impl $crate::Peripheral for $periph {
            const BASE: usize = $base;
        })*
    }
}
