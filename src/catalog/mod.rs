use std::collections::HashMap;

use serde::{Deserialize, Serialize};

type Catalog = HashMap<String, Shelf>;
type Books = HashMap<String, Book>;
type Pages = HashMap<String, Page>;

#[derive(Serialize, Deserialize, Debug)]
struct Shelf {
    title: String,
    name: String,
    content: Books,
}

#[derive(Serialize, Deserialize, Debug)]
struct Book {
    title: String,
    name: String,
    info: std::path::PathBuf,
    content: Pages,
}

#[derive(Serialize, Deserialize, Debug)]
struct Page {
    title: String,
    name: String,
    data: std::path::PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
struct Divider {
    title: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database() {
        let mut db = Catalog::new();
        let shelf = Shelf {
            title: "Shelf 1".to_string(),
            name: "shelf_1".to_string(),
            content: Books::new(),
        };
        db.insert("shelf_1".to_string(), shelf);
        assert_eq!(db.len(), 1);
    }

    #[test]
    fn test_shelf_serialize() {
        let mut books = Books::new();
        let book = Book {
            title: "Ag".to_string(),
            name: "Ag (Silver)".to_string(),
            info: std::path::PathBuf::from("main/Ag.html"),
            content: Pages::new(),
        };
        books.insert("Ag".to_string(), book);
        let shelf = Shelf {
            title: "main".to_string(),
            name: "MAIN - simple inorganic materials".to_string(),
            content: books,
        };
        let serialized = serde_yaml::to_string(&shelf).unwrap();
        println!("serialized = {}", serialized);
    }
}
