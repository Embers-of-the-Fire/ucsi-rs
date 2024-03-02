#[cfg(feature = "const_soft_float")]
use const_soft_float::{soft_f32::SoftF32, soft_f64::SoftF64};

use crate::{
    core::units::base::Kilogram,
    macros::{conversion::unit_conversion, unit_def::si_associated_unit_def},
};

si_associated_unit_def! {
    alias pub(crate) mod impl_aliases;

    unit Gram
    based on Kilogram {
        full_name: "Gram",
        short_name: "gram",
        unit_symbol: "g",
    }
    alias g
    conversion {
        const (u16, u32, u64, u128, i16, i32, i64, i128, usize, isize) {
            to: |value| { value / 1000 },
            from: |value| { value * 1000 },
        };
        const (#[cfg(feature = "const_soft_float")] SoftF32) {
            to: |value| { value.div(SoftF32(1000.0))},
            from: |value| { value.mul(SoftF32(1000.0))},
        };
        const (#[cfg(feature = "const_soft_float")] SoftF64) {
            to: |value| { value.div(SoftF64(1000.0))},
            from: |value| { value.mul(SoftF64(1000.0))},
        };
        (f32, f64) {
            to: |value| { value / 1000.0 },
            from: |value| { value * 1000.0 }
        };
    };
}
