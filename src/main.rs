#[macro_use]
extern crate clap;

extern crate tdo_core;
extern crate tdo_export;


#[allow(unused_imports)]
use tdo_core::{tdo, error};
use clap::App;
use std::env;
use std::process::exit;
mod subcommands;


#[allow(unused_variables)]
fn main() {
    // initialize the clap App handle
    let yml = load_yaml!("cli.yml");
    let app = App::from_yaml(yml).version(crate_version!()).get_matches();

    let save_path = match env::home_dir() {
        Some(path) => path.join(".tdo/list.json"),
        None => {
            println!("[ERROR] You seem to have no home directory. Unfortunately this is required \
                      in order to use tdo.");
            exit(1);
        }
    };
    println!("[DEBUG] location: {:?}", &save_path);
    let mut tdo: tdo::Tdo = match tdo::Tdo::load(save_path.to_str().unwrap()) {
        Ok(loaded) => loaded,
        Err(error::ErrorKind::StorageError(error::StorageError::FileNotFound)) => tdo::Tdo::new(),
        Err(error::ErrorKind::StorageError(error::StorageError::FileCorrupted)) => {
            println!("[ERROR] The saved JSON could not be parsed.\n\
            [ERROR] Please fix the saved json file manually or delete it to continue.");
            exit(1);
        }
        Err(error::ErrorKind::StorageError(error::StorageError::UnableToConvert)) => {
            println!("[ERROR] The File could not be converted to the new version automatically.\n\
            [ERROR] Please fix the saved json file manually or delete it to continue.");
            exit(1);
        }
        Err(e) => {
            println!("{:?}", e);
            panic!();
        }
    };

    match app.subcommand() {
        ("", None) => subcommands::print_out(&tdo, false),
        ("all", Some(_)) => subcommands::print_out(&tdo, true),
        ("add", Some(sub_m)) => {
            let task_string = sub_m.value_of("task").unwrap();
            let task_list = sub_m.value_of("list").unwrap_or("default");
            subcommands::add(&mut tdo, task_string, task_list);
        }
        ("edit", Some(sub_m)) => {
            let id: u32 = match sub_m.value_of("id").unwrap().parse() {
                Ok(x) => x,
                Err(_) => {
                    println!("[Error] id must be va valid integer.");
                    exit(1);
                }
            };
            subcommands::edit(&mut tdo, id);
        }
        ("done", Some(sub_m)) => {
            let id: u32 = match sub_m.value_of("id").unwrap().parse() {
                Ok(x) => x,
                Err(_) => {
                    println!("[Error] id must be va valid integer.");
                    exit(1);
                }
            };
            subcommands::done(&mut tdo, id);
        }
        ("newlist", Some(sub_m)) => {
            let new_list = match sub_m.value_of("listname") {
                Some(x) => x,
                None => {
                    println!("[Error] listname could not be parsed.");
                    exit(1);
                }
            };
            subcommands::newlist(&mut tdo, new_list);
        }
        ("remove", Some(sub_m)) => {
            let list_name = match sub_m.value_of("listname") {
                Some(x) => x,
                None => {
                    println!("[Error] listname could not be parsed.");
                    exit(1);
                }
            };
            subcommands::remove(&mut tdo, list_name);
        }
        ("clean", Some(_)) => subcommands::clean(&mut tdo),
        ("lists", Some(_)) => subcommands::lists(&tdo),
        ("export", Some(sub_m)) => {
            let filepath = match sub_m.value_of("destination") {
                Some(x) => x,
                None => {
                    println!("[Error] destination could not be parsed.");
                    exit(1);
                }
            };
            subcommands::export(&tdo, filepath, sub_m.is_present("undone"));
        }
        ("reset", Some(_)) => subcommands::reset(&mut tdo),
        _ => println!("{:?}", app.usage()),
    };

    println!("[DEBUG] tdo json content: {:?}", tdo);

}
