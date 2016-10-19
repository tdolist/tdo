use todo::Todo;

pub struct TodoList {
    pub name: String,
    pub list: Vec<Todo>
}


impl TodoList {
    pub fn create(name: &str) -> TodoList {
        TodoList {
            name: name.to_string(),
            list: Vec::new()
        }
    }
}
