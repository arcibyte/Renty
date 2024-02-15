use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
    pub due_date: Option<NaiveDate>,
}

impl Task {
    pub fn new(id: u32, description: String, due_date: Option<NaiveDate>) -> Self {
        Task {
            id,
            description,
            completed: false,
             due_date,
        }
    }
}
