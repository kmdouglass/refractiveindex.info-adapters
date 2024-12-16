use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::database::Data as UnparsedData;
use crate::database::{BookContent, Catalog, ShelfContent};

use super::parsers::{
    parse_coefficients, parse_material, parse_tabulated_2d, parse_tabulated_3d,
    parse_wavelength_range,
};
use super::readers::read_material;

/// A flat, key-value store for material refractive index data.
#[derive(Serialize, Deserialize, Debug)]
pub struct Store {
    inner: HashMap<String, Item>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(super) struct Item {
    pub shelf: String,
    pub book: String,
    pub page: String,
    pub comments: String,
    pub references: String,
    pub data: Vec<Data>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(super) enum Data {
    TabulatedK {
        data: Vec<[f64; 2]>,
    },
    TabulatedN {
        data: Vec<[f64; 2]>,
    },
    TabulatedNK {
        data: Vec<[f64; 3]>,
    },
    Formula1 {
        wavelength_range: [f64; 2],
        coefficients: Vec<f64>,
    },
    Formula2 {
        wavelength_range: [f64; 2],
        coefficients: Vec<f64>,
    },
    Formula3 {
        wavelength_range: [f64; 2],
        coefficients: Vec<f64>,
    },
    Formula4 {
        wavelength_range: [f64; 2],
        coefficients: Vec<f64>,
    },
    Formula5 {
        wavelength_range: [f64; 2],
        coefficients: Vec<f64>,
    },
    Formula6 {
        wavelength_range: [f64; 2],
        coefficients: Vec<f64>,
    },
    Formula7 {
        wavelength_range: [f64; 2],
        coefficients: Vec<f64>,
    },
    Formula8 {
        wavelength_range: [f64; 2],
        coefficients: Vec<f64>,
    },
    Formula9 {
        wavelength_range: [f64; 2],
        coefficients: Vec<f64>,
    },
}

impl Store {
    fn new() -> Self {
        Store {
            inner: HashMap::new(),
        }
    }
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
                            store.inner.insert(key, item);
                        }
                    }
                }
            }
        }

        Ok(store)
    }
}

impl TryFrom<UnparsedData> for Data {
    type Error = anyhow::Error;

    fn try_from(data: UnparsedData) -> Result<Self, Self::Error> {
        match data {
            UnparsedData::TabulatedK { data } => {
                let data = parse_tabulated_2d(&data)?;
                Ok(Data::TabulatedK { data })
            }
            UnparsedData::TabulatedN { data } => {
                let data = parse_tabulated_2d(&data)?;
                Ok(Data::TabulatedN { data })
            }
            UnparsedData::TabulatedNK { data } => {
                let data = parse_tabulated_3d(&data)?;
                Ok(Data::TabulatedNK { data })
            }
            UnparsedData::Formula1 {
                wavelength_range,
                coefficients,
            } => {
                let wavelength_range = parse_wavelength_range(&wavelength_range)?;
                let coefficients = parse_coefficients(&coefficients)?;
                Ok(Data::Formula1 {
                    wavelength_range,
                    coefficients,
                })
            }
            UnparsedData::Formula2 {
                wavelength_range,
                coefficients,
            } => {
                let wavelength_range = parse_wavelength_range(&wavelength_range)?;
                let coefficients = parse_coefficients(&coefficients)?;
                Ok(Data::Formula2 {
                    wavelength_range,
                    coefficients,
                })
            }
            UnparsedData::Formula3 {
                wavelength_range,
                coefficients,
            } => {
                let wavelength_range = parse_wavelength_range(&wavelength_range)?;
                let coefficients = parse_coefficients(&coefficients)?;
                Ok(Data::Formula3 {
                    wavelength_range,
                    coefficients,
                })
            }
            UnparsedData::Formula4 {
                wavelength_range,
                coefficients,
            } => {
                let wavelength_range = parse_wavelength_range(&wavelength_range)?;
                let coefficients = parse_coefficients(&coefficients)?;
                Ok(Data::Formula4 {
                    wavelength_range,
                    coefficients,
                })
            }
            UnparsedData::Formula5 {
                wavelength_range,
                coefficients,
            } => {
                let wavelength_range = parse_wavelength_range(&wavelength_range)?;
                let coefficients = parse_coefficients(&coefficients)?;
                Ok(Data::Formula5 {
                    wavelength_range,
                    coefficients,
                })
            }
            UnparsedData::Formula6 {
                wavelength_range,
                coefficients,
            } => {
                let wavelength_range = parse_wavelength_range(&wavelength_range)?;
                let coefficients = parse_coefficients(&coefficients)?;
                Ok(Data::Formula6 {
                    wavelength_range,
                    coefficients,
                })
            }
            UnparsedData::Formula7 {
                wavelength_range,
                coefficients,
            } => {
                let wavelength_range = parse_wavelength_range(&wavelength_range)?;
                let coefficients = parse_coefficients(&coefficients)?;
                Ok(Data::Formula7 {
                    wavelength_range,
                    coefficients,
                })
            }
            UnparsedData::Formula8 {
                wavelength_range,
                coefficients,
            } => {
                let wavelength_range = parse_wavelength_range(&wavelength_range)?;
                let coefficients = parse_coefficients(&coefficients)?;
                Ok(Data::Formula8 {
                    wavelength_range,
                    coefficients,
                })
            }
            UnparsedData::Formula9 {
                wavelength_range,
                coefficients,
            } => {
                let wavelength_range = parse_wavelength_range(&wavelength_range)?;
                let coefficients = parse_coefficients(&coefficients)?;
                Ok(Data::Formula9 {
                    wavelength_range,
                    coefficients,
                })
            }
        }
    }
}
