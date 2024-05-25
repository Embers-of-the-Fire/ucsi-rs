use core::fmt;

use crate::{SiAnyUnit, Value};

extern crate alloc;
use alloc::string::String;

pub mod ufmt;

pub trait UnitFormat<Unit: SiAnyUnit> {
    type Option;

    fn ufmt(op: Self::Option, w: &mut impl fmt::Write) -> Result<(), fmt::Error>;
}

impl<V, T: SiAnyUnit> Value<V, T> {
    pub fn fmt_unit<Fmt: UnitFormat<T>>(&self, op: Fmt::Option) -> String {
        let mut string = String::new();
        Fmt::ufmt(op, &mut string).unwrap();
        string
    }
}

pub trait ValueFormat<Unit: SiAnyUnit> {
    type Option;

    fn vfmt(op: Self::Option, w: &mut impl fmt::Write) -> Result<(), fmt::Error>;
}
