use core::marker::PhantomData;

use datastruct::DataStruct;

use crate::{core::format::UnitFormat, fraction::Fraction, units::any::SiOpsUnit};

#[derive(Default)]
pub struct SiFormatter<'a> {
    _p: PhantomData<&'a ()>,
}

impl<'a> SiFormatter<'a> {
    pub const fn new() -> Self {
        Self { _p: PhantomData }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DataStruct)]
#[dstruct(default, const, set)]
pub struct SiFormatterOption<'a> {
    #[dfield(default = r#"" + ""#)]
    pub unit_seperator: &'a str,
    #[dfield(default = r#"("", "")"#)]
    pub unit_symbol_wrapper: (&'a str, &'a str),
    #[dfield(default = r#""^""#)]
    pub exponent_seperator: &'a str,
    #[dfield(default = r#"("(", ")")"#)]
    pub exponent_symbol_wrapper: (&'a str, &'a str),
    #[dfield(default = "false")]
    pub show_all_exponent: bool,
    #[dfield(default = "true")]
    pub ignore_zero: bool,
}

impl<'a, U: SiOpsUnit> UnitFormat<U> for SiFormatter<'a> {
    type Option = SiFormatterOption<'a>;

    fn ufmt(op: Self::Option, w: &mut impl core::fmt::Write) -> Result<(), core::fmt::Error> {
        let mut it = U::UNIT_MAP
            .unit_fields()
            .into_iter()
            .filter(|(_, u)| !u.is_zero())
            .peekable();

        while let Some((n, u)) = it.next() {
            let pow = u.simplify();
            write!(
                w,
                "{}{}{}",
                op.unit_symbol_wrapper.0, n, op.unit_symbol_wrapper.1
            )?;

            if op.show_all_exponent || !pow.eq(&Fraction::ONE) {
                write!(w, "{}", op.exponent_seperator)?;
            }

            if !op.show_all_exponent && pow.eq(&Fraction::ONE) {
            } else {
                pow.format_plain_wrap(w, op.exponent_symbol_wrapper)?;
            }

            if it.peek().is_some() {
                write!(w, "{}", op.unit_seperator)?;
            }
        }

        Ok(())
    }
}
