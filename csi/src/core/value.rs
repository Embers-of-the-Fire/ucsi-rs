//! # The `value` module
//!
//! The core functionality of this module is the [`Value`][crate::Value] struct.
//! It works as a wrapper that saves the unit information at **compile time**
//! and theoretically zero-cost.
//!
//! ## Example
//!
//! > The following example is the same as [the library-level documentation][crate],
//! you can skip them if you've already read the library-level document.
//!
//! ```rust
//! use csi::units::base::{kg, m, s};
//! use csi::unit;
//! use csi::Value;
//!
//! // create your new unit
//! // see the `unit` macro or `csi::core::ops` for more information.
//! type Newton = unit!((kg * m) / (s ** { 2 }));
//!
//! // build some value
//! // `Value<value_type, value_unit>`
//! let speed: Value<f64, unit!(m / s)> = Value::new(10.0);
//! let time: Value<f64, s> = Value::new(1.0);
//! let acc = speed / time;
//! let mass: Value<f64, kg> = Value::new(1.0);
//! // this is `newton` in SI,
//! // but not now because it's currently typed as `((m / s) / s) * kg`
//! let force = acc * mass;
//! // lets cast it to newton
//! let checked_force: Value<_, Newton> =
//!     // note the `cast_const` here. `f64` is copy, so this cast is const-able.
//!     // If your value is not copy-able, use `cast` instead.
//!     force.cast_const();
//! ```
//!
//! Thanks to rust's const evaluation, the type-casting is **compile-time checked**!
//!
//! To clearify the type-casting check, consider the following example:
//!
//! ```rust,compile_fail
//! # use csi::units::base::{kg, m, s};
//! # use csi::unit;
//! # use csi::Value;
//! # type Newton = unit!((kg * m) / (s ** { 2 }));
//! # let speed: Value<f64, unit!(m / s)> = Value::new(10.0);
//! # let time: Value<f64, s> = Value::new(1.0);
//! # let acc = speed / time;
//! # let mass: Value<f64, kg> = Value::new(1.0);
//! # let force = acc * mass;
//! let s_can_never_be_newton: Value<_, s> = force.cast_const();
//! ```
//!
//! The example above cannot compile, and will throw some error like this:
//!
//! ```plain
//! error[E0080]: evaluation of `<Second as CastFrom<csi::ops::Mul<csi::ops::Div<csi::ops::Div<Meter, Second>, Second>,
//! Kilogram>>>::CAN_CAST_FROM` failed
//! --> D:\WBH\rust\csi\csi\src\core\units\any.rs:25:9
//!    |
//! 25 |         panic!("cannot cast si type")
//!    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'cannot cast si type', D:\WBH\rust\csi\csi\src\core\units\any.rs:25:9
//!    |
//! note: inside `is_same_type_or_panic::<csi::ops::Mul<csi::ops::Div<csi::ops::Div<Meter, Second>, Second>, Kilogram>,
//! Second>`
//! --> D:\WBH\rust\csi\csi\src\core\units\any.rs:25:9
//!    |
//! 25 |         panic!("cannot cast si type")
//!    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//! note: inside `<Second as CastFrom<csi::ops::Mul<csi::ops::Div<csi::ops::Div<Meter, Second>, Second>, Kilogram>>>::CAN_CAST_FROM`
//! --> D:\WBH\rust\csi\csi\src\core\units\any.rs:36:9
//!    |
//! 36 |         is_same_type_or_panic::<T, B>();
//!    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//!  = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `panic` (in Nightly builds, run with -Z macro-backtrace for more info)
//!
//! note: the above error was encountered while instantiating `fn csi::Value::<f64, csi::ops::Mul<csi::ops::Div<csi::ops::Div<csi::units::base::Meter, csi::units::base::Second>, csi::units::base::Second>, csi::units::base::Kilogram>>::cast_const::<csi::units::base::Second>`
//! --> csi-test\tests\test_ops.rs:26:25
//!    |
//! 26 |     let s_can_never_be_newton: Value<_, s> = force.cast_const();
//!    |                                              ^^^^^^^^^^^^^^^^^^
//!
//! For more information about this error, try `rustc --explain E0080`.
//! ```
//!
//! Well, there's probably a little bit of a lot of information that's being reported.
//! Since rust's const evaluation and generic system is still under rapid development,
//! this could be improved in the future. \
//! Anyway, at least we can check units at compile time.

use core::{fmt, marker::PhantomData, ops};

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(all(feature = "no_std", not(feature = "no_alloc")))] {
        extern crate alloc;
        use alloc::{string::String, format};
    }
}

use crate::{units::{any::{CastFrom, SiDefinedUnitDefinition}, base::BaseUnitMap}, SiDefinedUnit};

