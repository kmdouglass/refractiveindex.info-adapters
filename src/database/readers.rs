//! Reads the input database files from disk.
use anyhow::Error;

use crate::database::{BookContent, Catalog, RIInfoMaterial, ShelfContent};
use crate::database::parsers::parse_material;
use crate::internal::store::Store;

#[cfg(feature = "cli")]
pub(crate) fn read_material(path: std::path::PathBuf) -> Result<RIInfoMaterial, Error> {
    let text = std::fs::read_to_string(path)?;
    let material: RIInfoMaterial = serde_yaml::from_str(&text)?;
    Ok(material)
}

impl TryFrom<Catalog> for Store {
    type Error = anyhow::Error;

    /// Converts a RefractiveIndex.INFO catalog into a flat, key-value store of
    /// materials data.
    fn try_from(catalog: Catalog) -> Result<Self, Self::Error> {
        let mut store = Store::new();

        for shelf in catalog {
            let shelf_key = &shelf.shelf;
            let shelf_name = &shelf.name;

            for shelf_content in shelf.content {
                match shelf_content {
                    ShelfContent::Divider { divider: _ } => {
                        // store divider
                    }
                    ShelfContent::Book {
                        book,
                        name,
                        info: _,
                        content,
                    } => {
                        let book_key = &book;
                        let book_name = &name;

                        for book_content in content {
                            let (page, name, data, _info) = match book_content {
                                BookContent::Divider { divider: _ } => {
                                    // Skip the divider for now
                                    continue;
                                }
                                BookContent::Page {
                                    page,
                                    name,
                                    data,
                                    info,
                                } => (page, name, data, info),
                                BookContent::PageNumberName {
                                    page,
                                    name,
                                    data,
                                    info,
                                } => (page.to_string(), name, data, info),
                            };

                            let page_key = &page;
                            let page_name = &name;

                            // Try to read the material data; if it fails, skip this page
                            let material = match read_material(data) {
                                Ok(material) => material,
                                Err(_) => {
                                    continue;
                                }
                            };

                            // Parse the material data
                            let item =
                                match parse_material(material, shelf_name, book_name, page_name) {
                                    Ok(item) => item,
                                    Err(_) => {
                                        continue;
                                    }
                                };

                            let key = format!("{}:{}:{}", shelf_key, book_key, page_key);
                            store.insert(key, item);
                        }
                    }
                }
            }
        }

        Ok(store)
    }
}
