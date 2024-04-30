use embd;
use serde_json::{Result, Value};

use crate::{table::Table, utils::{prep, ptable_repr}};

mod table;
mod utils;

fn main() {
    let db_unformatted = embd::string!("../db/periodic-table-lookup.json");
    let db: Value = serde_json::from_str(&db_unformatted).unwrap();
    let ptable_order = db.get("order").unwrap(); // peel a layer of from db

    let ptable_repr_raw = ptable_repr::STANDARD;
    let ptable_repr = prep::table_to_bool(ptable_repr_raw);

    let ptable = vec![vec![' '; ptable_repr[0].len()]; ptable_repr.len()];

    let group_len = ptable_repr.len();
    let period_len = ptable_repr[0].len();

    for i in 0..period_len {
        for j in 0..group_len {
            
        }
    }

    println!("{:#?}", ptable_order);
}
