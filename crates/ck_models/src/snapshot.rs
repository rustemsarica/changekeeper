use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: u32,
    pub created_at: DateTime<Utc>,
}