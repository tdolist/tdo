//! Commands that operate on todo lists.

use crate::store;
use anyhow::{Result, bail};
use colored::Colorize;
use std::io::{self, BufRead, Write};
use tdo_core::model::{TodoCollection, TodoList};

pub fn newlist(collection: &mut TodoCollection, name: &str) -> Result<()> {
    if collection.find_list_by_name(name).is_some() {
        bail!("a list named {name:?} already exists");
    }
    collection.add_list(TodoList::new(name));
    println!("Created list {}.", name.bold());
    store::save(collection)
}

pub fn remove(collection: &mut TodoCollection, name: &str) -> Result<()> {
    if collection.find_list_by_name(name).is_none() {
        bail!("list {name:?} not found");
    }

    print!(
        "{} Are you sure you want to delete list {name:?} and all its todos? [y/N] ",
        "[WARNING]".red().bold(),
    );
    io::stdout().flush()?;

    let mut answer = String::new();
    io::stdin().lock().read_line(&mut answer)?;

    match answer.trim().to_lowercase().as_str() {
        "y" | "yes" => {}
        _ => {
            println!("{}", "Aborted.".dimmed());
            return Ok(());
        }
    }

    let list = collection
        .lists
        .iter_mut()
        .find(|l| l.deleted_at.is_none() && l.name == name)
        .context("list disappeared — this is a bug")?;

    list.soft_delete();
    println!("Removed list {}.", name.bold());
    store::save(collection)
}

trait ContextExt<T> {
    fn context(self, msg: &'static str) -> Result<T>;
}

impl<T> ContextExt<T> for Option<T> {
    fn context(self, msg: &'static str) -> Result<T> {
        self.ok_or_else(|| anyhow::anyhow!("{msg}"))
    }
}
