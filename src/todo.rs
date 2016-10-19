extern crate json;

use json::JsonValue;


#[derive(Debug, Clone)]
pub struct Todo {
    id: u32,
    name: String,
    done: bool
}


impl Into<JsonValue> for Todo {
    fn into(self) -> JsonValue {
        object!{
            "id"   => self.id,
            "name" => self.name,
            "done" => self.done
        }
    }
}


impl Todo {
    pub fn new(id: u32, name: &str) -> Todo { Todo{ id:id, name:name.to_string(), done:false } }
}
