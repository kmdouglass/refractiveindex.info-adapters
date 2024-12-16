//! RefractiveIndex.INFO Adapters
//!
//! An adapter for converting the RefractiveIndex.INFO database into a flat,
//! key-value store.
pub mod database;
mod internal;

pub use internal::store::Store;
