//! Core domain types for tdo.
//!
//! All identifiers use UUIDs. Sync metadata (`updated_at`, `deleted_at`) is
//! present on every mutable entity to support offline-first, last-write-wins merging.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A single todo item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Todo {
    /// Unique, stable identifier.
    pub id: Uuid,
    /// Short description of the task.
    pub title: String,
    /// Whether the task has been completed.
    pub done: bool,
    /// Optional deadline. The daemon fires a notification when this is reached.
    pub due_at: Option<DateTime<Utc>>,
    /// Optional explicit reminder time (fires before `due_at`).
    pub remind_at: Option<DateTime<Utc>>,
    /// Creation timestamp (immutable after creation).
    pub created_at: DateTime<Utc>,
    /// Last modification timestamp — used for last-write-wins conflict resolution.
    pub updated_at: DateTime<Utc>,
    /// Soft-delete tombstone. `Some` means the item has been deleted and the
    /// timestamp should be propagated during sync.
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Todo {
    /// Create a new, incomplete todo with a generated UUID and current timestamps.
    pub fn new(title: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title: title.into(),
            done: false,
            due_at: None,
            remind_at: None,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
    }

    /// Mark the todo as done, bumping `updated_at`.
    pub fn mark_done(&mut self) {
        self.done = true;
        self.updated_at = Utc::now();
    }

    /// Mark the todo as undone, bumping `updated_at`.
    pub fn mark_undone(&mut self) {
        self.done = false;
        self.updated_at = Utc::now();
    }

    /// Update the title, bumping `updated_at`.
    pub fn edit(&mut self, new_title: impl Into<String>) {
        self.title = new_title.into();
        self.updated_at = Utc::now();
    }

    /// Set a deadline, bumping `updated_at`.
    pub fn set_due(&mut self, due_at: Option<DateTime<Utc>>) {
        self.due_at = due_at;
        self.updated_at = Utc::now();
    }

    /// Set an explicit reminder, bumping `updated_at`.
    pub fn set_remind(&mut self, remind_at: Option<DateTime<Utc>>) {
        self.remind_at = remind_at;
        self.updated_at = Utc::now();
    }

    /// Soft-delete this todo, setting `deleted_at` to now.
    pub fn soft_delete(&mut self) {
        self.deleted_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Returns `true` if this todo has been soft-deleted.
    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }
}

/// A named group of todo items.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodoList {
    /// Unique, stable identifier.
    pub id: Uuid,
    /// Display name.
    pub name: String,
    /// All todos in this list, including soft-deleted ones (needed for sync).
    pub todos: Vec<Todo>,
    /// Creation timestamp (immutable).
    pub created_at: DateTime<Utc>,
    /// Last modification timestamp (name change, etc.).
    pub updated_at: DateTime<Utc>,
    /// Soft-delete tombstone for the whole list.
    pub deleted_at: Option<DateTime<Utc>>,
}

impl TodoList {
    /// Create a new, empty list with a generated UUID.
    pub fn new(name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            todos: Vec::new(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
    }

    /// Rename the list, bumping `updated_at`.
    pub fn rename(&mut self, new_name: impl Into<String>) {
        self.name = new_name.into();
        self.updated_at = Utc::now();
    }

    /// Soft-delete this list (and all its todos).
    pub fn soft_delete(&mut self) {
        let now = Utc::now();
        self.deleted_at = Some(now);
        self.updated_at = now;
        for todo in &mut self.todos {
            if todo.deleted_at.is_none() {
                todo.deleted_at = Some(now);
                todo.updated_at = now;
            }
        }
    }

    /// Returns `true` if this list has been soft-deleted.
    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }

    /// Returns only the non-deleted, undone todos.
    pub fn active_todos(&self) -> impl Iterator<Item = &Todo> {
        self.todos
            .iter()
            .filter(|t| t.deleted_at.is_none() && !t.done)
    }

    /// Returns all non-deleted todos (done and undone).
    pub fn visible_todos(&self) -> impl Iterator<Item = &Todo> {
        self.todos.iter().filter(|t| t.deleted_at.is_none())
    }
}

/// Top-level container — the user's full todo collection.
///
/// Replaces the old `Tdo` struct. Includes soft-deleted entities so that
/// deletions can be propagated during sync via tombstones.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TodoCollection {
    /// All lists, including soft-deleted ones.
    pub lists: Vec<TodoList>,
}

impl TodoCollection {
    /// Create an empty collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a new list. Returns a reference to it.
    pub fn add_list(&mut self, list: TodoList) -> &TodoList {
        self.lists.push(list);
        self.lists.last().unwrap()
    }

    /// Find a list by UUID, including soft-deleted ones.
    pub fn find_list(&self, id: Uuid) -> Option<&TodoList> {
        self.lists.iter().find(|l| l.id == id)
    }

    /// Find a list by UUID mutably.
    pub fn find_list_mut(&mut self, id: Uuid) -> Option<&mut TodoList> {
        self.lists.iter_mut().find(|l| l.id == id)
    }

    /// Find a list by name (case-sensitive), non-deleted only.
    pub fn find_list_by_name(&self, name: &str) -> Option<&TodoList> {
        self.lists
            .iter()
            .find(|l| l.deleted_at.is_none() && l.name == name)
    }

    /// Find a todo by UUID across all lists.
    pub fn find_todo(&self, id: Uuid) -> Option<(&TodoList, &Todo)> {
        self.lists
            .iter()
            .find_map(|l| l.todos.iter().find(|t| t.id == id).map(|t| (l, t)))
    }

    /// Find a todo mutably by UUID across all lists.
    pub fn find_todo_mut(&mut self, id: Uuid) -> Option<&mut Todo> {
        self.lists
            .iter_mut()
            .find_map(|l| l.todos.iter_mut().find(|t| t.id == id))
    }

    /// Returns all non-deleted lists.
    pub fn active_lists(&self) -> impl Iterator<Item = &TodoList> {
        self.lists.iter().filter(|l| l.deleted_at.is_none())
    }

    /// Returns `true` if there are no non-deleted todos across all lists.
    pub fn is_empty(&self) -> bool {
        self.lists
            .iter()
            .all(|l| l.is_deleted() || l.todos.iter().all(|t| t.is_deleted()))
    }
}
