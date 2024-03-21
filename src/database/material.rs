use serde::{Deserialize, Serialize};
use serde_yaml::Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum Data {
    #[serde(rename = "formula 1")]
    Formula1 {
        wavelength_range: String,
        coefficients: String,
    },

    #[serde(rename = "formula 2")]
    Formula2 {
        wavelength_range: String,
        coefficients: String,
    },

    #[serde(rename = "formula 3")]
    Formula3 {
        wavelength_range: String,
        coefficients: String,
    },

    #[serde(rename = "tabulated k")]
    TabulatedK { data: String },

    #[serde(rename = "tabulated nk")]
    TabulatedNK { data: String },
}

#[derive(Serialize, Deserialize, Debug)]
struct Material {
    #[serde(rename = "REFERENCES")]
    references: String,

    #[serde(rename = "COMMENTS")]
    comments: String,

    #[serde(rename = "DATA")]
    data: Vec<Data>,

    #[serde(rename = "SPECS")]
    specs: Option<Value>,
}

#[cfg(test)]
mod test {
    use super::*;

    fn yaml() -> &'static str {
        r#"
        REFERENCES: \"<a href=\"http://refractiveindex.info/download/data/2017/schott_2017-01-20b.agf\">SCHOTT Zemax catalog 2017-01-20b</a> (obtained from <a href=\"http://www.schott.com/advanced_optics/english/download/\">http://www.schott.com</a>)<br>See also <a href=\"http://refractiveindex.info/download/data/2017/schott_2017-01-20.pdf\">SCHOTT glass data sheets</a>\"
        COMMENTS: \"lead containing glass type\"
        DATA:
          - type: formula 2 
            wavelength_range: 0.37 2.5
            coefficients: 0 1.70579259 0.0133874699 0.344223052 0.0579561608 1.09601828 121.616024
          - type: tabulated k
            data: |
                0.370 5.4237E-06
                0.380 2.7852E-06
                0.390 1.1068E-06
        SPECS:
            n_is_absolute: false
            wavelength_is_vacuum: false
            temperature: 20.0 °C
            thermal_dispersion:
              - type: "Schott formula"
                coefficients: 6.02e-06 1.7e-08 -2.61e-11 1.63e-06 1.59e-09 0.269
            nd: 1.7847
            Vd: 26.08
            glass_code: 785261.492
            glass_status: standard
            density: 4.92 g/cm<sup>3</sup>
            thermal_expansion:
              - temperature_range: -30 70 °C
                coefficient: 7.9e-06 K<sup>-1</sup>
              - temperature_range: 20 300 °C
                coefficient: 8.8e-06 K<sup>-1</sup>
            dPgF: 0.0098
            climatic_resistance: 1.0
            stain_resistance: 1.0
            acid_resistance: 3.2
            alkali_resistance: 2.2
            phosphate_resistance: 3.2
        "#
    }

    #[test]
    fn test_deserialize_material() {
        let _: Material = serde_yaml::from_str(yaml()).unwrap();
    }
}