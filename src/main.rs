use embd;
use serde_json::{Result, Value};

mod table;
mod utils;

fn main() {
    let db_unformatted = embd::string!("../db/PeriodicTableJSON.json");
    let db_formatted: Value = serde_json::from_str(&db_unformatted).unwrap();
    let db = &db_formatted["elements"];
}
