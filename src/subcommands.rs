#![allow(unused_variables, unused_imports)]
use tdo_core::*;
use tdo_export;
use std::fs::File;
use std::io::{Write, Read, stdin, stdout};
use super::exit;
use super::colored::*;

use filesystem;

pub fn print_out(tdo: &super::tdo_core::tdo::Tdo, all: bool) {
    match tdo_export::render_terminal_output(tdo, all) {
        Some(printlines) => {
            for item in printlines.iter() {
                println!("{}", item);
            }
        }
        None => {
            if tdo.get_highest_id() > 0 {
                println!("No undone todos");
            } else {
                println!("No todos yet");
            }
        }
    }
}

pub fn add(tdo: &mut tdo::Tdo, new_todo: &str, in_list: Option<&str>) {
    let todo = todo::Todo::new(tdo.get_highest_id() + 1, new_todo);
    match tdo.add_todo(in_list, todo) {
        Err(e) => errorprint!(e.description()),
        _ => {}
    }
}

pub fn edit(tdo: &mut tdo::Tdo, id: u32) {
    let list = match tdo.find_id(id) {
        Ok(list_id) => list_id,
        Err(e) => {
            errorprint!(e.description());
            exit(1);
        }
    };
    let position = match tdo.lists[list].contains_id(id) {
        Ok(position) => position,
        Err(e) => {
            errorprint!(e.description());
            exit(1);
        }
    };
    println!("Task #{}: {}", id, tdo.lists[list].list[position].name);
    println!("Enter your new task description (leave blank for abort)");
    let mut new_task = String::new();
    stdin().read_line(&mut new_task).unwrap();
    match new_task.trim() {
        "" => {},
        _ => tdo.lists[list].list[position].edit(&new_task.trim()),
    };
}

pub fn done(tdo: &mut tdo::Tdo, id: u32) {
    match tdo.done_id(id) {
        Ok(()) => {}
        Err(e) => errorprint!(e.description()),
    }
}

pub fn newlist(tdo: &mut tdo::Tdo, new_list: &str) {
    match tdo.add_list(list::TodoList::new(new_list)) {
        Ok(()) => { println!("Created a new list named '{}'", new_list); }
        Err(e) => errorprint!(e.description()),
    }
}

pub fn remove(tdo: &mut tdo::Tdo, list_name: &str) {
    println!("{}",
             format!("[WARNING] Are you sure you want to delete \"{}\" ans all todos in it? [y/N] ", &list_name).red());
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
        match tdo.remove_list(list_name) {
            Ok(()) => println!("Removed the list '{}'", list_name),
            Err(e) => errorprint!(e),
        }
    }

}

pub fn clean(tdo: &mut tdo::Tdo, list_name: Option<&str>) {
    match list_name {
        Some(name) => match tdo.clean_list(name) {
            Err(e) => errorprint!(e.description()),
            _ => {},
        },
        None => tdo.clean_lists(),
    }
}

pub fn lists(tdo: &tdo::Tdo) {
    println!("Your todo collection currently encompasses the following lists:\n");

    for list in tdo.lists.iter() {
        let done = list.list
            .iter()
            .fold(0, |acc, &ref item| if item.done { acc + 1 } else { acc });
        let undone: u64 = (list.list.len() - done) as u64;
        println!("    [{undone}|{done}] {name}",
                 undone = undone.to_string().red(),
                 done = (done as u64).to_string().green(),
                 name = &list.name);
    }
}

pub fn export(tdo: &tdo::Tdo, destination: &str, undone: bool) {
    let target = match filesystem::validate_target_file(destination) {
        Ok(path) => path,
        Err(e) => {
            errorprint!(e);
            return;
        }
    };
    match File::create(target) {
        Ok(mut file) => {
            file.write(&tdo_export::gen_tasks_md(tdo, true).unwrap().into_bytes())
                .unwrap();
        }
        Err(e) => errorprint!(e),
    }
}

pub fn reset(tdo: &mut tdo::Tdo) -> Option<tdo::Tdo> {
    println!("{}",
             "[WARNING] Are you sure you want to delete all todos and lists? [y/N] ".red());
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
