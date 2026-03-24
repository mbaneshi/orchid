use anyhow::Result;
use rusqlite::Connection;
use std::path::Path;

use crate::session::Session;

/// Abstract storage backend.
pub trait Storage {
    fn init(&self) -> Result<()>;
    fn save_session(&self, session: &Session) -> Result<()>;
    fn load_session(&self, id: &uuid::Uuid) -> Result<Option<Session>>;
    fn list_sessions(&self) -> Result<Vec<Session>>;
}

/// SQLite-backed storage implementation.
pub struct SqliteStorage {
    conn: Connection,
}

impl SqliteStorage {
    pub fn open(path: &Path) -> Result<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(path)?;
        let storage = Self { conn };
        storage.init()?;
        Ok(storage)
    }
}

impl Storage for SqliteStorage {
    fn init(&self) -> Result<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS sessions (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                data TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS migrations (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                applied_at TEXT NOT NULL DEFAULT (datetime('now'))
            );",
        )?;
        Ok(())
    }

    fn save_session(&self, session: &Session) -> Result<()> {
        let data = serde_json::to_string(session)?;
        self.conn.execute(
            "INSERT OR REPLACE INTO sessions (id, name, data, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                session.id.to_string(),
                &session.name,
                &data,
                session.created_at.to_rfc3339(),
                session.updated_at.to_rfc3339(),
            ),
        )?;
        Ok(())
    }

    fn load_session(&self, id: &uuid::Uuid) -> Result<Option<Session>> {
        let mut stmt = self
            .conn
            .prepare("SELECT data FROM sessions WHERE id = ?1")?;
        let mut rows = stmt.query_map([id.to_string()], |row| {
            let data: String = row.get(0)?;
            Ok(data)
        })?;

        match rows.next() {
            Some(Ok(data)) => {
                let session: Session = serde_json::from_str(&data)?;
                Ok(Some(session))
            }
            Some(Err(e)) => Err(e.into()),
            None => Ok(None),
        }
    }

    fn list_sessions(&self) -> Result<Vec<Session>> {
        let mut stmt = self
            .conn
            .prepare("SELECT data FROM sessions ORDER BY updated_at DESC")?;
        let rows = stmt.query_map([], |row| {
            let data: String = row.get(0)?;
            Ok(data)
        })?;

        let mut sessions = Vec::new();
        for row in rows {
            let data = row?;
            let session: Session = serde_json::from_str(&data)?;
            sessions.push(session);
        }
        Ok(sessions)
    }
}
