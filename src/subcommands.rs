#![allow(unused_variables, unused_imports)]
use tdo_core::*;
use tdo_export;
use std::fs::File;
use std::io::{Write, Read, stdin, stdout};
use super::exit;

pub fn print_out(tdo: &super::tdo_core::tdo::Tdo, all: bool) {
    match tdo_export::render_terminal_output(tdo, all) {
        Some(printlines) => {
            for item in printlines.iter() {
                println!("{}", item);
            }
        }
        None => println!("No todos yet"),
    }
}

pub fn add(tdo: &mut tdo::Tdo, new_todo: &str, in_list: Option<&str>) {
    let todo = todo::Todo::new(tdo.get_highest_id() + 1, new_todo);
    match tdo.add_todo(in_list, todo) {
        Err(e) => errorprint!(e),
        _ => {}
    }
}

pub fn edit(tdo: &mut tdo::Tdo, id: u32) {
    let list = match tdo.find_id(id) {
        Ok(list_id) => list_id,
        Err(e) => {
            errorprint!(e);
            exit(1);
        }
    };
    let position = match tdo.lists[list].contains_id(id) {
        Ok(position) => position,
        Err(e) => {
            errorprint!(e);
            exit(1);
        }
    };
    println!("Task #{}: {}", id, tdo.lists[list].list[position].name);
    println!("Enter your new task description (leave blank for abort)");
    let mut new_task = String::new();
    stdin().read_line(&mut new_task).unwrap();
    match new_task.trim() {
        "" => {}
        _ => tdo.lists[list].list[position].edit(&new_task.trim()),
    };
}

pub fn done(tdo: &mut tdo::Tdo, id: u32) {
    match tdo.done_id(id) {
        Ok(()) => {}
        Err(e) => println!("{:?}", e),
    }
}

pub fn newlist(tdo: &mut tdo::Tdo, new_list: &str) {
    println!("newlist", );
}

pub fn remove(tdo: &mut tdo::Tdo, list_name: &str) {
    println!("remove", );
}

pub fn clean(tdo: &mut tdo::Tdo) {
    println!("clean", );
}

pub fn lists(tdo: &tdo::Tdo) {
    println!("lists", );
}

pub fn export(tdo: &tdo::Tdo, destination: &str, undone: bool) {
    println!("export", );
    // TODO: check/create path; check for overwrite
    match File::create(destination) {
        Ok(mut file) => {
            file.write(&tdo_export::gen_tasks_md(tdo, true).unwrap().into_bytes()).unwrap();
        }
        Err(x) => println!("{:?}", x),
    }
}

pub fn reset(tdo: &mut tdo::Tdo) -> Option<tdo::Tdo> {
    println!("[WARNING] Are you sure you want to delete all todos and lists? [y/N] ");
    let mut answer = String::new();
    stdin().read_line(&mut answer).unwrap();
    let should_delete = match answer.to_lowercase().trim() {
        "y" | "yes" => true,
        "n" | "no" | "" => false,
        _ => {
            errorprint!("No valid answer given. Defaulting to \"no\"");
            false
        }
    };
    if should_delete {
        Some(tdo::Tdo::new())
    } else {
        None
    }
}
