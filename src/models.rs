use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

#[derive(Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub amount: f64,
    pub category: String,
    pub date: NaiveDate,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct Budget {
    pub transactions: Vec<Transaction>,
}

#[derive(PartialEq)]
pub enum Tab {
    Transactions,
    Summary,
    AddTransaction,
}

pub struct AppState {
    pub budget: Budget,
    pub selected_tab: Tab, // Track selected tab
    pub selected_transaction: Option<usize>, // Track selected transaction
    pub current_input_field: usize,
    pub input_amount: String,
    pub input_category: String,
    pub input_date: String,
    pub input_description: String,
}

impl Budget {
    pub fn new() -> Self {
        Budget {
            transactions: Vec::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    pub fn remove_transaction(&mut self, index: usize) {
        if index < self.transactions.len() {
            self.transactions.remove(index);
        }
    }

    pub fn total_balance(&self) -> f64 {
        self.transactions.iter().map(|t| t.amount).sum()
    }

    pub fn spending_by_category(&self) -> Vec<(String, f64)> {
        use std::collections::HashMap;
        let mut categories = HashMap::new();
        
        for t in &self.transactions {
            *categories.entry(t.category.clone()).or_insert(0.0) += t.amount.abs(); // Use absolute values
        }
        
        categories.into_iter().collect()
    }
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            budget: Budget::new(),
            selected_tab: Tab::Transactions,
            selected_transaction: None,
            current_input_field: 0,
            input_amount: String::new(),
            input_category: String::new(),
            input_date: String::new(),
            input_description: String::new(),
        }
    }
}
