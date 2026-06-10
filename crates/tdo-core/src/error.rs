//! Error types for tdo-core.
//!
//! Uses `thiserror` for derive-based, typed errors that callers can match on.
//! Binaries wrap these with `anyhow` for richer context.

use uuid::Uuid;

/// Errors that can occur when operating on a [`TodoCollection`](crate::model::TodoCollection).
#[derive(Debug, thiserror::Error)]
pub enum CollectionError {
    #[error("list with id {0} not found")]
    ListNotFound(Uuid),

    #[error("list named {0:?} not found")]
    ListNameNotFound(String),

    #[error("todo with id {0} not found")]
    TodoNotFound(Uuid),

    #[error("a list named {0:?} already exists")]
    DuplicateListName(String),
}

/// Errors that can occur during serialization/deserialization of a collection.
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
}
