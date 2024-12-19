use std::collections::HashMap;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

/// A flat, key-value store for material refractive index data.
#[derive(Serialize, Deserialize, Debug)]
pub struct Store {
    inner: HashMap<String, Material>,
}

/// A single item in the store containing materials data.
#[derive(Serialize, Deserialize, Debug)]
pub struct Material {
    pub shelf: String,
    pub book: String,
    pub page: String,
    pub comments: String,
    pub references: String,
    pub data: Vec<DispersionData>,
}

#[derive(Debug)]
enum DataType {
    Real,
    Imaginary,
    Both,
}

/// The refractive index data associated with a material.
#[derive(Serialize, Deserialize, Debug)]
pub enum DispersionData {
    TabulatedK {
        data: Vec<[f64; 2]>,
    },
    TabulatedN {
        data: Vec<[f64; 2]>,
    },
    TabulatedNK {
        data: Vec<[f64; 3]>,
    },

    /// The Sellmeier formula.
    Formula1 {
        wavelength_range: [f64; 2],
        c: Vec<f64>,
    },

    /// The Sellmeier-2 formula.
    Formula2 {
        wavelength_range: [f64; 2],
        c: Vec<f64>,
    },

    /// Polynomial
    Formula3 {
        wavelength_range: [f64; 2],
        c: Vec<f64>,
    },

    /// RefractiveIndex.INFO
    Formula4 {
        wavelength_range: [f64; 2],
        c: Vec<f64>,
    },

    /// Cauchy
    Formula5 {
        wavelength_range: [f64; 2],
        c: Vec<f64>,
    },

    /// Gases
    Formula6 {
        wavelength_range: [f64; 2],
        c: Vec<f64>,
    },

    /// Herzberger
    Formula7 {
        wavelength_range: [f64; 2],
        c: Vec<f64>,
    },

    /// Retro
    Formula8 {
        wavelength_range: [f64; 2],
        c: Vec<f64>,
    },

    /// Exotic
    Formula9 {
        wavelength_range: [f64; 2],
        c: Vec<f64>,
    },
}

impl Store {
    pub fn new() -> Self {
        Store {
            inner: HashMap::new(),
        }
    }

    /// Returns the item from the store associated with the given key.
    ///
    /// # Arguments
    /// - `key`: The key to look up in the store.
    ///
    /// # Returns
    /// The item associated with the given key, if it exists.
    pub fn get(&self, key: &str) -> Option<&Material> {
        self.inner.get(key)
    }

    /// Inserts a new item into the store.
    /// 
    /// # Arguments
    /// - `key`: The key to associate with the item.
    /// - `material`: The item to insert into the store.
    pub fn insert(&mut self, key: String, material: Material) {
        self.inner.insert(key, material);
    }

    /// Returns an iterator over the keys in the store.
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.inner.keys()
    }

    /// Removes the item associated with the given key from the store.
    pub fn remove(&mut self, key: &str) -> Option<Material>{
        self.inner.remove(key)
    }
}

impl Material {
    /// Computes the real part of the refractive index of the material at the
    /// given wavelength.
    ///
    /// # Arguments
    /// - `wavelength`: The wavelength at which to evaluate the refractive
    ///   index.
    ///
    /// # Returns
    /// The real part of the refractive index of the material at the given
    /// wavelength.
    ///
    /// # Errors
    /// - If no real data is found for the item.
    /// - If the wavelength is outside the range of the real data.
    pub fn n(&self, wavelength: f64) -> Result<f64> {
        let data = self
            .data
            .iter()
            .find(|d| matches!(d.data_type(), DataType::Real | DataType::Both));
        let (n, _) = match data {
            Some(data) => data.interpolate(wavelength)?,
            None => return Err(anyhow!("No real data found for item.")),
        };
        Ok(n)
    }

