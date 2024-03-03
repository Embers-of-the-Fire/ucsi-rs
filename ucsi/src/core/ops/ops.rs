//! See [the module-level documentation](super) for more information.

use core::{fmt, marker::PhantomData};

use crate::{
    core::units::{
        any::{SiAnyUnit, SiOpsUnit},
        base::BaseUnitMap,
    },
    fraction::Fraction,
    units::any::SiDisplayableUnit,
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

impl<L: SiOpsUnit + SiDisplayableUnit, R: SiOpsUnit + SiDisplayableUnit> SiDisplayableUnit
    for Mul<L, R>
{
    const DISPLAYABLE: bool = true;

    fn display_symbol(w: &mut impl fmt::Write) -> fmt::Result {
        L::display_symbol_wrapped(w)?;
        if L::DISPLAYABLE && R::DISPLAYABLE {
            write!(w, " * ")?;
        }
        R::display_symbol_wrapped(w)
    }

    fn display_symbol_wrapped(w: &mut impl fmt::Write) -> fmt::Result {
        if L::DISPLAYABLE && R::DISPLAYABLE {
            write!(w, "(")?;
            Self::display_symbol(w)?;
            write!(w, ")")
        } else {
            Self::display_symbol(w)
        }
    }
}

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

impl<N: SiOpsUnit + SiDisplayableUnit, D: SiOpsUnit + SiDisplayableUnit> SiDisplayableUnit
    for Div<N, D>
{
    const DISPLAYABLE: bool = true;

    fn display_symbol(w: &mut impl fmt::Write) -> fmt::Result {
        N::display_symbol_wrapped(w)?;
        if N::DISPLAYABLE && D::DISPLAYABLE {
            write!(w, " / ")?;
        }
        D::display_symbol_wrapped(w)
    }

    fn display_symbol_wrapped(w: &mut impl fmt::Write) -> fmt::Result {
        if N::DISPLAYABLE && D::DISPLAYABLE {
            write!(w, "(")?;
            Self::display_symbol(w)?;
            write!(w, ")")
        } else {
            Self::display_symbol(w)
        }
    }
}

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

impl<B: SiOpsUnit + SiDisplayableUnit, const P: i32> SiDisplayableUnit for PowI<B, P> {
    const DISPLAYABLE: bool = B::DISPLAYABLE;

    fn display_symbol(w: &mut impl fmt::Write) -> fmt::Result {
        if B::DISPLAYABLE {
            B::display_symbol_wrapped(w)?;
            write!(w, " ** {{{}}}", P)
        } else {
            Ok(())
        }
    }

    fn display_symbol_wrapped(w: &mut impl fmt::Write) -> fmt::Result {
        if B::DISPLAYABLE {
            write!(w, "(")?;
            B::display_symbol_wrapped(w)?;
            write!(w, "**{{{}}})", P)
        } else {
            Ok(())
        }
    }
}

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

impl<B: SiOpsUnit + SiDisplayableUnit, const N: i32, const D: u32> SiDisplayableUnit for PowFrac<B, N, D> {
    const DISPLAYABLE: bool = B::DISPLAYABLE;

    fn display_symbol(w: &mut impl fmt::Write) -> fmt::Result {
        if B::DISPLAYABLE {
            B::display_symbol_wrapped(w)?;
            write!(w, " ** {{{}/{}}}", N, D)
        } else {
            Ok(())
        }
    }

    fn display_symbol_wrapped(w: &mut impl fmt::Write) -> fmt::Result {
        if B::DISPLAYABLE {
            write!(w, "(")?;
            B::display_symbol_wrapped(w)?;
            write!(w, "**{{{}/{}}})", N, D)
        } else {
            Ok(())
        }
    }
}
