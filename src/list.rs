extern crate json;

use json::JsonValue;
use todo::Todo;

#[derive(Debug, Clone)]
pub struct TodoList {
    pub name: String,
    pub list: Vec<Todo>
}


impl Into<JsonValue> for TodoList {
    fn into(self) -> JsonValue {
        // new list for the generated JSON values of the todos
        let mut jsonlist = JsonValue::new_array();

        // clone and reverse the list
        // seems to be counter-intuitive to clone and reverse the list just to pop every
        // element and push it onto a new vec but this is the only way I could think of in my
        // undercaffeinated state. TODO: Rewrite this
        let mut exportlist = self.list.clone();
        exportlist.reverse();

        while !exportlist.is_empty() {
            jsonlist.push(exportlist.pop().unwrap());
        }

        object!{
            "name" => self.name,
            "list" => jsonlist
        }
    }
}


impl TodoList {
    pub fn create(name: &str) -> TodoList {
        TodoList {
            name: name.to_string(),
            list: Vec::new()
        }
    }
}
