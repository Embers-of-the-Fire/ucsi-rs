/// **Note:** Use the alias `ucsi::macros::cast::cast_si_value`.
///
/// ## Deprecated
///
/// Under most circumstances [`Value::cast`] / [`Value::cast_const`] can be used instead.
///
/// ## Example
///
/// ```rust,ignore
/// use ucsi::units::base::m;
/// use ucsi::macros::cast::cast_si_value;
/// use ucsi::Value;
/// use ucsi::unit;
///
/// let raw: Value<f64, m> = Value::new(10.0);
/// let orig = raw * raw;
/// let checked = cast_si_value!(orig => unit!(m * m) as unit!(m ** { -2 }))
/// ```
///
/// ## Syntax
///
/// ```rust,ignore
/// cast_si_value!(value:expr => TypeA as TypeB)
/// ```
///
/// ## Implementation
///
/// This will invoke [`is_same_type_or_panic`][is_same_type_or_panic] with the two given types.
///
/// **Note:** This will not check that the type given is the same as the type of the expression provided.
///
/// [is_same_type_or_panic]: crate::core::units::any::is_same_type_or_panic
#[macro_export]
macro_rules! __impl_cast_si_value {
    ($origin:expr => $tyA:ty as $tyB:ty) => {{
        const _: () = $crate::core::units::any::is_same_type_or_panic::<$tyA, $tyB>();
        $crate::core::value::Value::<_, $tyB>::new($origin.value)
    }};
}

#[cfg(feature = "infer_cast")]
/// **Note:** Use the alias `ucsi::macros::cast::infer_cast_si_value`.
///
/// ## Deprecated
///
/// Under most circumstances [`Value::cast`] / [`Value::cast_const`] can be used instead.
///
/// ## Syntax
///
/// ```rust,ignore
/// infer_cast_si_value!(value:expr => TargetType)
/// ```
///
/// ## Implementation
///
/// This works like the [`cast_si_value`][__impl_cast_si_value],
/// but infers the origin expression's value with the help of `type_alias_impl_trait`(TAIT).
///
/// You must enable the TAIT feature when you call this macro.
///
/// ```rust,ignore
/// #![feature(type_alias_impl_trait)]
/// ```
///
/// **Note:** This macro requires the `infer_cast` library feature to be enabled.
#[macro_export]
macro_rules! __impl_infer_cast_si_value {
    ($origin:expr => $ty:ty) => {{
        type __Refer = impl $crate::core::units::any::SiOpsUnit;
        let __orig: $crate::core::value::Value<_, __Refer> = $origin;
        const _: () = $crate::core::units::any::is_same_type_or_panic::<__Refer, $ty>();
        $crate::core::value::Value::<_, $ty>::new($origin.value)
    }};
}

#[cfg(feature = "force_assert")]
#[doc(hidden)]
#[macro_export]
macro_rules! __dbg_assert {
    ($($arg:tt)*) => {
        core::assert!($($arg)*)
    };
}

#[cfg(not(feature = "force_assert"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __dbg_assert {
    ($($arg:tt)*) => {
        core::debug_assert!($($arg)*)
    };
}
