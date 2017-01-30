#![feature(custom_derive)]
#[macro_use]
extern crate clap;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tdo_core;


#[allow(unused_imports)]
use std::fs::File;
use tdo_core::{todo, list, storage};
use clap::App;


#[allow(unused_variables)]
fn main() {
    let yml = load_yaml!("cli.yml");
    let m = App::from_yaml(yml).version(crate_version!()).author(crate_authors!()).get_matches();
    test();
}

#[allow(dead_code)]
fn test() {
    let mut tdo = list::Tdo::new();
    let mut li = list::TodoList::new("kek");
    let tdo1 = todo::Todo::new(0, "Bla");
    let tdo2 = todo::Todo::new(1, "Bla1");

    li.list.push(tdo1.clone());
    li.list.push(tdo2);
    tdo.lists.push(li);


    let _ = tdo.save("foo.json");

    // deserialisation

    let mut new_tdo = list::Tdo::new();
    new_tdo.load("foo.json");
    println!("{:?}", &new_tdo);
    println!("{:?}", &new_tdo.lists[0]);
}
