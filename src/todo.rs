
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    id: u32,
    name: String,
    done: bool,
}


impl Todo {
    pub fn new(id: u32, name: &str) -> Todo {
        Todo {
            id: id,
            name: name.to_string(),
            done: false,
        }
    }

}
