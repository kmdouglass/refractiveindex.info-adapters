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
        info: std::path::PathBuf,
        content: Vec<BookContent>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Shelf {
    #[serde(rename = "SHELF")]
    shelf: String,
    name: String,
    content: Vec<ShelfContent>,
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_deserialize_catalog() {
        let _: Catalog = serde_yaml::from_str(yaml()).unwrap();
    }
}
