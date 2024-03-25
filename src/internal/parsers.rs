use anyhow::{anyhow, Result};

use crate::database::Material;

use super::store::Item;

pub(super) fn parse_material(
    material: Material,
    shelf: &str,
    book: &str,
    page: &str,
) -> Result<Item> {
    let data = material
        .data
        .into_iter()
        .map(|data| data.try_into())
        .collect::<Result<Vec<_>>>()?;
    Ok(Item {
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
