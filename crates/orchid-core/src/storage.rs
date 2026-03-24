use anyhow::Result;
use rusqlite::Connection;
use std::path::Path;
use uuid::Uuid;

use crate::session::Session;

/// Abstract storage backend.
pub trait Storage {
    fn init(&self) -> Result<()>;
    fn save_session(&self, session: &Session) -> Result<()>;
    fn load_session(&self, id: &Uuid) -> Result<Option<Session>>;
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

    pub fn save_artifact(
        &self,
        id: &Uuid,
        artifact_type: &str,
        content: &str,
        metadata: Option<&str>,
    ) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO artifacts (id, artifact_type, content, metadata)
             VALUES (?1, ?2, ?3, ?4)",
            (id.to_string(), artifact_type, content, metadata),
        )?;
        Ok(())
    }

    pub fn execute_batch(&self, sql: &str) -> Result<()> {
        self.conn.execute_batch(sql)?;
        Ok(())
    }

    pub fn execute_sql(&self, sql: &str, params: &[&str]) -> Result<()> {
        self.conn
            .execute(sql, rusqlite::params_from_iter(params))?;
        Ok(())
    }

    pub fn query_one(&self, sql: &str, params: &[&str]) -> Result<Option<String>> {
        let mut stmt = self.conn.prepare(sql)?;
        let mut rows = stmt.query_map(rusqlite::params_from_iter(params), |row| {
            let val: String = row.get(0)?;
            Ok(val)
        })?;
        match rows.next() {
            Some(Ok(val)) => Ok(Some(val)),
            Some(Err(e)) => Err(e.into()),
            None => Ok(None),
        }
    }

    pub fn query_all(&self, sql: &str, params: &[&str]) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare(sql)?;
        let rows = stmt.query_map(rusqlite::params_from_iter(params), |row| {
            let val: String = row.get(0)?;
            Ok(val)
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn list_artifacts_by_type(
        &self,
        artifact_type: &str,
    ) -> Result<Vec<(Uuid, String, String)>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, content, created_at FROM artifacts WHERE artifact_type = ?1 ORDER BY created_at DESC",
        )?;
        let rows = stmt.query_map([artifact_type], |row| {
            let id: String = row.get(0)?;
            let content: String = row.get(1)?;
            let created_at: String = row.get(2)?;
            Ok((id, content, created_at))
        })?;

        let mut results = Vec::new();
        for row in rows {
            let (id, content, created_at) = row?;
            let uuid = Uuid::parse_str(&id).unwrap_or_else(|_| Uuid::new_v4());
            results.push((uuid, content, created_at));
        }
        Ok(results)
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

            CREATE TABLE IF NOT EXISTS artifacts (
                id TEXT PRIMARY KEY,
                session_id TEXT,
                artifact_type TEXT NOT NULL,
                content TEXT NOT NULL,
                metadata TEXT,
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
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

    fn load_session(&self, id: &Uuid) -> Result<Option<Session>> {
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
