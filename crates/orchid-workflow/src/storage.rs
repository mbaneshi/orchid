use anyhow::Result;
use orchid_core::SqliteStorage;

use crate::Workflow;

/// Extension trait for workflow CRUD on SqliteStorage.
pub trait WorkflowStorage {
    fn init_workflows(&self) -> Result<()>;
    fn save_workflow(&self, workflow: &Workflow) -> Result<()>;
    fn load_workflow(&self, name: &str) -> Result<Option<Workflow>>;
    fn list_workflows(&self) -> Result<Vec<Workflow>>;
    fn delete_workflow(&self, name: &str) -> Result<()>;
}

impl WorkflowStorage for SqliteStorage {
    fn init_workflows(&self) -> Result<()> {
        self.execute_batch(
            "CREATE TABLE IF NOT EXISTS workflows (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                definition TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at TEXT NOT NULL DEFAULT (datetime('now'))
            );",
        )?;
        Ok(())
    }

    fn save_workflow(&self, workflow: &Workflow) -> Result<()> {
        self.init_workflows()?;
        let definition = serde_json::to_string(workflow)?;
        self.execute_sql(
            "INSERT OR REPLACE INTO workflows (id, name, definition, created_at, updated_at)
             VALUES (?1, ?2, ?3, datetime('now'), datetime('now'))",
            &[&workflow.id.to_string(), &workflow.name, &definition],
        )?;
        Ok(())
    }

    fn load_workflow(&self, name: &str) -> Result<Option<Workflow>> {
        self.init_workflows()?;
        let result = self.query_one(
            "SELECT definition FROM workflows WHERE name = ?1",
            &[name],
        )?;
        match result {
            Some(json) => Ok(Some(serde_json::from_str(&json)?)),
            None => Ok(None),
        }
    }

    fn list_workflows(&self) -> Result<Vec<Workflow>> {
        self.init_workflows()?;
        let rows = self.query_all(
            "SELECT definition FROM workflows ORDER BY name",
            &[],
        )?;
        rows.iter()
            .map(|json| Ok(serde_json::from_str(json)?))
            .collect()
    }

    fn delete_workflow(&self, name: &str) -> Result<()> {
        self.init_workflows()?;
        self.execute_sql("DELETE FROM workflows WHERE name = ?1", &[name])?;
        Ok(())
    }
}
