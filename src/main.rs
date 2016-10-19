#[macro_use] extern crate json;
extern crate clap;

#[allow(dead_code, unused_imports)]

use std::io::prelude::*;
use std::fs::File;
use clap::{Arg, App, SubCommand};
mod todo;
mod list;

fn main() {
    let matches = App::new("tdo")
                          .version("0.0.1")
                          .author("Felix Wittwer <dev@felixwittwer.de>, Felix Döring <mail@felixdoering.com>")
                          .about("A todo list tool for the terminal")
                          .get_matches();

    test();
}

fn test() {
    let mut li = list::TodoList::create("kek");
    let tdo1 = todo::Todo::new(0, "Bla");
    let tdo2 = todo::Todo::new(1, "Bla1");

    li.list.push(tdo1.clone());
    li.list.push(tdo2);

    let bla = json::from(tdo1);
    print!("{}", bla.dump());

    let mut f = File::create("foo.json").unwrap();

    bla.write_pretty(&mut f, 4);
}
