mod device;
mod field_type;
mod periph;
mod register;

#[macro_export]
#[doc(hidden)]
macro_rules! periph_attr_inner {
    (@expand $($rest:tt)*) => { $($rest)* };

    (@$kind:ident {} {$($attr:tt)*} { $callback:ident: @ $rule:ident $($args:tt)*}) => {
        $crate::$callback!(@$rule $($attr)* $($args)*);
    };

    (@type {[$(type)? $(all)?: $attr:meta] $($rest:tt)*} {$($exp:tt)*} $callback:tt) => {
        $crate::periph_attr_inner!(@type {$($rest)*} {$($exp)* #[$attr]} $callback);
    };
    (@type {[$kind:ident: $attr:meta] $($rest:tt)*} {$($exp:tt)*} $callback:tt) => {
        $crate::periph_attr_inner!(@type {$($rest)*} {$($exp)*} $callback);
    };
    (@type {[$attr:meta] $($rest:tt)*} {$($exp:tt)*} $callback:tt) => {
        $crate::periph_attr_inner!(@type {$($rest)*} {$($exp)* #[$attr]} $callback);
    };

    (@field {[$(field)? $(all)?: $attr:meta] $($rest:tt)*} {$($exp:tt)*} $callback:tt) => {
        $crate::periph_attr_inner!(@field {$($rest)*} {$($exp)* #[$attr]} $callback);
    };
    (@field {[$kind:ident: $attr:meta] $($rest:tt)*} {$($exp:tt)*} $callback:tt) => {
        $crate::periph_attr_inner!(@field {$($rest)*} {$($exp)*} $callback);
    };
    (@field {[$attr:meta] $($rest:tt)*} {$($exp:tt)*} $callback:tt) => {
        $crate::periph_attr_inner!(@field {$($rest)*} {$($exp)* #[$attr]} $callback);
    };

    (@impl {[$(impl)? $(all)?: $attr:meta] $($rest:tt)*} {$($exp:tt)*} $callback:tt) => {
        $crate::periph_attr_inner!(@impl {$($rest)*} {$($exp)* #[$attr]} $callback);
    };
    (@impl {[$kind:ident: $attr:meta] $($rest:tt)*} {$($exp:tt)*} $callback:tt) => {
        $crate::periph_attr_inner!(@impl {$($rest)*} {$($exp)*} $callback);
    };
    (@impl {[$attr:meta] $($rest:tt)*} {$($exp:tt)*} $callback:tt) => {
        $crate::periph_attr_inner!(@impl {$($rest)*} {$($exp)*} $callback);
    };
}
