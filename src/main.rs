#![feature(custom_derive)]
#[macro_use]
extern crate clap;

extern crate tdo_core;


#[allow(unused_imports)]
use std::fs::File;
use tdo_core::{tdo,error};
use clap::App;
use std::env;
use std::process::exit;


#[allow(unused_variables)]
fn main() {
    // initialize the clap App handle
    let yml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yml).version(crate_version!()).get_matches();

    let save_path = match env::home_dir() {
        Some(path) => path.join("/.tdo/list.json"),
        None       => {
            println!("[ERROR] You seem to have no home directory. Unfortunately this is required in order to use tdo.");
            exit(1);
        }
    };
    println!("[DEBUG] location: {:?}", &save_path);

    let tdo = match tdo::Tdo::load(save_path.to_str().unwrap()) {
        Ok(loaded)                              => loaded,
        Err(error::ErrorKind::StorageError(error::StorageError::FileNotFound))  => tdo::Tdo::new(),
        Err(error::ErrorKind::StorageError(error::StorageError::FileCorrupted)) => {
            println!("[ERROR] The saved JSON could not be parsed. \
            [ERROR] Please fix the saved json file manually or delete it to continue.");
            exit(1);
        }
        Err(_) => panic!()
    };


    tdo.save("./test.json").unwrap();
    println!("[DEBUG] tdo json content: {:?}", tdo);
}
