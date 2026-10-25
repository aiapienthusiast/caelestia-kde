//! GithubStore Singleton Model
//! Parity with `shell/modules/bar/components/GithubStore.qml`.

use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
pub struct GithubStoreSnapshot {
    pub days: Vec<u32>,
    pub total: u32,
    pub username: String,
    pub last_error: String,
    pub token_missing: bool,
    pub token_notice_shown: bool,
    pub available: bool,
}

#[derive(Debug, Clone, Default)]
pub struct GithubStore {
    inner: Arc<Mutex<GithubStoreSnapshot>>,
}

impl GithubStore {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(GithubStoreSnapshot::default())),
        }
    }

    pub fn snapshot(&self) -> GithubStoreSnapshot {
        self.inner.lock().unwrap_or_else(|e| e.into_inner()).clone()
    }

    pub fn update(&self, snapshot: GithubStoreSnapshot) {
        let mut lock = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        *lock = snapshot;
    }
}
