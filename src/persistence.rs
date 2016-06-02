use std::cell::RefCell;


pub struct tdolist {
    table: bool,
    tick: bool,
    global_id: u32,
    categories: RefCell<Vec<category>>
}

pub struct category {
    name: String,
    todos: RefCell<Vec<todo>>,
}

pub struct todo {
    id: u32,
    todo_text: String,
    done: bool,
}

impl todo {
    pub fn new(list: tdolist, todo_text: String, category_str: String) -> Result< (), &'static str> {
        let id = get_id();

        let mut exists: bool = false;
        let mut in_category: category;
            for x in list.categories.borrow_mut().iter() {
                if category_str.to_lowercase() == x.name.to_lowercase() {
                    let exists = true;
                    let in_category = x;
                    break;
                }
            }

        if !exists {
            Err("Category does not exist.")
        } else {
            let result = todo {
                id: id,
                todo_text: todo_text,
                done: false,
            };
            in_category.todos.borrow_mut().push(result);
            Ok(())
        }
    }
}

fn get_id() -> u32 {
    1
}
