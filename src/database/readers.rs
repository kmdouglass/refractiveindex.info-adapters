//! Reads the input database files from disk.
use anyhow::Error;

use crate::database::parsers::parse_material;
use crate::database::{BookContent, Catalog, CatalogEntry, RIInfoMaterial, ShelfContent};
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
        let mut store = Store::default();

        for entry in catalog {
            let shelf = match entry {
                CatalogEntry::Divider { .. } => continue,
                CatalogEntry::Shelf(shelf) => shelf,
            };

            let shelf_key = &shelf.shelf;
            let shelf_name = &shelf.name;
            let mut current_shelf_divider: Option<String> = None;

            for shelf_content in shelf.content {
                match shelf_content {
                    ShelfContent::Divider { divider } => {
                        current_shelf_divider = Some(divider);
                    }
                    ShelfContent::Book {
                        book,
                        name,
                        info: _,
                        content,
                    } => {
                        let book_key = &book;
                        let book_name = &name;
                        let mut current_book_divider: Option<String> = None;

                        for book_content in content {
                            let (page, name, data, _info) = match book_content {
                                BookContent::Divider { divider } => {
                                    current_book_divider = Some(divider);
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
                            let item = match parse_material(
                                material,
                                shelf_name,
                                book_name,
                                page_name,
                                current_shelf_divider.clone(),
                                current_book_divider.clone(),
                            ) {
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

#[cfg(feature = "cli")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::{BookContent, Catalog, CatalogEntry, Shelf, ShelfContent};
    use crate::Store;
    use std::path::PathBuf;

    fn johnson_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("refractiveindex.info-database/database/data/main/Ag/nk/Johnson.yml")
    }

    fn minimal_catalog(data: PathBuf) -> Catalog {
        vec![CatalogEntry::Shelf(Shelf {
            shelf: "main".to_string(),
            name: "MAIN - simple inorganic materials".to_string(),
            info: None,
            content: vec![ShelfContent::Book {
                book: "Ag".to_string(),
                name: "Ag (Silver)".to_string(),
                info: None,
                content: vec![BookContent::Page {
                    page: "Johnson".to_string(),
                    name: "Johnson and Christy 1972: n,k 0.188-1.94 um".to_string(),
                    data,
                    info: None,
                }],
            }],
        })]
    }

    #[test]
    fn test_catalog_to_store_inserts_material() {
        let store = Store::try_from(minimal_catalog(johnson_path())).unwrap();
        let material = store.get("main:Ag:Johnson");
        assert!(material.is_some());
        let m = material.unwrap();
        assert_eq!(m.shelf, "MAIN - simple inorganic materials");
        assert_eq!(m.book, "Ag (Silver)");
        assert_eq!(m.page, "Johnson and Christy 1972: n,k 0.188-1.94 um");
    }

    #[test]
    fn test_catalog_to_store_tracks_dividers() {
        let catalog = vec![CatalogEntry::Shelf(Shelf {
            shelf: "main".to_string(),
            name: "MAIN - simple inorganic materials".to_string(),
            info: None,
            content: vec![
                ShelfContent::Divider {
                    divider: "Ag - Silver".to_string(),
                },
                ShelfContent::Book {
                    book: "Ag".to_string(),
                    name: "Ag (Silver)".to_string(),
                    info: None,
                    content: vec![
                        BookContent::Divider {
                            divider: "Experimental data: bulk, thick film".to_string(),
                        },
                        BookContent::Page {
                            page: "Johnson".to_string(),
                            name: "Johnson and Christy 1972: n,k 0.188-1.94 um".to_string(),
                            data: johnson_path(),
                            info: None,
                        },
                    ],
                },
            ],
        })];

        let store = Store::try_from(catalog).unwrap();
        let m = store.get("main:Ag:Johnson").unwrap();
        assert_eq!(m.shelf_divider, Some("Ag - Silver".to_string()));
        assert_eq!(
            m.book_divider,
            Some("Experimental data: bulk, thick film".to_string())
        );
    }

    #[test]
    fn test_catalog_to_store_skips_invalid_page() {
        let catalog = vec![CatalogEntry::Shelf(Shelf {
            shelf: "main".to_string(),
            name: "MAIN - simple inorganic materials".to_string(),
            info: None,
            content: vec![ShelfContent::Book {
                book: "Ag".to_string(),
                name: "Ag (Silver)".to_string(),
                info: None,
                content: vec![
                    BookContent::Page {
                        page: "Johnson".to_string(),
                        name: "Johnson and Christy 1972: n,k 0.188-1.94 um".to_string(),
                        data: johnson_path(),
                        info: None,
                    },
                    BookContent::Page {
                        page: "Invalid".to_string(),
                        name: "Invalid entry".to_string(),
                        data: PathBuf::from("/nonexistent/path.yml"),
                        info: None,
                    },
                ],
            }],
        })];

        let store = Store::try_from(catalog).unwrap();
        assert!(store.get("main:Ag:Johnson").is_some());
        assert!(store.get("main:Ag:Invalid").is_none());
    }

    #[test]
    fn test_catalog_to_store_skips_top_level_divider() {
        let catalog = vec![
            CatalogEntry::Divider {
                divider: "Research data".to_string(),
            },
            CatalogEntry::Shelf(Shelf {
                shelf: "main".to_string(),
                name: "MAIN - simple inorganic materials".to_string(),
                info: None,
                content: vec![ShelfContent::Book {
                    book: "Ag".to_string(),
                    name: "Ag (Silver)".to_string(),
                    info: None,
                    content: vec![BookContent::Page {
                        page: "Johnson".to_string(),
                        name: "Johnson and Christy 1972: n,k 0.188-1.94 um".to_string(),
                        data: johnson_path(),
                        info: None,
                    }],
                }],
            }),
        ];

        let store = Store::try_from(catalog).unwrap();
        assert!(store.get("main:Ag:Johnson").is_some());
    }
}
