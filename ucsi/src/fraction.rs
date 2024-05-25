//! # The `fraction` module.
//!
//! This module includes a manually implemented constant fraction,
//! which is used to represent the units.
//!
//! Currently rust does not compute floating point numbers in a constant environment,
//! due to some hardware or software issue.
//! So using a simple fraction system can solve this problem to some extent.
//!
//! This feature is not well considered for performance,
//! as they are not designed to be used in real-world applications,
//! and rough implementations are perfectly adequate at compile time.
//! There may be optimisations planned for the future, but at least not right now.
//!
//! You could use this module by enabling `fraction` feature,
//! which is included in the `full` or `internal_utils` feature.

use core::{fmt, num::NonZeroU32};

use crate::macros::unwrap::unwrap_option_const;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fraction(i32, NonZeroU32);

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.1.get() == 1 {
            write!(f, "{}", self.0)
        } else {
            write!(f, "{}/{}", self.0, self.1)
        }
    }
}

impl Fraction {
    pub const ZERO: Fraction = Self::new(0, 1);
    pub const ONE: Fraction = Self::new(1, 1);
    pub const NEG_ONE: Fraction = Self::new(-1, 1);

    #[inline]
    pub const fn new(l: i32, r: u32) -> Fraction {
        assert!(r != 0, "denominator should never be zero");
        Fraction(l, unwrap_option_const!(NonZeroU32::new(r)))
    }

    #[inline]
    pub const fn eq(&self, rhs: &Self) -> bool {
        let lhs = self.simplify();
        let rhs = rhs.simplify();
        lhs.0 == rhs.0 && lhs.1.get() == rhs.1.get()
    }

    #[inline]
    pub const fn numerator(&self) -> i32 {
        self.0
    }

    #[inline]
    pub const fn denominator(&self) -> NonZeroU32 {
        self.1
    }

    #[inline]
    pub const fn simplify(self) -> Self {
        if let Some(left) = NonZeroU32::new(self.0.unsigned_abs()) {
            let g = gcd::binary_nonzero_u32(left, self.1);
            let i32g = g.get() as i32;
            Fraction(
                self.0 / i32g,
                unwrap_option_const!(NonZeroU32::new(self.1.get() / g.get())),
            )
        } else {
            Fraction(0, unwrap_option_const!(NonZeroU32::new(1)))
        }
    }

    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub const fn add(self, rhs: Self) -> Self {
        if self.is_zero() {
            rhs
        } else if rhs.is_zero() {
            self
        } else {
            let Fraction(na, da) = self;
            let Fraction(nb, db) = rhs;
            let g = gcd::binary_nonzero_u32(
                unwrap_option_const!(NonZeroU32::new(na.unsigned_abs())),
                da,
            );
            if g.get() == 1 {
                Fraction(
                    na * db.get() as i32 + da.get() as i32 * nb,
                    unwrap_option_const!(da.checked_mul(db)),
                )
            } else {
                let s = da.get() / g.get();
                let t = na * (db.get() / g.get()) as i32 + nb * s as i32;
                let g2 = gcd::binary_nonzero_u32(
                    unwrap_option_const!(NonZeroU32::new(t.unsigned_abs())),
                    g,
                );
                if g2.get() == 1 {
                    Fraction(t, unwrap_option_const!(NonZeroU32::new(s * db.get())))
                } else {
                    Fraction(
                        t / g2.get() as i32,
                        unwrap_option_const!(NonZeroU32::new(s * (db.get() / g2.get()))),
                    )
                }
            }
        }
        .simplify()
    }

    #[inline]
    pub const fn as_reciprocal(self) -> Self {
        if self.0 == 0 {
            Fraction::ZERO
        } else if self.0 > 0 {
            Fraction(
                self.1.get() as i32,
                unwrap_option_const!(NonZeroU32::new(self.0.unsigned_abs())),
            )
        } else {
            Fraction(
                -(self.1.get() as i32),
                unwrap_option_const!(NonZeroU32::new(self.0.unsigned_abs())),
            )
        }
    }

    #[inline]
    pub const fn neg(self) -> Self {
        Fraction(-self.0, self.1)
    }

    #[inline]
    pub const fn imul(self, rhs: i32) -> Self {
        if rhs == 0 || self.is_zero() {
            Self::ZERO
        } else {
            Fraction(self.0 * rhs, self.1).simplify()
        }
    }

    #[inline]
    pub const fn fmul(self, rhs: Self) -> Self {
        let lhs = self.simplify();
        let rhs = rhs.simplify();
        let lhs2 = Fraction(lhs.0, rhs.1).simplify();
        let rhs2 = Fraction(rhs.0, lhs.1).simplify();
        Fraction(
            lhs2.0 * rhs2.0,
            unwrap_option_const!(lhs2.1.checked_mul(rhs2.1)),
        )
    }

    #[inline]
    pub fn format_plain(&self, w: &mut impl fmt::Write) -> fmt::Result {
        if self.denominator().get() == 1 && self.numerator() >= 0 {
            write!(w, "{}", self.numerator())
        } else if self.denominator().get() == 1 {
            write!(w, "({})", self.numerator())
        } else {
            write!(w, "({}/{})", self.numerator(), self.denominator().get())
        }
    }

    #[inline]
    pub fn format_plain_wrap(
        &self,
        w: &mut impl fmt::Write,
        (left, right): (&str, &str),
    ) -> fmt::Result {
        if self.denominator().get() == 1 && self.numerator() >= 0 {
            write!(w, "{}", self.numerator())
        } else if self.denominator().get() == 1 {
            write!(w, "{}{}{}", left, self.numerator(), right)
        } else {
            write!(
                w,
                "{}{}/{}{}",
                left,
                self.numerator(),
                self.denominator().get(),
                right
            )
        }
    }

    #[inline]
    pub fn format_latex(&self, w: &mut impl fmt::Write) -> fmt::Result {
        if self.denominator().get() == 1 {
            if self.numerator() >= 0 {
                write!(w, "{}", self.numerator())
            } else {
                write!(w, "{{{}}}", self.numerator())
            }
        } else {
            write!(
                w,
                r"\frac {{{}}}{{{}}}",
                self.numerator(),
                self.denominator().get()
            )
        }
    }
}

#[macro_export]
macro_rules! format_fraction {
    ($frac:expr) => {{
        const LEFT: i32 = $frac.numerator();
        const RIGHT: u32 = $frac.denominator().get();
        $crate::utils::formatcp!("{}/{}", LEFT, RIGHT)
    }};
}
