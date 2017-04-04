#![feature(custom_derive)]
#[macro_use]
extern crate clap;

extern crate tdo_core;


#[allow(unused_imports)]
use std::fs::File;
use tdo_core::tdo;
use clap::App;
use std::env;


#[allow(unused_variables)]
fn main() {
    let yml = load_yaml!("cli.yml");
    let m = App::from_yaml(yml).version(crate_version!()).author(crate_authors!()).get_matches();
    let location: String;
    println!("{:?}", env::home_dir());
    match env::home_dir() {
        Some(path) => {
            location = String::from(path.to_str().unwrap()) + "/.tdo/list.json";
        },
        None => location = ".".to_owned(),
    }
    println!("{:?}", &location);
    let tdo = tdo::Tdo::load(&location).unwrap();
    tdo.save("./test.json").unwrap();
    println!("{:?}", tdo);
}
