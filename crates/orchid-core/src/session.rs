use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Common session metadata shared by all session types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Session {
    pub fn new(name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            created_at: now,
            updated_at: now,
        }
    }
}

/// A coding-focused session (repos, branches, tasks).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSession {
    #[serde(flatten)]
    pub session: Session,
    pub repo_path: Option<String>,
    pub branch: Option<String>,
}

/// A content-authoring session (posts, threads, newsletters).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentSession {
    #[serde(flatten)]
    pub session: Session,
    pub content_type: String,
    pub draft: Option<String>,
}

/// A relationship/CRM session (contacts, follow-ups).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipSession {
    #[serde(flatten)]
    pub session: Session,
    pub contact_name: Option<String>,
    pub context: Option<String>,
}
