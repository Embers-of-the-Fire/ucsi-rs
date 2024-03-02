//! See [the module-level documentation](super) for more information.

use core::marker::PhantomData;

use crate::{
    core::units::{any::{SiOpsUnit, SiAnyUnit}, base::BaseUnitMap},
    fraction::Fraction,
};

/// `TypeA * Type B`
/// 
/// This is not recommended to write the definition manually.
/// See [the `unit` macro][unit-macro] for more information.
/// 
/// [unit-macro]: crate::unit
pub struct Mul<L: SiOpsUnit, R: SiOpsUnit> {
    _l: PhantomData<L>,
    _r: PhantomData<R>,
}

impl<L: SiOpsUnit, R: SiOpsUnit> Mul<L, R> {
    /// Create an instance of the `Mul` type.
    pub const fn new() -> Mul<L, R> {
        Mul {
            _l: PhantomData,
            _r: PhantomData,
        }
    }
}

impl<L: SiOpsUnit, R: SiOpsUnit> SiOpsUnit for Mul<L, R> {
    const UNIT_MAP: BaseUnitMap = L::UNIT_MAP.add(R::UNIT_MAP);
}

impl<L: SiOpsUnit, R: SiOpsUnit> SiAnyUnit for Mul<L, R> {}


/// `TypeA / Type B`
/// 
/// This is not recommended to write the definition manually.
/// See [the `unit` macro][unit-macro] for more information.
/// 
/// [unit-macro]: crate::unit
pub struct Div<N: SiOpsUnit, D: SiOpsUnit> {
    _n: PhantomData<N>,
    _d: PhantomData<D>,
}

impl<N: SiOpsUnit, D: SiOpsUnit> Div<N, D> {
    /// Create an instance of the `Div` type.
    pub const fn new() -> Div<N, D> {
        Div {
            _n: PhantomData,
            _d: PhantomData,
        }
    }
}

impl<N: SiOpsUnit, D: SiOpsUnit> SiOpsUnit for Div<N, D> {
    const UNIT_MAP: BaseUnitMap = N::UNIT_MAP.add(D::UNIT_MAP.neg());
}

impl<N: SiOpsUnit, D: SiOpsUnit> SiAnyUnit for Div<N, D> {}


/// `TypeA ** { int32 }`
/// 
/// This is not recommended to write the definition manually.
/// See [the `unit` macro][unit-macro] for more information.
/// 
/// [unit-macro]: crate::unit
pub struct PowI<B: SiOpsUnit, const P: i32> {
    _b: PhantomData<B>,
}

impl<B: SiOpsUnit, const P: i32> PowI<B, P> {
    /// Create an instance of the `PowI` type.
    pub const fn new() -> PowI<B, P> {
        PowI { _b: PhantomData }
    }
}

impl<B: SiOpsUnit, const P: i32> SiOpsUnit for PowI<B, P> {
    const UNIT_MAP: BaseUnitMap = B::UNIT_MAP.imul(P);
}

impl<B: SiOpsUnit, const P: i32> SiAnyUnit for PowI<B, P> {}


/// `TypeA ** { int32 / uint32 }`
/// 
/// This is not recommended to write the definition manually.
/// See [the `unit` macro][unit-macro] for more information.
/// 
/// [unit-macro]: crate::unit
pub struct PowFrac<B: SiOpsUnit, const N: i32, const D: u32> {
    _b: PhantomData<B>,
}

impl<B: SiOpsUnit, const N: i32, const D: u32> PowFrac<B, N, D> {
    /// Create an instance of the `PowFrac` type.
    pub const fn new() -> PowFrac<B, N, D> {
        PowFrac { _b: PhantomData }
    }
}

impl<B: SiOpsUnit, const N: i32, const D: u32> SiOpsUnit for PowFrac<B, N, D> {
    const UNIT_MAP: BaseUnitMap = B::UNIT_MAP.fmul(Fraction::new(N, D));
}

impl<B: SiOpsUnit, const N: i32, const D: u32> SiAnyUnit for PowFrac<B, N, D> {}