    /// Computes the imaginary part of the refractive index of the material at
    /// the given wavelength.
    ///
    /// # Arguments
    /// - `wavelength`: The wavelength at which to evaluate the refractive
    ///  index.
    ///
    /// # Returns
    /// The imaginary part of the refractive index of the material at the given
    /// wavelength.
    ///
    /// # Errors
    /// - If no imaginary data is found for the item.
    /// - If the wavelength is outside the range of the imaginary data.
    pub fn k(&self, wavelength: f64) -> Result<f64> {
        let data = self
            .data
            .iter()
            .find(|d| matches!(d.data_type(), DataType::Imaginary | DataType::Both));
        let (_, k) = match data {
            Some(data) => data.interpolate(wavelength)?,
            None => return Err(anyhow!("No imaginary data found for item.")),
        };

        match k {
            Some(k) => Ok(k),
            None => Err(anyhow!("No imaginary data found for item.")),
        }
    }
}

impl DispersionData {
    /// Computes the value of the dispersion curve at the given wavelength.
    ///
    /// # Arguments
    /// - `wavelength`: The wavelength at which to evaluate the dispersion
    ///   curve.
    ///
    /// # Returns
    /// The value of the dispersion curve at the given wavelength. The first
    /// value is the real part of the refractive index, and the second value
    /// is the imaginary part of the refractive index.
    ///
    /// # Errors
    /// - If the wavelength is outside the range of dispersion data.
    pub fn interpolate(&self, wavelength: f64) -> Result<(f64, Option<f64>)> {
        let n: f64 = match &self {
            Self::Formula1 {
                wavelength_range,
                c,
            } => {
                // Sellmeier (preferred)
                if wavelength < wavelength_range[0] || wavelength > wavelength_range[1] {
                    return Err(anyhow!(
                        "The wavelength is outside the range of the real spec."
                    ));
                }
                let mut sum = 0.0;
                for i in (1..c.len()).step_by(2) {
                    sum += c[i] * wavelength.powi(2) / (wavelength.powi(2) - c[i + 1].powi(2));
                }
                (1.0 + c[0] + sum).sqrt()
            }
            Self::Formula2 {
                wavelength_range,
                c,
            } => {
                // Sellmeier-2
                if wavelength < wavelength_range[0] || wavelength > wavelength_range[1] {
                    return Err(anyhow!(
                        "The wavelength is outside the range of the real spec."
                    ));
                }
                let mut sum = 0.0;
                for i in (1..c.len()).step_by(2) {
                    sum += c[i] * wavelength.powi(2) / (wavelength.powi(2) - c[i + 1]);
                }
                (1.0 + c[0] + sum).sqrt()
            }
            Self::Formula3 {
                wavelength_range,
                c,
            } => {
                // Polynomial
                if wavelength < wavelength_range[0] || wavelength > wavelength_range[1] {
                    return Err(anyhow!(
                        "The wavelength is outside the range of the real spec."
                    ));
                }

                let mut sum = 0.0;
                for i in (1..c.len()).step_by(2) {
                    sum += c[i] * wavelength.powf(c[i + 1]);
                }
                (c[0] + sum).sqrt()
            }
            Self::Formula4 {
                wavelength_range,
                c,
            } => {
                // RefractiveIndex.INFO
                if wavelength < wavelength_range[0] || wavelength > wavelength_range[1] {
                    return Err(anyhow!(
                        "The wavelength is outside the range of the real spec."
                    ));
                }

                let mut sum = 0.0;
                for i in (1..c.len()).step_by(4) {
                    // Formula 4 is kind of wild.
                    if i <= 9 {
                        sum += c[i] * wavelength.powf(c[i + 1])
                            / (wavelength.powi(2) - c[i + 2].powf(c[i + 3]));
                    } else {
                        sum += c[i] * wavelength.powf(c[i + 1]);
                    }
                }
                (c[0] + sum).sqrt()
            }
            Self::Formula5 {
                wavelength_range,
                c,
            } => {
                // Cauchy
                if wavelength < wavelength_range[0] || wavelength > wavelength_range[1] {
                    return Err(anyhow!(
                        "The wavelength is outside the range of the real spec."
                    ));
                }

                let mut sum = 0.0;
                for i in (1..c.len()).step_by(2) {
                    sum += c[i] * wavelength.powf(c[i + 1]);
                }
                c[0] + sum
            }
            Self::Formula6 {
                wavelength_range,
                c,
            } => {
                // Gases
                if wavelength < wavelength_range[0] || wavelength > wavelength_range[1] {
                    return Err(anyhow!(
                        "The wavelength is outside the range of the real spec."
                    ));
                }

                let mut sum = 0.0;
                for i in (1..c.len()).step_by(2) {
                    sum += c[i] / (c[i + 1] - wavelength.powi(-2));
                }
                1.0 + c[0] + sum
            }
            Self::Formula7 {
                wavelength_range,
                c,
            } => {
                // Herzberger
                if wavelength < wavelength_range[0] || wavelength > wavelength_range[1] {
                    return Err(anyhow!(
                        "The wavelength is outside the range of the real spec."
                    ));
                }
                let mut sum = 0.0;
                for i in (3..c.len()).step_by(2) {
                    sum += c[i] * wavelength.powi(i as i32 - 1);
                }
                c[0] + c[1] / (wavelength.powi(2) - 0.028)
                    + c[2] / (wavelength.powi(2) - 0.028).powi(2)
                    + sum
            }
            Self::Formula8 {
                wavelength_range,
                c,
            } => {
                // Retro
                if wavelength < wavelength_range[0] || wavelength > wavelength_range[1] {
                    return Err(anyhow!(
                        "The wavelength is outside the range of the real spec."
                    ));
                }

                let sum = c[0]
                    + c[1] * wavelength.powi(2) / (wavelength.powi(2) - c[2])
                    + c[3] * wavelength.powi(2);
                println!("sum: {}", sum);
                ((2.0 * sum + 1.0) / (1.0 - sum)).sqrt()
            }
            Self::Formula9 {
                wavelength_range,
                c,
            } => {
                // Exotic
                if wavelength < wavelength_range[0] || wavelength > wavelength_range[1] {
                    return Err(anyhow!(
                        "The wavelength is outside the range of the real spec."
                    ));
                }

                (c[0]
                    + c[1] / (wavelength.powi(2) - c[2])
                    + c[3] * (wavelength - c[4]) / ((wavelength - c[4]).powi(2) + c[5]))
                    .sqrt()
            }
            _ => {
                return Err(anyhow!("Tabulated dispersion data are not implemented."));
            }
        };

        Ok((n, None))
    }

