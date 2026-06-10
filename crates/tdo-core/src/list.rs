//! General implementation of todo lists.
use crate::todo::Todo;
use crate::error::*;

/// Simple todo list structure.
///
/// Todos can be grouped together in so called todo lists (as in the real world).
/// Therefore, the `TodoList` struct can be used. It's a simple data structure that holds a
/// number of `Todo` items and offers all basic functions for managing them.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoList {
    /// Name of the todo list.
    pub name: String,
    /// The actual vector of `Todo` items.
    pub list: Vec<Todo>,
}

impl TodoList {
    /// Create a new list with the given name.
    ///
    /// # Example
    ///
    /// ```
    /// # use tdo_core::list::*;
    /// let mut list = TodoList::new("important");
    /// ```
    pub fn new(name: &str) -> TodoList {
        TodoList {
            name: name.to_string(),
            list: Vec::new(),
        }
    }

    /// Add a new todo to the list.
    ///
    /// # Example
    ///
    /// ```
    /// # use tdo_core::list::*;
    /// # use tdo_core::todo::*;
    /// # let mut list = TodoList::new("important");
    /// list.add(Todo::new(0,"A first important todo", None));
    /// ```
    pub fn add(&mut self, new_todo: Todo) {
        self.list.push(new_todo);
    }

    /// Check if the list contains a todo with the given ID.
    ///
    /// This function returns a `TdoResult`, wich will contion a `TodoError::NotInList`
    /// if the list does not contain any todo with the given ID or the position in the list.
    pub fn contains_id(&self, id: u32) -> TdoResult<usize> {
        match self.list.iter().position(|x| x.id == id) {
            Some(index) => Ok(index),
            None => Err(ErrorKind::TodoError(todo_error::ErrorKind::NotInList).into()),
        }
    }
    /// Mark a todo from the list with the given ID as done.
    ///
    /// This function returns a `TdoResult`, which will contain a `TodoError::NotInList`
    /// if the list does not contain any todo with the given ID.
    pub fn done_id(&mut self, id: u32) -> TdoResult<()> {
        match self.contains_id(id) {
            Ok(index) => Ok(self.list[index].set_done()),
            _ => Err(ErrorKind::TodoError(todo_error::ErrorKind::NotInList).into()),
        }
    }

    /// Remove a todo with the given ID from the list.
    ///
    /// This function returns a `TdoResult`, which will contain the removed Todo itself or a
    /// `TodoError::NotInList` if the list does not contain any todo with the given id.
    pub fn remove_id(&mut self, id: u32) -> TdoResult<Todo> {
        match self.contains_id(id) {
            Ok(index) => Ok(self.list.remove(index)),
            _ => Err(ErrorKind::TodoError(todo_error::ErrorKind::NotInList).into()),
        }
    }

    /// Search for all undone todos in the list.
    ///
    /// Returns a vector of all undone todos.
    pub fn list_undone(&self) -> Vec<Todo> {
        let mut undone: Vec<Todo> = vec![];
        for entry in self.to_owned().list.into_iter() {
            if !entry.done {
                undone.push(entry);
            }
        }
        undone
    }

    /// Remove all done todos from the list.
    pub fn clean(&mut self) {
        for entry in self.to_owned().list.into_iter() {
            if entry.done {
                let _ = self.remove_id(entry.id);
            }
        }
    }

    /// Remove a todo with a specific ID from the list.
    pub fn pop_id(&mut self, todo_id: u32) -> TdoResult<Todo> {
        let list_pos = self.contains_id(todo_id)?;

        Ok(self.list.remove(list_pos))
    }

    /// Insert an existing todo into the list, preserving the ordering of the internal list.
    pub fn insert_todo(&mut self, todo: Todo) {
        let insert_id = self.list
            .iter()
            .fold(0, |acc, &ref x| if todo.id > x.id { acc + 1 } else { acc });

        self.list.insert(insert_id, todo);
    }
}

/// Instanciates a _default_ `TodoList`.
/// This function is invoked when a `Tdo` container structure is instanciated.
impl Default for TodoList {
    fn default() -> TodoList {
        TodoList {
            name: "default".to_string(),
            list: Vec::new(),
        }
    }
}
