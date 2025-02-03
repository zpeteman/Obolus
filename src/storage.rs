use std::{fs, io, path::Path};
use serde_json;
use super::app::Transaction;

const FILE_PATH: &str = "transactions.json";

pub fn save_transactions(transactions: &[Transaction]) -> io::Result<()> {
    let data = serde_json::to_string(transactions)?;
    fs::write(FILE_PATH, data)
}

pub fn load_transactions() -> io::Result<Vec<Transaction>> {
    if Path::new(FILE_PATH).exists() {
        let data = fs::read_to_string(FILE_PATH)?;
        Ok(serde_json::from_str(&data)?)
    } else {
        Ok(Vec::new())
    }
}
