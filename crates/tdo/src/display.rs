//! Terminal rendering using comfy-table.
//!
//! Because UUIDs are not user-friendly, the CLI assigns short sequential
//! display indices (1, 2, 3 …) to non-deleted todos in list/display order.
//! The mapping is rebuilt on every invocation — it is deterministic given
//! the current collection state.

use colored::Colorize;
use comfy_table::{Cell, CellAlignment, Color, ContentArrangement, Table, presets};
use std::collections::HashMap;
use tdo_core::model::{Todo, TodoCollection};
use uuid::Uuid;

/// A session-scoped mapping between short display indices and UUIDs.
pub struct IdMap {
    uuid_to_idx: HashMap<Uuid, u32>,
    idx_to_uuid: HashMap<u32, Uuid>,
}

impl IdMap {
    fn new() -> Self {
        Self {
            uuid_to_idx: HashMap::new(),
            idx_to_uuid: HashMap::new(),
        }
    }

    pub fn register(&mut self, id: Uuid) -> u32 {
        let next = self.uuid_to_idx.len() as u32 + 1;
        self.uuid_to_idx.insert(id, next);
        self.idx_to_uuid.insert(next, id);
        next
    }

    pub fn uuid_for(&self, idx: u32) -> Option<Uuid> {
        self.idx_to_uuid.get(&idx).copied()
    }
}

/// Build an [`IdMap`] from a collection in display order (non-deleted todos only).
pub fn build_id_map(collection: &TodoCollection) -> IdMap {
    let mut map = IdMap::new();
    for list in collection.active_lists() {
        for todo in list.visible_todos() {
            map.register(todo.id);
        }
    }
    map
}

/// Render the collection as a comfy-table, printing to stdout.
/// If `all` is false, only undone todos are shown.
pub fn print_collection(collection: &TodoCollection, all: bool) -> IdMap {
    let mut id_map = IdMap::new();

    let has_anything = collection
        .active_lists()
        .any(|l| l.visible_todos().any(|t| all || !t.done));

    if !has_anything {
        println!("{}", "No todos yet.".dimmed());
        return id_map;
    }

    let mut table = Table::new();
    table
        .load_preset(presets::NOTHING)
        .set_content_arrangement(ContentArrangement::Dynamic);

    for list in collection.active_lists() {
        let todos: Vec<&Todo> = list.visible_todos().filter(|t| all || !t.done).collect();

        if todos.is_empty() {
            continue;
        }

        // List header row
        table.add_row(vec![
            Cell::new(""),
            Cell::new(""),
            Cell::new(&list.name)
                .fg(Color::Blue)
                .add_attribute(comfy_table::Attribute::Bold),
        ]);

        for todo in todos {
            let idx = id_map.register(todo.id);
            let check = if todo.done {
                "[x]".green().to_string()
            } else {
                "[ ]".normal().to_string()
            };
            let title = if todo.done {
                todo.title.dimmed().to_string()
            } else {
                todo.title.clone()
            };
            let id_cell = Cell::new(idx).set_alignment(CellAlignment::Right);
            table.add_row(vec![Cell::new(check), id_cell, Cell::new(title)]);

            if let Some(due) = todo.due_at {
                let due_str = format!("due {}", due.format("%Y-%m-%d %H:%M UTC"));
                table.add_row(vec![
                    Cell::new(""),
                    Cell::new(""),
                    Cell::new(due_str).fg(Color::Yellow),
                ]);
            }
        }

        // Blank separator between lists
        table.add_row(vec![Cell::new(""), Cell::new(""), Cell::new("")]);
    }

    println!("{table}");
    id_map
}

/// Print a summary of all lists with done/undone counts.
pub fn print_list_summary(collection: &TodoCollection) {
    if collection.active_lists().next().is_none() {
        println!(
            "{}",
            "No lists yet. Use `tdo newlist <name>` to create one.".dimmed()
        );
        return;
    }

    println!();
    for list in collection.active_lists() {
        let total = list.visible_todos().count();
        let done = list.visible_todos().filter(|t| t.done).count();
        let undone = total - done;
        println!(
            "  [{undone}|{done}] {name}",
            undone = undone.to_string().red(),
            done = done.to_string().green(),
            name = &list.name,
        );
    }
    println!();
}
