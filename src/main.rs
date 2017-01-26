#![feature(custom_derive)]
#[macro_use] extern crate clap;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[allow(unused_imports)]

use std::fs::File;
use clap::App;
mod todo;
mod list;
mod storage;

fn main() {
    let yml = load_yaml!("cli.yml");
    let m = App::from_yaml(yml).version(crate_version!()).author(crate_authors!()).get_matches();
    // let matches = App::new("tdo")
    //     .version("0.0.1")
    //     .author("Felix Wittwer <dev@felixwittwer.de>, Felix Döring <mail@felixdoering.com>")
    //     .about("A todo list tool for the terminal")
    //     .subcommand(SubCommand::with_name("all").about("Lists all tasks."))
    //     .get_matches();
    println!("{:?}", m.args);

    test();
}

fn test() {
    let mut li = list::TodoList::new("kek");
    let tdo1 = todo::Todo::new(0, "Bla");
    let tdo2 = todo::Todo::new(1, "Bla1");

    li.list.push(tdo1.clone());
    li.list.push(tdo2);


    let mut f = File::create("foo.json").unwrap();

    let _ = serde_json::to_writer_pretty(&mut f, &li);

    println!("{:?}", serde_json::to_string_pretty(&li).unwrap());

    // deserialisation

    // let mut inpf = File::open("foo.json").unwrap();
    // let mut serialized = String::new();
    //
    // let _ = inpf.read_to_string(&mut serialized);
    //
    // if let Ok(j) = todo::Todo::from_json(&bla) {
    //     println!("{:?}", j);
    // }
}
