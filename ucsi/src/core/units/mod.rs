//! # The `units` module
//! 
//! This module contains SI base units and built-in exported and associated units.
//! 
//! ## Related features
//! 
//! ### Standard units
//! 
//! - `exported_types`:
//!   All exported units will be available if this feature is enabled.
//!   (Enabled by default.)
//! - `associated_types`:
//!   All associated units will be available if this feature is enabled.
//!   (Disabled by default.)
//! - `all_types`:
//!   Enable `exported_types` and `associated_types`.
//!   (Disabled by default. Included in the `full` feature.)
//! 
//! ### Unit alias
//! 
//! - `alias_export`:
//!   All aliases are exported directly in the corresponding module.
//!   (Enabled by default.)\
//!   e.g. `crate::core::units::base::kg`(Kilogram),
//!        `crate::core::units::exported::N`(Newton).
//! - `alias_mod`:
//!   All aliases are hidden behind a single module inside the corresponding module.\
//!   e.g. `crate::core::units::base::aliases::kg`,
//!        `crate::core::units::exported::aliases::N`.
//! - `alias_hidden`:
//!   Hide all aliases and do not export them.

#![allow(dead_code, unused_imports)]

/// SI base units.
pub mod base;

#[cfg(feature = "exported_types")]
/// SI exported units.
pub mod exported;
#[cfg(not(feature = "exported_types"))]
pub(crate) mod exported;

#[cfg(feature = "associated_types")]
/// Associated units, including non-si units.
pub mod associated;
#[cfg(not(feature = "associated_types"))]
pub(crate) mod associated;

/// Primitive configuration and traits of units.
pub mod any;
