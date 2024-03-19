use std::collections::HashMap;

type Database = HashMap<String, Shelf>;
type Books = HashMap<String, Book>;
type Pages = HashMap<String, Page>;

#[derive(Debug)]
struct Shelf { title: String, name: String, books: Books, }

#[derive(Debug)]
struct Book { title: String, name: String, info: std::path::PathBuf, pages: Pages,}

#[derive(Debug)]
struct Page { title: String, name: String, data: std::path::PathBuf }

#[derive(Debug)]
struct Divider { title: String, }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database() {
        let mut db = Database::new();
        let shelf = Shelf { title: "Shelf 1".to_string(), name: "shelf_1".to_string(), books: Books::new() };
        db.insert("shelf_1".to_string(), shelf);
        assert_eq!(db.len(), 1);
    }
}
