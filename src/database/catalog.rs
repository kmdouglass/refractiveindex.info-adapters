//! A RefractiveIndex.INFO catalog.
//!
//! A catalog is a hierarchical structure that contains information about the
//! location of material data within the database. Materials are identified by a
//! shelf, book, and page key.
use serde::{Deserialize, Serialize};

pub type Catalog = Vec<Shelf>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum BookContent {
    Divider {
        #[serde(rename = "DIVIDER")]
        divider: String,
    },
    Page {
        #[serde(rename = "PAGE")]
        page: String,
        name: String,
        data: std::path::PathBuf,
        info: Option<std::path::PathBuf>,
    },

    // Special case for Hikari i-line glasses which have numbers as names
    PageNumberName {
        #[serde(rename = "PAGE")]
        page: u64,
        name: String,
        data: std::path::PathBuf,
        info: Option<std::path::PathBuf>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ShelfContent {
    Divider {
        #[serde(rename = "DIVIDER")]
        divider: String,
    },
    Book {
        #[serde(rename = "BOOK")]
        book: String,
        name: String,
        info: Option<std::path::PathBuf>,
        content: Vec<BookContent>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Shelf {
    #[serde(rename = "SHELF")]
    pub shelf: String,
    pub name: String,
    pub info: Option<std::path::PathBuf>,
    pub content: Vec<ShelfContent>,
}

#[cfg(test)]
mod tests {
    fn yaml() -> &'static str {
        "
        - SHELF: main
          name: \"MAIN - simple inorganic materials\"
          content:
            - DIVIDER: \"Ag - Silver\"
            - BOOK: Ag
              name: \"Ag (Silver)\"
              info: \"main/Ag.html\"
              content:
                - DIVIDER: \"Experimental data: bulk, thick film\"
                - PAGE: Johnson
                  name: \"Johnson and Christy 1972: n,k 0.188–1.94 µm\"
                  data: \"main/Ag/Johnson.yml\"
                - PAGE: Choi
                  name: \"Choi et al. 2020: n,k 1.23–6.99 µm\"
                  data: \"main/Ag/Choi.yml\"
        "
    }

    #[test]
    #[cfg(feature = "cli")]
    fn test_deserialize_catalog() {
        let _: crate::database::Catalog = serde_yaml::from_str(yaml()).unwrap();
    }
}
