use std::cell::RefCell;


pub struct Tdolist {
    table: bool,
    tick: bool,
    global_id: u32,
    categories: RefCell<Vec<category>>
}

pub struct Category {
    name: String,
    todos: RefCell<Vec<todo>>,
}

pub struct Todo {
    id: u32,
    title: String,
    done: bool,
}

impl Todo {
    pub fn new(list: Tdolist, todo_text: String, category_str: String) -> Result< (), &'static str> {
        // Usually, new() returns the instance. Maybe This should be wrapped with an add()?

        let mut exists: bool = false;
        let mut in_category: Category;
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
                id: get_id(),
                title: todo_text,
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
