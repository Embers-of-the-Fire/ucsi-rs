use core::fmt;

use cfg_if::cfg_if;

use crate::fraction::Fraction;

use super::any::{SiAnyUnit, SiDefinedUnit, SiDefinedUnitDefinition, SiOpsUnit};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SiBaseUnitDefinition {
    pub full_name: &'static str,
    pub short_name: &'static str,
    pub unit_symbol: &'static str,
}

pub trait SiBaseUnit: SiDefinedUnit + SiOpsUnit + SiAnyUnit {
    const DEF: SiBaseUnitDefinition;
}

macro_rules! __impl_si_base_unit_definition {
    {
        alias $vis:vis mod $ident:ident;

        $(unit $name:ident field $field:ident {
            $($row:ident: $val:expr),*
            $(,)?
        } $(alias $($al:ident),+ $(,)?)?;)*
    } => {
        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct $name;

            impl SiBaseUnit for $name {
                const DEF: SiBaseUnitDefinition = SiBaseUnitDefinition {
                    $($row: $val),*
                };
            }

            impl SiDefinedUnit for $name {
                const DEF: Option<SiDefinedUnitDefinition> = Some(SiDefinedUnitDefinition {
                    full_name: <$name as SiBaseUnit>::DEF.full_name,
                    short_name: <$name as SiBaseUnit>::DEF.short_name,
                    unit_symbol: <$name as SiBaseUnit>::DEF.unit_symbol,
                });
            }

            impl SiOpsUnit for $name {
                const UNIT_MAP: BaseUnitMap = BaseUnitMap {
                    $field: Fraction::ONE,
                    ..BaseUnitMap::EMPTY
                };
            }

            impl $crate::core::units::any::SiAnyUnit for $name {}
        )+

        $vis mod $ident {
            $($($(
                #[allow(non_camel_case_types)]
                pub type $al = super::$name;
            )+)*)?
        }
    };
}

__impl_si_base_unit_definition! {
    alias pub(crate) mod impl_aliases;

    unit Meter field meter {
        full_name: "Meter",
        short_name: "meter",
        unit_symbol: "m",
    } alias m;

    unit Second field second {
        full_name: "Second",
        short_name: "second",
        unit_symbol: "s",
    } alias s;

    unit Kilogram field kilogram {
        full_name: "Kilogram",
        short_name: "kilogram",
        unit_symbol: "kg",
    } alias kg;

    unit Ampere field ampere {
        full_name: "Ampere",
        short_name: "ampere",
        unit_symbol: "A",
    } alias A;

    unit Kelvins field kelvins {
        full_name: "Kelvins",
        short_name: "kelvins",
        unit_symbol: "K",
    } alias K;

    unit Mole field mole {
        full_name: "Mole",
        short_name: "mole",
        unit_symbol: "mol",
    } alias mol;

    unit Candela field candela {
        full_name: "Candela",
        short_name: "candela",
        unit_symbol: "cd",
    } alias cd;
}

cfg_if! {
    if #[cfg(feature = "alias_hidden")] {
    } else if #[cfg(feature = "alias_mod")] {
        pub mod aliases {
            pub use super::impl_aliases::*;
        }
    } else if #[cfg(feature = "alias_export")] {
        pub use impl_aliases::*;
    }
}

/// # Pure value type
/// 
/// This struct represents **no unit** value.
/// 
/// Implementing two sets of operators (`ops::XXX`) for arbitrary values and arbitrary `Value` values
/// is not possible due to rust stable's current trait mechanism.
/// Therefore `PureValue` is provided here to represent unitless quantities.
/// 
/// You can use the `cadd/sub/xxx` or `padd/sub/xxx` methods implemented for `Value<int/uint/float, T>`,
/// but these are only implemented for basic types.
pub struct PureValue;

impl SiOpsUnit for PureValue {
    const UNIT_MAP: BaseUnitMap = BaseUnitMap::EMPTY;
}

impl SiDefinedUnit for PureValue {
    const DEF: Option<SiDefinedUnitDefinition> = None;
}

