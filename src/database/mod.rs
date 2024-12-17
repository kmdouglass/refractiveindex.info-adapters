//! A strongly-typed model of the RefractiveIndex.INFO database.
mod catalog;
mod material;
mod readers;

pub use catalog::*;
pub use material::*;

pub(crate) use readers::*;
