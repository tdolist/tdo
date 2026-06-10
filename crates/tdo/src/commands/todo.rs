//! Commands that operate on individual todo items.

use crate::{config::Config, display::IdMap, store, timeparse};
use anyhow::{Context, Result, bail};
use colored::Colorize;
use std::io::{self, BufRead, Write};
use tdo_core::model::{Todo, TodoCollection, TodoList};

pub fn add(
    collection: &mut TodoCollection,
    title: &str,
    list_name: Option<&str>,
    due: Option<&str>,
    remind: Option<&str>,
    config: &Config,
) -> Result<()> {
    let target_name = list_name.unwrap_or(&config.local.default_list);

    // Create the default list on first use.
    if collection.find_list_by_name(target_name).is_none() {
        collection.add_list(TodoList::new(target_name));
        tracing::debug!("created list {target_name:?} on first use");
    }

    let list = collection
        .lists
        .iter_mut()
        .find(|l| l.deleted_at.is_none() && l.name == target_name)
        .context("list not found after creation — this is a bug")?;

    let mut todo = Todo::new(title);

    if let Some(due_str) = due {
        todo.set_due(Some(
            timeparse::parse_time_expr(due_str)
                .with_context(|| format!("invalid --due value {due_str:?}"))?,
        ));
    }

    if let Some(remind_str) = remind {
        todo.set_remind(Some(
            timeparse::parse_time_expr(remind_str)
                .with_context(|| format!("invalid --remind value {remind_str:?}"))?,
        ));
    }

    println!(
        "Added {} to list {list_name}",
        todo.title.bold(),
        list_name = target_name.italic(),
    );
    list.todos.push(todo);
    store::save(collection)
}

pub fn mark_done(collection: &mut TodoCollection, idx: u32, id_map: &IdMap) -> Result<()> {
    let uuid = id_map
        .uuid_for(idx)
        .with_context(|| format!("no task with index {idx}"))?;

    let todo = collection
        .find_todo_mut(uuid)
        .context("task not found — this is a bug")?;

    if todo.done {
        println!("Task #{idx} is already done.");
        return Ok(());
    }

    todo.mark_done();
    println!("Marked #{idx} {} as done.", todo.title.italic());
    store::save(collection)
}

pub fn edit(collection: &mut TodoCollection, idx: u32, id_map: &IdMap) -> Result<()> {
    let uuid = id_map
        .uuid_for(idx)
        .with_context(|| format!("no task with index {idx}"))?;

    let current_title = collection
        .find_todo(uuid)
        .map(|(_, t)| t.title.clone())
        .context("task not found — this is a bug")?;

    println!("Task #{idx}: {}", current_title.bold());
    print!("New description (leave blank to abort): ");
    io::stdout().flush()?;

    let mut new_title = String::new();
    io::stdin().lock().read_line(&mut new_title)?;
    let new_title = new_title.trim();

    if new_title.is_empty() {
        println!("{}", "Aborted.".dimmed());
        return Ok(());
    }

    let todo = collection
        .find_todo_mut(uuid)
        .context("task not found — this is a bug")?;
    todo.edit(new_title);
    println!("Updated task #{idx}.");
    store::save(collection)
}

pub fn set_due(
    collection: &mut TodoCollection,
    idx: u32,
    when: &str,
    id_map: &IdMap,
) -> Result<()> {
    let uuid = id_map
        .uuid_for(idx)
        .with_context(|| format!("no task with index {idx}"))?;

    let due = if when.eq_ignore_ascii_case("clear") {
        None
    } else {
        Some(
            timeparse::parse_time_expr(when)
                .with_context(|| format!("invalid time expression {when:?}"))?,
        )
    };

    let todo = collection
        .find_todo_mut(uuid)
        .context("task not found — this is a bug")?;
    todo.set_due(due);

    match due {
        Some(dt) => println!(
            "Set due date for #{idx} to {}.",
            dt.format("%Y-%m-%d %H:%M UTC")
        ),
        None => println!("Cleared due date for #{idx}."),
    }

    store::save(collection)
}

pub fn move_todo(
    collection: &mut TodoCollection,
    idx: u32,
    target_list_name: &str,
    id_map: &IdMap,
) -> Result<()> {
    let uuid = id_map
        .uuid_for(idx)
        .with_context(|| format!("no task with index {idx}"))?;

    // Find source list and clone the todo out.
    let todo = {
        let (list, todo) = collection
            .find_todo(uuid)
            .with_context(|| format!("no task with index {idx}"))?;
        if list.deleted_at.is_some() {
            bail!("the source list has been deleted");
        }
        todo.clone()
    };

    // Ensure target list exists.
    if collection.find_list_by_name(target_list_name).is_none() {
        bail!(
            "list {target_list_name:?} not found — create it with `tdo newlist {target_list_name}`"
        );
    }

    // Soft-delete from source.
    let src_todo = collection
        .find_todo_mut(uuid)
        .context("source task disappeared — this is a bug")?;
    src_todo.soft_delete();

    // Add fresh copy to target (new UUID-identical tombstone stays in source for sync).
    let target = collection
        .lists
        .iter_mut()
        .find(|l| l.deleted_at.is_none() && l.name == target_list_name)
        .context("target list disappeared — this is a bug")?;
    target.todos.push(todo.clone());

    println!(
        "Moved #{idx} {} to {target_list_name}.",
        todo.title.italic(),
        target_list_name = target_list_name.bold(),
    );
    store::save(collection)
}

pub fn clean(collection: &mut TodoCollection, list_name: Option<&str>) -> Result<()> {
    let mut count = 0usize;
    for list in collection.lists.iter_mut() {
        if list.deleted_at.is_some() {
            continue;
        }
        if let Some(name) = list_name
            && list.name != name
        {
            continue;
        }
        for todo in list.todos.iter_mut() {
            if todo.done && todo.deleted_at.is_none() {
                todo.soft_delete();
                count += 1;
            }
        }
    }

    match count {
        0 => println!("{}", "Nothing to clean.".dimmed()),
        n => println!(
            "Removed {n} completed task{}.",
            if n == 1 { "" } else { "s" }
        ),
    }

    store::save(collection)
}