use super::{
    ops::{Div, Mul, PowFrac, PowI},
    units::any::{is_same_type, SiAnyUnit, SiOpsUnit},
};

pub struct Value<V, T: SiAnyUnit> {
    pub value: V,
    _t: PhantomData<T>,
}

impl<T: SiAnyUnit, V: fmt::Debug> fmt::Debug for Value<V, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Value").field(&self.value).finish()
    }
}

impl<T: SiAnyUnit, V: Clone> Clone for Value<V, T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            _t: PhantomData,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.value = source.value.clone();
    }
}

impl<T: SiAnyUnit, V: Copy> Copy for Value<V, T> {}

impl<T: SiAnyUnit, V> Value<V, T> {
    #[inline]
    pub const fn new(value: V) -> Value<V, T> {
        Self {
            value,
            _t: PhantomData,
        }
    }

    #[inline]
    pub const fn as_ref(&self) -> &V {
        &self.value
    }

    #[inline]
    pub fn as_mut_ref(&mut self) -> &mut V {
        &mut self.value
    }
}

impl<T: SiAnyUnit + SiOpsUnit, V> Value<V, T> {
    #[inline]
    pub fn cast<B: SiAnyUnit + SiOpsUnit + CastFrom<T>>(self) -> Value<V, B> {
        debug_assert!(B::CAN_CAST_FROM);
        Value::new(self.value)
    }

    #[cfg(not(feature = "no_alloc"))]
    #[inline]
    pub fn try_cast<B: SiAnyUnit + SiOpsUnit>(self) -> Result<Value<V, B>, String> {
        if is_same_type::<T, B>() {
            Ok(Value::new(self.value))
        } else {
            Err(format!(
                r#"
cannot cast SI value
origin: {}
target: {}
"#,
                T::UNIT_MAP,
                B::UNIT_MAP
            ))
        }
    }

    /// # Safety
    ///
    /// This function is not rust-unsafe and will not create any ub.
    ///
    /// However, using this will bypass the lib's static type checking,
    /// so this function is marked as `unsafe`.
    #[inline]
    pub unsafe fn unsafe_cast<B: SiAnyUnit>(self) -> Value<V, B> {
        Value::new(self.value)
    }

    #[inline]
    pub fn powi_type<const N: i32>(self) -> Value<V, PowI<T, N>> {
        Value::new(self.value)
    }

    #[inline]
    pub fn powf_type<const N: i32, const D: u32>(self) -> Value<V, PowFrac<T, N, D>> {
        assert!(D > 0);
        Value::new(self.value)
    }
}

impl<T: SiAnyUnit + SiOpsUnit, V: Copy> Value<V, T> {
    #[inline]
    pub const fn cast_const<B: SiAnyUnit + SiOpsUnit + CastFrom<T>>(self) -> Value<V, B> {
        assert!(B::CAN_CAST_FROM);
        Value::new(self.value)
    }

    #[inline]
    pub const fn powi_type_const<const N: i32>(self) -> Value<V, PowI<T, N>> {
        Value::new(self.value)
    }

    #[inline]
    pub const fn powf_type_const<const N: i32, const D: u32>(self) -> Value<V, PowFrac<T, N, D>> {
        assert!(D > 0);
        Value::new(self.value)
    }
}

impl<T: SiAnyUnit + SiOpsUnit, V: PartialEq> Value<V, T> {
    #[inline]
    pub fn is_value_equal<R: SiAnyUnit + SiOpsUnit>(&self, rhs: &Value<V, R>) -> bool {
        self.value == rhs.value
    }

    #[inline]
    pub fn is_equal<R: SiAnyUnit + SiOpsUnit>(&self, rhs: &Value<V, R>) -> bool {
        is_same_type::<T, R>() && self.is_value_equal(rhs)
    }
}

#[cfg(not(feature = "no_alloc"))]
impl<T: SiAnyUnit + SiOpsUnit, V: fmt::Display> Value<V, T> {
    #[inline]
    pub fn format_si(&self) -> String {
        format!("{} ({})", self.value, T::UNIT_MAP)
    }
}

// ops

macro_rules! __impl_int_ops {
    ($ty0:ty, $($ty:ty),+ $(,)?) => {
        __impl_int_ops!($ty0);
        $(__impl_int_ops!($ty);)+
    };

    ($ty:ty) => {
        impl<T: SiAnyUnit> Value<$ty, T> {
            #[inline]
            pub const fn cadd(mut self, rhs: Value<$ty, T>) -> Value<$ty, T> {
                self.value += rhs.value;
                self
            }

            #[inline]
            pub const fn csub(mut self, rhs: Value<$ty, T>) -> Value<$ty, T> {
                self.value -= rhs.value;
                self
            }

            #[inline]
            pub const fn cmul(mut self, rhs: $ty) -> Value<$ty, T> {
                self.value *= rhs;
                self
            }

            #[inline]
            pub const fn cdiv(mut self, rhs: $ty) -> Value<$ty, T> {
                self.value /= rhs;
                self
            }
        }
    };
}

