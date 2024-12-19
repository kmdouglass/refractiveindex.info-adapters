//! RefractiveIndex.INFO Adapters
//!
//! An adapter for converting the RefractiveIndex.INFO database into a flat,
//! key-value store.

#[cfg(feature = "cli")]
pub mod database;
mod internal;

pub use internal::store::{DispersionData, Material, Store};
