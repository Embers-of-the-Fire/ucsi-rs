use cfg_if::cfg_if;

use crate::{
    core::units::base::{Kilogram, Meter, Second},
    macros::unit_def::si_exported_unit_def,
    unit,
};

si_exported_unit_def! {
    alias pub(crate) mod impl_aliases;

    unit Newton based on unit!((Kilogram * Meter) / (Second ** { 2 })) {
        full_name: "Newton",
        short_name: "newton",
        unit_symbol: "N",
    } alias N;
}

cfg_if! {
    if #[cfg(feature = "alias_export")] {
        pub use impl_aliases::*;
    }
}
