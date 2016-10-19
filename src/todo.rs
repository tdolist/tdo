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


    pub fn from_json(json: &JsonValue) -> Option<Todo> {
        //safeguards to detect if someone tampered with the JSON File
        if !json["id"].is_number() || !json["name"].is_string() || !json["done"].is_boolean() {
            println!("[Error] The JSON file is corrupted. Please fix or delete the file.");
            None //TODO: Replace this with Ok/Err
        } else {
            Some(Todo {
                id: json["id"].as_u32().unwrap(),
                name: json["name"].as_str().unwrap().to_string(),
                done: json["done"].as_bool().unwrap()
            })
        }
    }
}
