use cfg_if::cfg_if;

use super::{any::{SiOpsUnit, SiAnyUnit}, base::Kilogram};

pub struct SiAssociatedUnitDefinition {
    pub full_name: &'static str,
    pub short_name: &'static str,
    pub unit_symbol: &'static str,
}

pub trait SiAssociatedUnit: SiAnyUnit {
    type BaseUnit: SiOpsUnit;
    const DEF: SiAssociatedUnitDefinition;

    // const fn bound
    //  (suggested, since `SiAssociatedUnit` is not valid unit type currently):
    //
    //     const fn to_metric(self) -> T;
    //     const fn from_metric(val: T) -> Self;
}

pub mod weight;

mod __collect_impl {
    pub use super::weight::impl_aliases::*;
}

cfg_if! {
    if #[cfg(feature = "alias_hidden")] {
    } else if #[cfg(feature = "alias_mod")] {
        pub mod aliases {
            pub use super::__collect_impl::*;
        }
    } else if #[cfg(feature = "alias_export")] {
        pub use __collect_impl::*;
    }
}
