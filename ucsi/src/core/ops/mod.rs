//! # The `ops` module
//! 
//! The most basic and core feature of the module is the [`unit`](crate::unit) macro.
//! The macro offers a dsl to create unit declarations based on existing units.
//! 
//! ## Example
//! 
//! ```rust
//! use ucsi::unit;
//! use ucsi::Value;
//! // requires "alias_export" feature
//! // (included in "full" or default feature)
//! use ucsi::units::base::{kg, m, s};
//! 
//! // create a new type
//! type Newton = unit!((kg * m) / (s ** { 2 }));
//! // use this type
//! let force: Value<f64, Newton> = Value::new(42.0);
//! ```
//! 
//! ## Syntax
//! 
//! - `*`: Multiplication operator.
//!   Generated: `src::core::ops::Mul<T, U>`.
//! - `/`: Division operator.
//!   Generated: `src::core::ops::Div<T, U>`.
//! - `**`: Power operator.
//!   - `T ** { int32 }`: `src::core::ops::PowI<T, const int32>`
//!   - `T ** { int32 / uint32}`: `src::core::ops::PowFrac<T, const int32, const uint32>`
//! 
//! **Note:**
//! 
//! Consecutive operations are not currently supported,
//! so operations with more than two members need to be wrapped in parentheses.
//! 
//! ## Restrictions
//! 
//! All types involved in type operations must implement `SiOpsUnit`,
//! that means associated types cannot be used.

mod ops;

mod ops_creater;

pub use ops::*;
