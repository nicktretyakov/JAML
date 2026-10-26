use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Domain events – Dependency Inversion: higher layers depend on this abstraction.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DomainEvent {
    ContentCreated {
        id: Uuid,
        slug: String,
        at: DateTime<Utc>,
    },
    ContentUpdated {
        id: Uuid,
        slug: String,
        at: DateTime<Utc>,
    },
    ContentPublished {
        id: Uuid,
        slug: String,
        at: DateTime<Utc>,
    },
    BuildStarted {
        at: DateTime<Utc>,
    },
    BuildFinished {
        at: DateTime<Utc>,
        duration_ms: u64,
        pages: usize,
    },
    BuildFailed {
        at: DateTime<Utc>,
        error: String,
    },
}

pub trait EventPublisher: Send + Sync {
    fn publish(&self, event: DomainEvent);
}