impl SiAnyUnit for PureValue {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaseUnitMap {
    pub meter: Fraction,
    pub second: Fraction,
    pub kilogram: Fraction,
    pub ampere: Fraction,
    pub kelvins: Fraction,
    pub mole: Fraction,
    pub candela: Fraction,
}

impl fmt::Display for BaseUnitMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut it = self
            .unit_fields()
            .into_iter()
            .filter(|(_, x)| !x.is_zero())
            .peekable();

        if it.peek().is_none() {
            return write!(f, "pure value");
        }

        while let Some((name, num)) = it.next() {
            write!(f, "{}^({})", name, num)?;
            if it.peek().is_some() {
                write!(f, " + ")?;
            }
        }

        Ok(())
    }
}

impl BaseUnitMap {
    pub const EMPTY: BaseUnitMap = BaseUnitMap {
        meter: Fraction::ZERO,
        second: Fraction::ZERO,
        kilogram: Fraction::ZERO,
        ampere: Fraction::ZERO,
        kelvins: Fraction::ZERO,
        mole: Fraction::ZERO,
        candela: Fraction::ZERO,
    };

    pub const fn unit_fields(&self) -> [(&'static str, &Fraction); 7] {
        [
            (<Meter as SiBaseUnit>::DEF.unit_symbol, &self.meter),
            (<Second as SiBaseUnit>::DEF.unit_symbol, &self.second),
            (<Kilogram as SiBaseUnit>::DEF.unit_symbol, &self.kilogram),
            (<Ampere as SiBaseUnit>::DEF.unit_symbol, &self.ampere),
            (<Kelvins as SiBaseUnit>::DEF.unit_symbol, &self.kelvins),
            (<Mole as SiBaseUnit>::DEF.unit_symbol, &self.mole),
            (<Candela as SiBaseUnit>::DEF.unit_symbol, &self.candela),
        ]
    }

    #[inline]
    pub const fn add(&self, other: Self) -> Self {
        Self {
            meter: self.meter.add(other.meter),
            second: self.second.add(other.second),
            kilogram: self.kilogram.add(other.kilogram),
            ampere: self.ampere.add(other.ampere),
            kelvins: self.kelvins.add(other.kelvins),
            mole: self.mole.add(other.mole),
            candela: self.candela.add(other.candela),
        }
        .simplify()
    }

    #[inline]
    pub const fn simplify(mut self) -> Self {
        self.meter = self.meter.simplify();
        self.second = self.second.simplify();
        self.kilogram = self.kilogram.simplify();
        self.ampere = self.ampere.simplify();
        self.kelvins = self.kelvins.simplify();
        self.mole = self.mole.simplify();
        self.candela = self.candela.simplify();
        self
    }

    #[inline]
    pub const fn neg(mut self) -> Self {
        self.meter = self.meter.neg();
        self.second = self.second.neg();
        self.kilogram = self.kilogram.neg();
        self.ampere = self.ampere.neg();
        self.kelvins = self.kelvins.neg();
        self.mole = self.mole.neg();
        self.candela = self.candela.neg();
        self.simplify()
    }

    #[inline]
    pub const fn imul(&self, rhs: i32) -> Self {
        Self {
            meter: self.meter.imul(rhs),
            second: self.second.imul(rhs),
            kilogram: self.kilogram.imul(rhs),
            ampere: self.ampere.imul(rhs),
            kelvins: self.kelvins.imul(rhs),
            mole: self.mole.imul(rhs),
            candela: self.candela.imul(rhs),
        }
        .simplify()
    }

    #[inline]
    pub const fn fmul(&self, f: Fraction) -> Self {
        Self {
            meter: self.meter.fmul(f),
            second: self.second.fmul(f),
            kilogram: self.kilogram.fmul(f),
            ampere: self.ampere.fmul(f),
            kelvins: self.kelvins.fmul(f),
            mole: self.mole.fmul(f),
            candela: self.candela.fmul(f),
        }
        .simplify()
    }

    #[inline]
    pub const fn eq(&self, rhs: &Self) -> bool {
        self.meter.eq(&rhs.meter)
            && self.second.eq(&rhs.second)
            && self.kilogram.eq(&rhs.kilogram)
            && self.ampere.eq(&rhs.ampere)
            && self.kelvins.eq(&rhs.kelvins)
            && self.mole.eq(&rhs.mole)
            && self.candela.eq(&rhs.candela)
    }
}
