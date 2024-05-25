use core::fmt;

use datastruct::DataStruct;

use crate::{core::format::UnitFormat, fraction::Fraction, units::any::SiOpsUnit};

pub struct SiDefault;

#[derive(Debug, Clone, Copy, PartialEq, Eq, DataStruct)]
#[dstruct(default, const, set)]
pub struct SiDefaultOption {
    #[dfield(default = "false")]
    pub show_all_exponent: bool,
    #[dfield(default = "true")]
    pub ignore_zero: bool,
}

impl<U: SiOpsUnit> UnitFormat<U> for SiDefault {
    type Option = SiDefaultOption;

    fn ufmt(op: Self::Option, w: &mut impl fmt::Write) -> Result<(), fmt::Error> {
        let mut it = U::UNIT_MAP
            .unit_fields()
            .into_iter()
            .filter(|(_, u)| !u.is_zero())
            .peekable();

        while let Some((n, u)) = it.next() {
            let pow = u.simplify();
            if op.show_all_exponent || !pow.eq(&Fraction::ONE) {
                write!(w, "{}^", n)?;
            } else {
                write!(w, "{}", n)?;
            }

            if !op.show_all_exponent && pow.eq(&Fraction::ONE) {
            } else {
                pow.format_plain(w)?;
            }

            if it.peek().is_some() {
                write!(w, " + ")?;
            }
        }

        Ok(())
    }
}
