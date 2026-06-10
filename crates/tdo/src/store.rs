//! Local persistence for the TodoCollection.
//!
//! Stores data in `~/.local/share/tdo/todos.json`. Writes are atomic: a temp
//! file is written first, then renamed over the target to avoid corruption on
//! interrupted writes.

use anyhow::{Context, Result};
use std::path::PathBuf;
use tdo_core::model::TodoCollection;

/// Returns the path to the data file, creating parent directories if needed.
pub fn data_path() -> Result<PathBuf> {
    let dir = dirs::data_dir()
        .context("could not determine data directory")?
        .join("tdo");
    std::fs::create_dir_all(&dir)
        .with_context(|| format!("could not create data directory {}", dir.display()))?;
    Ok(dir.join("todos.json"))
}

/// Load the collection from disk. Returns an empty collection if no file exists yet.
pub fn load() -> Result<TodoCollection> {
    let path = data_path()?;
    if !path.exists() {
        tracing::debug!("no data file found, starting fresh");
        return Ok(TodoCollection::new());
    }
    let contents = std::fs::read_to_string(&path)
        .with_context(|| format!("could not read data file {}", path.display()))?;
    serde_json::from_str(&contents).context("could not parse data file — it may be corrupted")
}

/// Atomically save the collection to disk.
pub fn save(collection: &TodoCollection) -> Result<()> {
    let path = data_path()?;
    let tmp = path.with_extension("json.tmp");

    let contents = serde_json::to_string_pretty(collection).context("serialization failed")?;
    std::fs::write(&tmp, &contents)
        .with_context(|| format!("could not write temp file {}", tmp.display()))?;
    std::fs::rename(&tmp, &path)
        .with_context(|| format!("could not rename temp file to {}", path.display()))?;

    tracing::debug!("saved collection to {}", path.display());
    Ok(())
}
