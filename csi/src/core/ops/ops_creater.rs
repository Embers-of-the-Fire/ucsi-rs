//! See [the module-level documentation](super) for more information.

/// See [the module-level documentation](super) for more information.
#[macro_export]
macro_rules! unit {
    ($tt:tt) => {
        $crate::__unit_impl!($tt)
    };
    ($($tt:tt)+) => {
        $crate::__unit_impl!(($($tt)*))
    };
}

/// Internal implementation, do not use.
#[doc(hidden)]
#[macro_export]
macro_rules! __unit_impl {
    (($tt1:tt / $tt2:tt)) => {
        $crate::core::ops::Div<$crate::__unit_impl!($tt1), $crate::__unit_impl!($tt2)>
    };
    (($tt1:tt * $tt2:tt)) => {
        $crate::core::ops::Mul<$crate::__unit_impl!($tt1), $crate::__unit_impl!($tt2)>
    };
    (($tt1:tt ** { $tt2:literal })) => {
        $crate::core::ops::PowI<$crate::__unit_impl!($tt1), $tt2>
    };
    (($tt1:tt ** { $lit1:literal / $lit2:literal })) => {
        $crate::core::ops::PowFrac<$crate::__unit_impl!($tt1), $lit1, $lit2>
    };


    ($type:ty) => {
        $type
    };
    (( $expr:expr )) => {
        $expr
    };
}
