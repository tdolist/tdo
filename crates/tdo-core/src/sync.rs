//! Offline-first sync logic — last-write-wins with tombstone propagation.
//!
//! The [`merge`] function is a **pure function** with no I/O. It can be called
//! identically on the CLI (before pushing to the server) and on the server itself
//! (when receiving a push), ensuring consistent behaviour and easy unit-testing.

use crate::model::{Todo, TodoCollection, TodoList};
use std::collections::HashMap;
use uuid::Uuid;

/// The result of a [`merge`] operation.
#[derive(Debug, Clone)]
pub struct MergeResult {
    /// The merged collection — the new source of truth.
    pub merged: TodoCollection,
    /// Which entities were updated from the remote side (for CLI output).
    pub updated_from_remote: Vec<Uuid>,
    /// Which entities were updated from the local side (i.e., local won).
    pub updated_from_local: Vec<Uuid>,
}

/// Merge two collections using last-write-wins on `updated_at`.
///
/// Rules:
/// - Entity exists on only one side → include as-is.
/// - Entity exists on both sides → the one with the newer `updated_at` wins.
/// - If `deleted_at` is set on the winner, the tombstone propagates (the entity
///   remains in the collection as a tombstone to prevent resurrection on next sync).
pub fn merge(local: &TodoCollection, remote: &TodoCollection) -> MergeResult {
    let mut updated_from_remote: Vec<Uuid> = Vec::new();
    let mut updated_from_local: Vec<Uuid> = Vec::new();
    let mut merged_lists: Vec<TodoList> = Vec::new();

    // Index remote lists by id for O(1) lookup.
    let mut remote_lists: HashMap<Uuid, &TodoList> =
        remote.lists.iter().map(|l| (l.id, l)).collect();

    for local_list in &local.lists {
        if let Some(remote_list) = remote_lists.remove(&local_list.id) {
            // Both sides have this list — merge at list level, then merge todos.
            let (mut merged_list, from_remote, from_local) = merge_list(local_list, remote_list);

            // Merge todos within the list independently.
            let (merged_todos, todo_remote, todo_local) =
                merge_todos(&local_list.todos, &remote_list.todos);
            merged_list.todos = merged_todos;

            updated_from_remote.extend(from_remote);
            updated_from_remote.extend(todo_remote);
            updated_from_local.extend(from_local);
            updated_from_local.extend(todo_local);
            merged_lists.push(merged_list);
        } else {
            // Only exists locally.
            merged_lists.push(local_list.clone());
        }
    }

    // Remaining lists only exist on the remote side.
    for remote_list in remote_lists.into_values() {
        updated_from_remote.push(remote_list.id);
        for todo in &remote_list.todos {
            updated_from_remote.push(todo.id);
        }
        merged_lists.push(remote_list.clone());
    }

    MergeResult {
        merged: TodoCollection {
            lists: merged_lists,
        },
        updated_from_remote,
        updated_from_local,
    }
}

/// Merge two versions of the same list at the list level (name, deleted_at, etc.).
/// Returns (merged_list, ids_from_remote, ids_from_local).
/// Note: todos are *not* merged here; they are merged separately.
fn merge_list(local: &TodoList, remote: &TodoList) -> (TodoList, Vec<Uuid>, Vec<Uuid>) {
    let remote_wins = remote.updated_at > local.updated_at;
    let winner = if remote_wins { remote } else { local };

    let merged = TodoList {
        id: winner.id,
        name: winner.name.clone(),
        created_at: winner.created_at,
        updated_at: winner.updated_at,
        deleted_at: winner.deleted_at,
        todos: Vec::new(), // filled by the caller
    };

    let from_remote = if remote_wins { vec![local.id] } else { vec![] };
    let from_local = if !remote_wins { vec![local.id] } else { vec![] };

    (merged, from_remote, from_local)
}

