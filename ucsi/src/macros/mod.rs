//! The `macros` module.
//! 
//! This module contains internally used macros,
//! and not all of them are well-designed for external use.
//! 
//! ## Related features
//! 
//! - `internal_macros`: Enable all macros.
//!   (Disabled by default. Included in `full` feature.)
//! 
//! ### Sub-item features
//! 
//! - `typedef_macros`: Export unit definition macros. They are designed to use internally.
//! - `util_macros`: Export utility macros.

#![allow(unused_imports)]

mod impl_cast;
mod impl_const_unwrap;
mod impl_conversion;
mod impl_unit_def;

mod util_macro {
    pub mod cast {
        /// Cast a value to another unit.
        pub use crate::__impl_cast_si_value as cast_si_value;
        #[cfg(feature = "infer_cast")]
        /// Cast a value to another unit with TAIT inference.
        pub use crate::__impl_infer_cast_si_value as infer_cast_si_value;
    }

    pub mod unwrap {
        /// Constantly unwrap an option.
        pub use crate::__impl_unwrap_option_const as unwrap_option_const;
    }
}

#[cfg(feature = "util_macros")]
pub use util_macro::*;
#[cfg(not(feature = "util_macros"))]
pub(crate) use util_macro::*;

mod util_def {
    pub mod unit_def {
        /// Generate associated type definition.
        pub use crate::__impl_si_associated_unit_def as si_associated_unit_def;
        /// Generate exported type definition.
        pub use crate::__impl_si_exported_unit_def as si_exported_unit_def;
    }

    pub mod conversion {
        /// Generate associated type conversion definition.
        pub use crate::__impl_unit_conversion as unit_conversion;
    }
}

#[cfg(feature = "typedef_macros")]
pub use util_def::*;
#[cfg(not(feature = "typedef_macros"))]
pub(crate) use util_def::*;
