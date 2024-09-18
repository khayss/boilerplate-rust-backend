use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: String,
    hash: String,
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub timestamp: i64,
}

