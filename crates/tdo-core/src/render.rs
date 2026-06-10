//! Text rendering utilities for a [`TodoCollection`](crate::model::TodoCollection).
//!
//! All functions are pure — no I/O, no external dependencies beyond the model.

use crate::model::TodoCollection;

/// Render a [`TodoCollection`] as a Markdown document.
///
/// If `only_undone` is `true`, done todos are omitted. Soft-deleted entities are
/// always omitted. Empty lists (after filtering) are skipped.
///
/// Returns `None` if there is nothing to render.
pub fn render_markdown(collection: &TodoCollection, only_undone: bool) -> Option<String> {
    let mut body = String::new();

    for list in collection.active_lists() {
        let todos: Vec<_> = list
            .visible_todos()
            .filter(|t| !only_undone || !t.done)
            .collect();

        if todos.is_empty() {
            continue;
        }

        body.push_str(&format!("\n### {}\n", list.name));
        for todo in todos {
            let check = if todo.done { "[x]" } else { "[ ]" };
            body.push_str(&format!("- {} {}\n", check, todo.title));
            if let Some(due) = todo.due_at {
                body.push_str(&format!("  - Due: {}\n", due.format("%Y-%m-%d %H:%M UTC")));
            }
        }
    }

    if body.is_empty() {
        return None;
    }

    let mut doc = String::from("# Your tasks\n");
    doc.push_str(&body);
    Some(doc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Todo, TodoCollection, TodoList};

    fn collection_with(lists: Vec<(&str, Vec<(&str, bool)>)>) -> TodoCollection {
        let mut col = TodoCollection::new();
        for (name, items) in lists {
            let mut list = TodoList::new(name);
            for (title, done) in items {
                let mut todo = Todo::new(title);
                if done {
                    todo.mark_done();
                }
                list.todos.push(todo);
            }
            col.add_list(list);
        }
        col
    }

    #[test]
    fn empty_collection_returns_none() {
        let col = TodoCollection::new();
        assert!(render_markdown(&col, false).is_none());
    }

    #[test]
    fn renders_all_todos() {
        let col = collection_with(vec![(
            "inbox",
            vec![("buy milk", false), ("read book", true)],
        )]);
        let md = render_markdown(&col, false).unwrap();
        assert!(md.contains("buy milk"));
        assert!(md.contains("read book"));
        assert!(md.contains("[x]"));
        assert!(md.contains("[ ]"));
    }

    #[test]
    fn only_undone_filters_done() {
        let col = collection_with(vec![(
            "inbox",
            vec![("buy milk", false), ("read book", true)],
        )]);
        let md = render_markdown(&col, true).unwrap();
        assert!(md.contains("buy milk"));
        assert!(!md.contains("read book"));
    }

    #[test]
    fn empty_list_after_filter_is_skipped() {
        let col = collection_with(vec![("inbox", vec![("done task", true)])]);
        // only_undone = true → nothing to show
        assert!(render_markdown(&col, true).is_none());
    }

    #[test]
    fn deleted_todos_are_omitted() {
        let mut col = TodoCollection::new();
        let mut list = TodoList::new("work");
        let mut todo = Todo::new("deleted task");
        todo.soft_delete();
        list.todos.push(todo);
        list.todos.push(Todo::new("active task"));
        col.add_list(list);

        let md = render_markdown(&col, false).unwrap();
        assert!(!md.contains("deleted task"));
        assert!(md.contains("active task"));
    }
}
