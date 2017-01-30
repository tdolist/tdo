#![feature(custom_derive)]
#[macro_use]
extern crate clap;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tdo_core;


#[allow(unused_imports)]

use std::fs::File;
use std::io::Read;
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


    let mut f = File::create("foo.json").unwrap();

    let _ = serde_json::to_writer_pretty(&mut f, &tdo);

    println!("{:?}", serde_json::to_string_pretty(&tdo).unwrap());

    // deserialisation

    let mut s = String::new();
    let _ = File::open("foo.json").unwrap().read_to_string(&mut s).unwrap();
    let json: list::Tdo = serde_json::from_str(&s).unwrap();
    println!("{:?}", &json);
    println!("{:?}", &json.lists[0]);
}
