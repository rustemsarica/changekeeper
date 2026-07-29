use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub name: String,
    pub description: Option<String>,
    pub branch: String,
    pub base_commit: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: u32,
    pub current_snapshot: u32,
}

impl Workspace {
    pub fn new(
        name: impl Into<String>,
        branch: impl Into<String>,
    ) -> Self {
        let now = Utc::now();

        Self {
            name: name.into(),
            description: None,
            branch: branch.into(),
            base_commit: None,
            created_at: now,
            updated_at: now,
            version: 1,
            current_snapshot: 0,
        }
    }

    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
        self.version += 1;
    }
    pub fn next_snapshot(&mut self) -> u32 {
        self.current_snapshot += 1;
        self.current_snapshot
    }
}