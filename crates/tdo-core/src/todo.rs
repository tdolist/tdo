//! Implementation of a single Todo item.

/// Data Structure for a simple todo.
///
/// A `Todo` item is the atomic unit within the `tdo` microcosm.
/// It represents a single todo or task, which is identified by an ID.
/// Information about its state (_done_ or _undone_) can be derived from
/// the data structure as well as the title of the todo item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    /// Unique identifier for every Todo.
    pub id: u32,
    /// Title of the todo.
    pub name: String,
    /// Status of the todo.
    pub done: bool,
    /// Optional GitHub issue.
    pub github: Option<GitHub>,
}


impl Todo {
    /// Constructor. Creates a new Todo item.
    pub fn new(id: u32, name: &str, github: Option<GitHub>) -> Todo {
        Todo {
            id: id,
            name: name.to_string(),
            done: false,
            github: github,
        }
    }

    /// Edit the title of a given Todo.
    pub fn edit(&mut self, new_name: &str) {
        self.name = new_name.to_string();
    }

    /// Set the status of a Todo item to _done_.
    pub fn set_done(&mut self) {
        self.done = true;
    }

    /// Mark a todo item as _undone_.
    pub fn set_undone(&mut self) {
        self.done = false;
    }
}

/// Data Structure for a represented Github issue in an todo.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHub {
    /// Name of the repository (owner/repo).
    pub repo: String,
    /// Number of the issue.
    pub issue_number: u32,
}

impl GitHub {
    /// Constructor. Creates a new GitHub item.
    pub fn new(repo: &str, issue_number: u32) -> GitHub {
        GitHub {
            repo: repo.to_owned(),
            issue_number: issue_number,
        }
    }
}

/// Data Structure to parse responses from the Github API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GHIssueResponse {
    /// Repository URL.
    pub url: String,
    /// Issue number.
    pub number: u32,
    /// Current state of the Issue.
    pub state: String,
    /// Title of the Issue.
    pub title: String,
}
