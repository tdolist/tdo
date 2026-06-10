//! General implementation of tdos base structure.
use json::parse;
use std::fs::File;
use std::io::{Read, Write, stdout, stdin};
use crate::list::TodoList;
use crate::legacy::*;
use crate::todo::Todo;
use crate::error::*;

/// Basic container structure for a set of todo lists.
///
/// This data structure acts as a conatiner for all todo lists and its associated todos.
/// The whole `tdo` microcosm settles around this structure
/// which is also used for (de-)serialization.
///
/// When instanciated, it comes with an empty _default_ list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tdo {
    /// A vector of all todo lists.
    pub lists: Vec<TodoList>,
    //The Github API token.
    access_token: Option<String>,
    // The tdo version the last dump was saved with.
    version: String,
}

impl Tdo {
    /// Create a new `Tdo` container.
    /// Each new container is instanciated with a _default_ `TodoList`.
    ///
    /// # Example
    ///
    /// ```
    /// # use tdo_core::tdo::*;
    /// let tdo = Tdo::new();
    /// ```
    pub fn new() -> Tdo {
        Tdo {
            lists: vec![TodoList::default()],
            access_token: None,
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    /// Load a saved `Tdo` container from a JSON file.
    ///
    /// This function returns a `ResultType` which will yield the
    /// deserialized JSON or a `serde_json::Error`.
    ///
    /// # Example
    ///
    /// ```
    /// # use tdo_core::tdo::*;
    /// let mut tdo = Tdo::load("foo.json");
    /// ```
    pub fn load(path: &str) -> TdoResult<Tdo> {
        match File::open(path) {
            Ok(file) => {
                match serde_json::from_reader(&file) {
                    Ok(tdo) => Ok(tdo),
                    Err(_) => update_json(path),
                }
            }
            Err(_) => Err(ErrorKind::StorageError(storage_error::ErrorKind::FileNotFound).into()),
        }

    }

    /// Dump the `Tdo` container to a JSON file.
    ///
    /// This function returns a `ResultType` yielding a `StorageError::SaveFailure`
    /// if the JSON file could not be opened/saved.
    ///
    /// # Example
    ///
    /// ```
    /// # use tdo_core::tdo::*;
    /// # let mut tdo = Tdo::new();
    /// let res = tdo.save("foo.json");
    /// assert_eq!(res.unwrap(), ());
    /// ```
    pub fn save(&self, path: &str) -> TdoResult<()> {
        // TODO: At this point we could be much more precise about the error if we would include
        // the error from the file system as SaveFailure(ArbitraryErrorFromFS)
        //  -- Feliix42 (2017-03-14; 17:04)
        match File::create(path) {
            Ok(mut f) => {
                let _ = serde_json::to_writer_pretty(&mut f, self);
                Ok(())
            }
            Err(_) => Err(ErrorKind::StorageError(storage_error::ErrorKind::SaveFailure).into()),
        }
    }

    /// Sets the GitHub access token.
    pub fn set_gh_token(&mut self, token: Option<&str>) {
        let gh_token = match token {
            Some(x) => x.to_string(),
            None => {
                print!("Please generate an access token ({})\nand enter a valid accesstoken: ",
                       "https://github.com/settings/tokens/new?scopes=repo&description=tdolist");
                stdout().flush().ok().expect("Could not flush stdout!");
                let mut answer = String::new();
                stdin().read_line(&mut answer).unwrap();
                answer.trim().to_string()
            }
        };
        self.access_token = Some(gh_token);
    }

    /// Returns an Option<String> of the private access_token field.
    pub fn get_gh_token(&self) -> Option<String> {
        self.access_token.to_owned()
    }

    /// Add a todo list to the container.
    pub fn add_list(&mut self, list: TodoList) -> TdoResult<()> {
        match self.get_list_index(&list.name) {
            Ok(_) => Err(ErrorKind::TodoError(todo_error::ErrorKind::NameAlreadyExists).into()),
            Err(_) => {
                self.lists.push(list);
                Ok(())
            }
        }
    }

    /// Removes a list from the container.
    pub fn remove_list(&mut self, list_name: &str) -> TdoResult<()> {
        if list_name == "default" {
            Err(ErrorKind::TodoError(todo_error::ErrorKind::CanNotRemoveDefault).into())
        } else {
            match self.get_list_index(list_name) {
                Ok(index) => {
                    self.lists.remove(index);
                    Ok(())
                }
                Err(_) => Err(ErrorKind::TodoError(todo_error::ErrorKind::NoSuchList).into()),
            }
        }
    }

    /// Add a todo to the todo list, identified by its name.
    ///
    /// This function returns a `ResultType` with a `TodoError::NoSuchList`
    /// if there is no matching list found.
    pub fn add_todo(&mut self, list_name: Option<&str>, todo: Todo) -> TdoResult<()> {
        match self.get_list_index(&list_name.unwrap_or("default")) {
            Ok(index) => {
                self.lists[index].add(todo);
                Ok(())
            }
            Err(x) => Err(x),
        }
    }

    /// Cycle through all todo lists and find the list which contains the todo with the given ID
    ///
    /// This function retuns a `ResultType` with a `TodoError::NotInList`
    /// if there is no list found or a usize with the postition of the list in lists.
    pub fn find_id(&self, id: u32) -> TdoResult<usize> {
        for list in 0..self.lists.len() {
            if self.lists[list].contains_id(id).is_ok() {
                return Ok(list);
            }
        }
        Err(ErrorKind::TodoError(todo_error::ErrorKind::NotInList).into())
    }
    /// Cycle through all todo lists and mark a todo with the given ID as done.
    /// This function has no return value and thus won't indicate whether
    /// there was a matching todo found.
    pub fn done_id(&mut self, id: u32) -> TdoResult<()> {
        let list = match self.find_id(id) {
            Ok(list_id) => list_id,
            Err(e) => return Err(e),
        };
        self.lists[list].done_id(id)
    }

    /// Cycle through all todo lists and remove a todo with the given id.
    /// This function has no return value and thus won't indicate whether
    /// there was a matching todo found.
    pub fn remove_id(&mut self, id: u32) -> TdoResult<()> {
        let list = match self.find_id(id) {
            Ok(list_id) => list_id,
            Err(e) => return Err(e),
        };
        match self.lists[list].remove_id(id) {
            Err(e) => Err(e),
            _ => Ok(()),
        }
    }

    /// Remove all todos that have been marked as _done_ from all todo lists.
    pub fn clean_lists(&mut self) {
        for list in 0..self.lists.len() {
            self.lists[list].clean();
        }
    }

    /// Remove all todos that have been marked as _done_ from a given todo list.
    pub fn clean_list(&mut self, list: &str) -> TdoResult<()> {
        let index = match self.get_list_index(list) {
            Ok(index) => index,
            Err(e) => return Err(e),
        };
        self.lists[index].clean();
        Ok(())
    }

    fn get_list_index(&self, name: &str) -> TdoResult<usize> {
        match self.lists
            .iter()
            .position(|x| x.name.to_lowercase() == name.to_string().to_lowercase()) {
            Some(index) => Ok(index),
            None => Err(ErrorKind::TodoError(todo_error::ErrorKind::NoSuchList).into()),
        }
    }

    /// Get the highest ID used in the tdo container.
    pub fn get_highest_id(&self) -> u32 {
        self.lists
            .iter()
            .fold(0, |acc, &ref x| {
                x.list
                    .iter()
                    .fold(acc,
                          |inner_acc, &ref y| if inner_acc < y.id { y.id } else { inner_acc })
            })
    }

    /// Move a `todo` between two lists.
    pub fn move_todo(&mut self, id: u32, target_list: &str) -> TdoResult<()> {
        let src_index = self.find_id(id)?;
        let target = self.get_list_index(target_list)?;

        //Check if todo is a github Issue
        let list_index = self.lists[src_index].contains_id(id)?;
        if let Some(_) = self.lists[src_index].list[list_index].github {
            return Err(ErrorKind::GithubError(github_error::ErrorKind::NotAllowedToMove).into())
        }
        let todo = self.lists[src_index].pop_id(id)?;
        self.lists[target].insert_todo(todo);
        Ok(())
    }
}

fn update_json(path: &str) -> TdoResult<Tdo> {
    match Tdo01::load(path) {
        Ok(tdo) => Ok(tdo.into()),
        Err(_) => {
            println!("I have to do this here");
            let mut file = File::open(path).unwrap();
            let mut data = String::new();
            file.read_to_string(&mut data).unwrap();
            let mut json = match parse(&data) {
                Ok(content) => content,
                Err(_) => {
                    return Err(ErrorKind::StorageError(storage_error::ErrorKind::FileCorrupted)
                        .into())
                }
            };

            let mut lists: Vec<TodoList> = vec![];

            for outer in json.entries_mut() {
                let mut list = TodoList::new(outer.0);
                for inner in outer.1.entries_mut() {
                    let tdo_id = match inner.0.parse::<u32>() {
                        Ok(id) => id,
                        Err(_) => return Err(ErrorKind::StorageError(storage_error::ErrorKind::UnableToConvert).into()),
                    };
                    let done = match inner.1.pop().as_bool() {
                        Some(x) => x,
                        None => return Err(ErrorKind::StorageError(storage_error::ErrorKind::UnableToConvert).into()),
                    };
                    let tdo_name = match inner.1.pop().as_str() {
                        Some(x) => String::from(x),
                        None => return Err(ErrorKind::StorageError(storage_error::ErrorKind::UnableToConvert).into()),
                    };
                    let mut todo = Todo::new(tdo_id, &tdo_name, None);
                    if done {
                        todo.set_done();
                    }
                    list.add(todo);
                }
                lists.push(list);
            }
            let tdo = Tdo {
                lists: lists,
                access_token: None,
                version: env!("CARGO_PKG_VERSION").to_string(),
            };
            Ok(tdo)
        }
    }
}
