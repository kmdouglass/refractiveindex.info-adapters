//! Reads the input database files from disk.
use anyhow::Error;

use crate::database::Material;

pub(crate) fn read_material(path: std::path::PathBuf) -> Result<Material, Error> {
    let text = std::fs::read_to_string(path)?;
    let material: Material = serde_yaml::from_str(&text)?;
    Ok(material)
}
