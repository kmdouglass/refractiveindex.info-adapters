/// The database representation used internally by this library.
///
/// Note that this does not exactly match the schema used by refractiveindex.info. Instead, it uses maps for efficient lookups.
use std::collections::HashMap;

type Database = HashMap<String, Item>;

struct Item {
    shelf: String,
    book: String,
    page: String,
}
