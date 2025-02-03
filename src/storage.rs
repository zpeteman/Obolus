use std::fs;
use std::io;
use crate::models::Budget;
use serde_json;

const FILE_PATH: &str = "data/transactions.json";

pub fn save_budget(budget: &Budget) -> io::Result<()> {
    let data = serde_json::to_string(budget)?;
    fs::write(FILE_PATH, data)
}

pub fn load_budget() -> io::Result<Budget> {
    let data = fs::read_to_string(FILE_PATH)?;
    let budget: Budget = serde_json::from_str(&data)?;
    Ok(budget)
}
