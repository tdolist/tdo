#![allow(unused_variables, unused_imports)]
use super::tdo_core::{tdo, error};

pub fn print_out(tdo: &super::tdo_core::tdo::Tdo, all: bool) {
    println!("print out", );
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
}

pub fn reset(tdo: &mut tdo::Tdo) {
    println!("reset", );
}
