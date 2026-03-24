use orchid_core::{Config, SqliteStorage};
use std::sync::{Arc, Mutex};

pub struct AppState {
    pub config: Config,
    pub storage: Arc<Mutex<SqliteStorage>>,
}