macro_rules! __impl_float_ops {
    ($ty0:ty, $($ty:ty),+ $(,)?) => {
        __impl_float_ops!($ty0);
        $(__impl_float_ops!($ty);)+
    };

    ($ty:ty) => {
        impl<T: SiAnyUnit> Value<$ty, T> {
            #[inline]
            pub fn padd(mut self, rhs: Value<$ty, T>) -> Value<$ty, T> {
                self.value += rhs.value;
                self
            }

            #[inline]
            pub fn psub(mut self, rhs: Value<$ty, T>) -> Value<$ty, T> {
                self.value -= rhs.value;
                self
            }

            #[inline]
            pub fn pmul(mut self, rhs: $ty) -> Value<$ty, T> {
                self.value *= rhs;
                self
            }

            #[inline]
            pub fn pdiv(mut self, rhs: $ty) -> Value<$ty, T> {
                self.value /= rhs;
                self
            }
        }
    };
}

#[cfg(feature = "const_soft_float")]
macro_rules! __impl_const_float_ops {
    ($($ty:ty),*) => {
        $(
            impl<T: SiAnyUnit> Value<$ty, T> {
                #[inline]
                pub const fn cadd(self, rhs: Self) -> Self {
                    Self::new(self.value.add(rhs.value))
                }

                #[inline]
                pub const fn csub(self, rhs: Self) -> Self {
                    Self::new(self.value.sub(rhs.value))
                }

                #[inline]
                pub const fn cmul(self, rhs: $ty) -> Self {
                    Self::new(self.value.mul(rhs))
                }

                #[inline]
                pub const fn cdiv(self, rhs: $ty) -> Self {
                    Self::new(self.value.div(rhs))
                }
            }
        )*
    };
}

__impl_int_ops!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize);
__impl_float_ops!(f32, f64);
#[cfg(feature = "const_soft_float")]
__impl_const_float_ops!(
    const_soft_float::soft_f32::SoftF32,
    const_soft_float::soft_f64::SoftF64
);

impl<T: SiAnyUnit + SiOpsUnit, V: ops::Add<L>, L> ops::Add<Value<L, T>> for Value<V, T> {
    type Output = Value<V::Output, T>;

    #[inline]
    fn add(self, rhs: Value<L, T>) -> Self::Output {
        Value::new(self.value.add(rhs.value))
    }
}

impl<T: SiAnyUnit + SiOpsUnit, V: ops::Sub<L>, L> ops::Sub<Value<L, T>> for Value<V, T> {
    type Output = Value<V::Output, T>;

    #[inline]
    fn sub(self, rhs: Value<L, T>) -> Self::Output {
        Value::new(self.value.sub(rhs.value))
    }
}

impl<T: SiAnyUnit + SiOpsUnit, V: ops::Mul<L>, L, B: SiAnyUnit + SiOpsUnit> ops::Mul<Value<L, B>>
    for Value<V, T>
{
    type Output = Value<V::Output, Mul<T, B>>;

    #[inline]
    fn mul(self, rhs: Value<L, B>) -> Self::Output {
        Value::new(self.value.mul(rhs.value))
    }
}

impl<T: SiAnyUnit + SiOpsUnit, V: ops::Div<L>, L, B: SiAnyUnit + SiOpsUnit> ops::Div<Value<L, B>>
    for Value<V, T>
{
    type Output = Value<V::Output, Div<T, B>>;

    #[inline]
    fn div(self, rhs: Value<L, B>) -> Self::Output {
        Value::new(self.value.div(rhs.value))
    }
}

impl<T: SiAnyUnit + SiOpsUnit, V: ops::Rem<L>, L, B: SiAnyUnit + SiOpsUnit> ops::Rem<Value<L, B>>
    for Value<V, T>
{
    type Output = Value<V::Output, Div<T, B>>;

    #[inline]
    fn rem(self, rhs: Value<L, B>) -> Self::Output {
        Value::new(self.value.rem(rhs.value))
    }
}

impl<T: SiAnyUnit, V: ops::Neg> ops::Neg for Value<V, T> {
    type Output = Value<V::Output, T>;

    #[inline]
    fn neg(self) -> Self::Output {
        Value::new(self.value.neg())
    }
}

// pure-value

pub struct PureValue;

impl SiAnyUnit for PureValue {}

impl SiOpsUnit for PureValue {
    const UNIT_MAP: BaseUnitMap = BaseUnitMap::EMPTY;
}

impl SiDefinedUnit for PureValue {
    const DEF: Option<SiDefinedUnitDefinition> = None;
}