/// Merge two todo slices using last-write-wins.
/// Returns (merged_todos, ids_from_remote, ids_from_local).
fn merge_todos(local: &[Todo], remote: &[Todo]) -> (Vec<Todo>, Vec<Uuid>, Vec<Uuid>) {
    let mut from_remote: Vec<Uuid> = Vec::new();
    let mut from_local: Vec<Uuid> = Vec::new();
    let mut merged: Vec<Todo> = Vec::new();

    let mut remote_map: HashMap<Uuid, &Todo> = remote.iter().map(|t| (t.id, t)).collect();

    for local_todo in local {
        if let Some(remote_todo) = remote_map.remove(&local_todo.id) {
            if remote_todo.updated_at > local_todo.updated_at {
                from_remote.push(local_todo.id);
                merged.push(remote_todo.clone());
            } else {
                from_local.push(local_todo.id);
                merged.push(local_todo.clone());
            }
        } else {
            // Only local.
            merged.push(local_todo.clone());
        }
    }

    // Remaining todos only on the remote side.
    for remote_todo in remote_map.into_values() {
        from_remote.push(remote_todo.id);
        merged.push(remote_todo.clone());
    }

    (merged, from_remote, from_local)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Todo, TodoCollection, TodoList};
    use chrono::{Duration, Utc};

    fn make_collection(list_name: &str, todos: Vec<Todo>) -> TodoCollection {
        let mut col = TodoCollection::new();
        let mut list = TodoList::new(list_name);
        list.todos = todos;
        col.add_list(list);
        col
    }

    fn todo_aged(title: &str, hours_ago: i64) -> Todo {
        let mut t = Todo::new(title);
        t.updated_at = Utc::now() - Duration::hours(hours_ago);
        t
    }

    fn todo_with_id_aged(id: Uuid, title: &str, hours_ago: i64) -> Todo {
        let mut t = Todo::new(title);
        t.id = id;
        t.updated_at = Utc::now() - Duration::hours(hours_ago);
        t
    }

    #[test]
    fn only_local_items_are_preserved() {
        let local = make_collection("inbox", vec![Todo::new("local task")]);
        let remote = TodoCollection::new();
        let result = merge(&local, &remote);
        assert_eq!(result.merged.lists.len(), 1);
        assert_eq!(result.merged.lists[0].todos.len(), 1);
        assert_eq!(result.merged.lists[0].todos[0].title, "local task");
    }

    #[test]
    fn only_remote_items_are_preserved() {
        let local = TodoCollection::new();
        let remote = make_collection("inbox", vec![Todo::new("remote task")]);
        let result = merge(&local, &remote);
        assert_eq!(result.merged.lists.len(), 1);
        assert_eq!(result.merged.lists[0].todos[0].title, "remote task");
        assert!(!result.updated_from_remote.is_empty());
    }

    #[test]
    fn newer_local_wins_over_older_remote() {
        let local_todo = todo_aged("local version", 0); // most recent
        let id = local_todo.id;
        let remote_todo = todo_with_id_aged(id, "remote version", 2);

        let local = make_collection("inbox", vec![local_todo]);
        let mut remote = local.clone();
        remote.lists[0].todos = vec![remote_todo];

        let result = merge(&local, &remote);
        assert_eq!(result.merged.lists[0].todos[0].title, "local version");
        assert!(result.updated_from_local.contains(&id));
    }

    #[test]
    fn newer_remote_wins_over_older_local() {
        let local_todo = todo_aged("local version", 2);
        let id = local_todo.id;
        let remote_todo = todo_with_id_aged(id, "remote version", 0); // most recent

        let local = make_collection("inbox", vec![local_todo]);
        let mut remote = local.clone();
        remote.lists[0].todos = vec![remote_todo];

        let result = merge(&local, &remote);
        assert_eq!(result.merged.lists[0].todos[0].title, "remote version");
        assert!(result.updated_from_remote.contains(&id));
    }

    #[test]
    fn tombstone_propagates_on_sync() {
        let mut local_todo = todo_aged("to be deleted", 0); // deleted recently
        let id = local_todo.id;
        local_todo.soft_delete();

        let remote_todo = todo_with_id_aged(id, "to be deleted", 2); // older, not deleted

        let local = make_collection("inbox", vec![local_todo]);
        let mut remote = local.clone();
        remote.lists[0].todos = vec![remote_todo];

        let result = merge(&local, &remote);
        let todo = &result.merged.lists[0].todos[0];
        assert!(todo.is_deleted(), "tombstone should propagate");
    }

    #[test]
    fn newer_edit_beats_older_tombstone() {
        let mut local_todo = Todo::new("task");
        let id = local_todo.id;
        local_todo.updated_at = Utc::now() - Duration::hours(2);
        local_todo.deleted_at = Some(Utc::now() - Duration::hours(2));

        let remote_todo = todo_with_id_aged(id, "edited after deletion", 0);

        let local = make_collection("inbox", vec![local_todo]);
        let mut remote = local.clone();
        remote.lists[0].todos = vec![remote_todo];

        let result = merge(&local, &remote);
        let todo = &result.merged.lists[0].todos[0];
        assert!(!todo.is_deleted(), "newer edit should beat older tombstone");
        assert_eq!(todo.title, "edited after deletion");
    }
}
