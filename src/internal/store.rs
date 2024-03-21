use std::collections::HashMap;

use crate::database::Data as UnparsedData;
use crate::database::{Catalog, Material};

use super::parsers::{
    parse_coefficients, parse_tabulated_2d, parse_tabulated_3d, parse_wavelength_range,
};

struct Store {
    inner: HashMap<String, Item>,
}

struct Item {
    shelf: String,
    book: String,
    page: String,
    comments: String,
    references: String,
    data: Data,
}

enum Data {
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

impl From<Catalog> for Store {
    fn from(catalog: Catalog) -> Self {
        let mut store = Store::new();

        unimplemented!();

        store
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
