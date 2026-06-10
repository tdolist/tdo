//! Markdown export command.

use anyhow::{Context, Result, bail};
use std::path::Path;
use tdo_core::{model::TodoCollection, render};

pub fn export(collection: &TodoCollection, path: &Path, only_undone: bool) -> Result<()> {
    let md = render::render_markdown(collection, only_undone)
        .ok_or_else(|| anyhow::anyhow!("nothing to export — all todos are empty or deleted"))?;

    // Don't silently overwrite a directory.
    if path.is_dir() {
        bail!("{} is a directory", path.display());
    }

    // Create parent directories if needed.
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("could not create directory {}", parent.display()))?;
    }

    std::fs::write(path, &md).with_context(|| format!("could not write to {}", path.display()))?;

    println!("Exported todos to {}.", path.display());
    Ok(())
}
