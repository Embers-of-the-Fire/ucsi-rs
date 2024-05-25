use core::{default, fmt};

use datastruct::{ConstDataStruct, DataStruct};

use crate::{core::format::UnitFormat, fraction::Fraction, units::any::SiOpsUnit};

pub struct SiLatex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, DataStruct)]
#[dstruct(default, const, set)]
pub struct SiLatexOption {
    #[dfield(default = "false")]
    pub show_all_exponent: bool,
    #[dfield(default = "true")]
    pub ignore_zero: bool,
    #[dfield(default = "false")]
    pub plain_fraction: bool,
}

impl default::Default for SiLatexOption {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl<U: SiOpsUnit> UnitFormat<U> for SiLatex {
    type Option = SiLatexOption;

    fn ufmt(op: Self::Option, w: &mut impl fmt::Write) -> Result<(), fmt::Error> {
        let mut it = U::UNIT_MAP
            .unit_fields()
            .into_iter()
            .filter(|(_, u)| !op.ignore_zero || !u.is_zero())
            .peekable();

        while let Some((n, u)) = it.next() {
            let pow = u.simplify();
            if op.show_all_exponent || !pow.eq(&Fraction::ONE) {
                write!(w, "{}^", n)?;
            } else {
                write!(w, "{}", n)?;
            }

            if !op.show_all_exponent && pow.eq(&Fraction::ONE) {
            } else if !op.plain_fraction {
                pow.format_latex(w)?;
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
