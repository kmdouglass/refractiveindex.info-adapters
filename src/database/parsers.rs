use anyhow::{anyhow, Result};

use crate::database::{Data, RIInfoMaterial};
use crate::{DispersionData, Material};

pub(super) fn parse_material(
    material: RIInfoMaterial,
    shelf: &str,
    book: &str,
    page: &str,
) -> Result<Material> {
    // TODO: Ignore any errors and continue parsing;
    // TODO: Log the errors
    let data = material
        .data
        .into_iter()
        .map(|data| data.try_into())
        .collect::<Result<Vec<_>>>()?;
    Ok(Material {
        shelf: shelf.to_string(),
        book: book.to_string(),
        page: page.to_string(),
        references: material.references,
        comments: material.comments,
        data,
    })
}

pub(super) fn parse_coefficients(data: &str) -> Result<Vec<f64>> {
    data.split_whitespace()
        .map(|s| s.parse::<f64>().map_err(|e| e.into()))
        .collect()
}

pub(super) fn parse_wavelength_range(data: &str) -> Result<[f64; 2]> {
    let mut iter = data.split_whitespace();
    let start = iter
        .next()
        .ok_or(anyhow!("Cannot find minimum value"))?
        .parse()?;
    let end = iter
        .next()
        .ok_or(anyhow!("Cannot find maximum value"))?
        .parse()?;
    Ok([start, end])
}

pub(super) fn parse_tabulated_2d(data: &str) -> Result<Vec<[f64; 2]>> {
    data.lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let wavelength = iter
                .next()
                .ok_or(anyhow!("Cannot find wavelength"))?
                .parse()?;
            let value = iter
                .next()
                .ok_or(anyhow!("Cannot find refractive index value"))?
                .parse()?;
            Ok([wavelength, value])
        })
        .collect()
}

pub(super) fn parse_tabulated_3d(data: &str) -> Result<Vec<[f64; 3]>> {
    data.lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let wavelength = iter
                .next()
                .ok_or(anyhow!("Cannot find wavelength"))?
                .parse()?;
            let n = iter
                .next()
                .ok_or(anyhow!("Cannot find real refractive index value"))?
                .parse()?;
            let k = iter
                .next()
                .ok_or(anyhow!("Cannot find imaginary refractive index value"))?
                .parse()?;
            Ok([wavelength, n, k])
        })
        .collect()
}

impl TryFrom<Data> for DispersionData {
    type Error = anyhow::Error;

    fn try_from(data: Data) -> Result<Self, Self::Error> {
        match data {
            Data::TabulatedK { data } => {
                let data = parse_tabulated_2d(&data)?;
                Ok(DispersionData::TabulatedK { data })
            }
            Data::TabulatedN { data } => {
                let data = parse_tabulated_2d(&data)?;
                Ok(DispersionData::TabulatedN { data })
            }
            Data::TabulatedNK { data } => {
                let data = parse_tabulated_3d(&data)?;
                Ok(DispersionData::TabulatedNK { data })
            }
            Data::Formula1 {
                wavelength_range,
                coefficients,
            } => {
                let wavelength_range = parse_wavelength_range(&wavelength_range)?;
                let coefficients = parse_coefficients(&coefficients)?;
                Ok(DispersionData::Formula1 {
                    wavelength_range,
                    c: coefficients,
                })
            }
            Data::Formula2 {
                wavelength_range,
                coefficients,
            } => {
                let wavelength_range = parse_wavelength_range(&wavelength_range)?;
                let coefficients = parse_coefficients(&coefficients)?;
                Ok(DispersionData::Formula2 {
                    wavelength_range,
                    c: coefficients,
                })
            }
            Data::Formula3 {
                wavelength_range,
                coefficients,
            } => {
                let wavelength_range = parse_wavelength_range(&wavelength_range)?;
                let coefficients = parse_coefficients(&coefficients)?;
                Ok(DispersionData::Formula3 {
                    wavelength_range,
                    c: coefficients,
                })
            }
            Data::Formula4 {
                wavelength_range,
                coefficients,
            } => {
                let wavelength_range = parse_wavelength_range(&wavelength_range)?;
                let coefficients = parse_coefficients(&coefficients)?;
                Ok(DispersionData::Formula4 {
                    wavelength_range,
                    c: coefficients,
                })
            }
            Data::Formula5 {
                wavelength_range,
                coefficients,
            } => {
                let wavelength_range = parse_wavelength_range(&wavelength_range)?;
                let coefficients = parse_coefficients(&coefficients)?;
                Ok(DispersionData::Formula5 {
                    wavelength_range,
                    c: coefficients,
                })
            }
            Data::Formula6 {
                wavelength_range,
                coefficients,
            } => {
                let wavelength_range = parse_wavelength_range(&wavelength_range)?;
                let coefficients = parse_coefficients(&coefficients)?;
                Ok(DispersionData::Formula6 {
                    wavelength_range,
                    c: coefficients,
                })
            }
            Data::Formula7 {
                wavelength_range,
                coefficients,
            } => {
                let wavelength_range = parse_wavelength_range(&wavelength_range)?;
                let coefficients = parse_coefficients(&coefficients)?;
                Ok(DispersionData::Formula7 {
                    wavelength_range,
                    c: coefficients,
                })
            }
            Data::Formula8 {
                wavelength_range,
                coefficients,
            } => {
                let wavelength_range = parse_wavelength_range(&wavelength_range)?;
                let coefficients = parse_coefficients(&coefficients)?;
                Ok(DispersionData::Formula8 {
                    wavelength_range,
                    c: coefficients,
                })
            }
            Data::Formula9 {
                wavelength_range,
                coefficients,
            } => {
                let wavelength_range = parse_wavelength_range(&wavelength_range)?;
                let coefficients = parse_coefficients(&coefficients)?;
                Ok(DispersionData::Formula9 {
                    wavelength_range,
                    c: coefficients,
                })
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_coefficients() {
        let data = "  1.0  2.0  3.0  ";
        let result = parse_coefficients(data).unwrap();
        assert_eq!(result, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_parse_wavelength_range() {
        let data = "  1.0  2.0\n  ";
        let result = parse_wavelength_range(data).unwrap();
        assert_eq!(result, [1.0, 2.0]);
    }

    #[test]
    fn test_parse_tabulated_2d() {
        let data = "1.0 2.0\n3.0 4.0\n";
        let result = parse_tabulated_2d(data).unwrap();
        assert_eq!(result, [[1.0, 2.0], [3.0, 4.0]]);
    }

    #[test]
    fn test_parse_tabulated_3d() {
        let data = "1.0 2.0 3.0\n4.0 5.0 6.0\n";
        let result = parse_tabulated_3d(data).unwrap();
        assert_eq!(result, [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
    }
}
