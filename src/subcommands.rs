#![allow(unused_variables, unused_imports)]
use tdo_core::{tdo, error};
use tdo_export;
use std::fs::File;
use std::io::Write;


pub fn print_out(tdo: &super::tdo_core::tdo::Tdo, all: bool) {
    match tdo_export::render_terminal_output(&tdo, all) {
        Some(printlines) => {
            for item in printlines.iter() {
                println!("{}", item);
            }
        }
        None => println!("No todos yet"),
    }
}

pub fn add(tdo: &mut tdo::Tdo, new_todo: &str, in_list: &str) {
    println!("add", );
}

pub fn edit(tdo: &mut tdo::Tdo, id: u32) {
    println!("edit", );
}

pub fn done(tdo: &mut tdo::Tdo, id: u32) {
    println!("done", );
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
            file.write(&tdo_export::gen_tasks_md(&tdo, true).unwrap().into_bytes()).unwrap();
        }
        Err(x) => println!("{:?}", x),
    }
}

pub fn reset(tdo: &mut tdo::Tdo) {
    println!("reset", );
}
