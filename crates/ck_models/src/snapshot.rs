use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: u32,
    pub created_at: DateTime<Utc>,
    pub message: Option<String>,
}

impl Snapshot {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            created_at: Utc::now(),
            message: None,
        }
    }
}