    /// Returns the type of data stored in the DispersionData.
    ///
    /// An item may have either one or two dispersion data sets. If there are
    /// two, we have to infer from the type which is real and which is
    /// imaginary.  We use the following rules to determine which is which:
    ///
    /// 1. If the DispersionData is TabulatedN, then it's the real part.
    /// 2. If the DispersionData is TabulatedK, then it's the imaginary part.
    /// 3. If the DispersionData is a formula, then it's the real part.
    ///
    /// If there is only one dispersion data set, then we use one additional
    /// rule:
    ///
    /// 1. If the DispersionData is TabulatedNK, then it's both the real and
    ///    imaginary parts.
    fn data_type(&self) -> DataType {
        match self {
            self::DispersionData::TabulatedK { data: _ } => DataType::Imaginary,
            self::DispersionData::TabulatedN { data: _ } => DataType::Real,
            self::DispersionData::TabulatedNK { data: _ } => DataType::Both,
            _ => DataType::Real,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_interpolate_formula_1() {
        // Water Ice at 150 K from refractiveindex.info
        let data = DispersionData::Formula1 {
            wavelength_range: [0.210, 0.757],
            c: vec![0.0, 0.496, 0.071, 0.190, 0.134],
        };
        let (n, k) = data.interpolate(0.5876).unwrap();
        assert_abs_diff_eq!(n, 1.3053, epsilon = 1e-4);
        assert!(k.is_none());
    }

    #[test]
    fn test_interpolate_formula_2() {
        // N-BK7 from refractiveindex.info
        let data = DispersionData::Formula2 {
            wavelength_range: [0.3, 2.5],
            c: vec![
                0.0,
                1.03961212,
                0.00600069867,
                0.231792344,
                0.0200179144,
                1.01046945,
                103.560653,
            ],
        };

        let (n, k) = data.interpolate(0.5876).unwrap();
        assert_abs_diff_eq!(n, 1.51680, epsilon = 1e-5);
        assert!(k.is_none());
    }

    #[test]
    fn test_interpolate_formula_3() {
        // Ohara BAH10 from refractiveindex.info
        let data = DispersionData::Formula3 {
            wavelength_range: [0.365, 0.9],
            c: vec![
                2.730459,
                -0.01063385,
                2.0,
                0.01942756,
                -2.0,
                0.0008209873,
                -4.0,
                -5.210457e-05,
                -6.0,
                4.447534e-06,
                -8.0,
            ],
        };
        let (n, k) = data.interpolate(0.5876).unwrap();
        assert_abs_diff_eq!(n, 1.6700, epsilon = 1e-4);
        assert!(k.is_none());
    }

    #[test]
    fn test_interpolate_formula_4() {
        // CH4N20 Urea from refractiveindex.info
        let data = DispersionData::Formula4 {
            wavelength_range: [0.3, 1.06],
            c: vec![2.1823, 0.0125, 0.0, 0.0300, 1.0, 0.0, 0.0, 0.0, 1.0],
        };
        let (n, k) = data.interpolate(0.5876).unwrap();
        assert_abs_diff_eq!(n, 1.4906, epsilon = 1e-4);
        assert!(k.is_none());
    }

    #[test]
    fn test_interpolate_formula_5() {
        // BK7 matching liquid from refractiveindex.info
        let data = DispersionData::Formula5 {
            wavelength_range: [0.31, 1.55],
            c: vec![1.502787, 455872.4E-8, -2.0, 9.844856E-5, -4.0],
        };
        let (n, k) = data.interpolate(0.5876).unwrap();
        assert_abs_diff_eq!(n, 1.5168, epsilon = 1e-4);
        assert!(k.is_none());
    }

    #[test]
    fn test_interpolate_formula_6() {
        // H2 (Peck) in main shelf from refractiveindex.info
        let data = DispersionData::Formula6 {
            wavelength_range: [0.168, 1.6945],
            c: vec![0.0, 0.0148956, 180.7, 0.0049037, 92.0],
        };
        let (n, k) = data.interpolate(0.5876).unwrap();
        assert_abs_diff_eq!(n, 1.00013881, epsilon = 1e-8);
        assert!(k.is_none());
    }

    #[test]
    fn test_interpolate_formula_7() {
        // Si (Edwards) in main shelf of refractiveindex.info
        let data = DispersionData::Formula7 {
            wavelength_range: [2.4373, 25.0],
            c: vec![3.41983, 0.159906, -0.123109, 1.26878E-6, -1.95104E-9],
        };
        let (n, k) = data.interpolate(2.4373).unwrap();
        assert_abs_diff_eq!(n, 3.4434, epsilon = 1e-4);
        assert!(k.is_none());
    }

    #[test]
    fn test_interpolate_formula_8() {
        // TlCl (Schroter) in main shelf of refractiveindex.info
        let data = DispersionData::Formula8 {
            wavelength_range: [0.43, 0.66],
            c: vec![0.47856, 0.07858, 0.08277, -0.00881],
        };
        let (n, k) = data.interpolate(0.5876).unwrap();
        assert_abs_diff_eq!(n, 2.2636, epsilon = 1e-4);
        assert!(k.is_none());
    }

    #[test]
    fn test_interpolate_formula_9() {
        // CH4N2O Urea (Rosker-e) from refractiveindex.info
        let data = DispersionData::Formula9 {
            wavelength_range: [0.3, 1.06],
            c: vec![2.51527, 0.0240, 0.0300, 0.020, 1.52, 0.8771],
        };
        let (n, k) = data.interpolate(0.5876).unwrap();
        assert_abs_diff_eq!(n, 1.6065, epsilon = 1e-4);
        assert!(k.is_none());
    }
}
