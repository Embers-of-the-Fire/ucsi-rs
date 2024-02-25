use cfg_if::cfg_if;

use crate::{macros::unit_def::si_exported_unit_def, unit};

use super::{
    any::{SiAnyUnit, SiDefinedUnit, SiOpsUnit},
    base::{Kilogram, Meter, Second},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SiExportedUnitDefinition {
    pub full_name: &'static str,
    pub short_name: &'static str,
    pub unit_symbol: &'static str,
}

pub trait SiExportedUnit: SiDefinedUnit + SiAnyUnit {
    type BaseUnit: SiOpsUnit;
    const DEF: SiExportedUnitDefinition;
}

pub mod force;

cfg_if! {
    if #[cfg(feature = "alias_hidden")] {
    } else if #[cfg(feature = "alias_mod")] {
        pub mod aliases {
            pub use super::__collect_impl::*;
        }
    }
}
