use std::fs::File;
use std::sync::Arc;

use serde_json::from_reader;
use tokio::sync::Mutex;

use crate::models::Pokemon;


pub type Db = Arc<Mutex<Vec<Pokemon>>>;

pub fn init_db() -> Db {
    let file = File::open("./data/pokemon.json");
    match file {
        Ok(json) => {
            let customers = from_reader(json).unwrap();
            Arc::new(Mutex::new(customers))
        }
        Err(_) => Arc::new(Mutex::new(Vec::new())),
    }
}
