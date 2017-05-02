#[macro_use]
extern crate clap;

extern crate tdo_core;
extern crate tdo_export;
extern crate colored;

#[macro_use]
mod macros;
mod filesystem;

use tdo_core::{tdo, error};
use clap::App;
use std::env;
use std::process::exit;
use colored::*;
mod subcommands;



#[allow(unused_variables)]
fn main() {
    // initialize the clap App handle
    let yml = load_yaml!("cli.yml");
    let app = App::from_yaml(yml)
        .version(crate_version!())
        .get_matches();

    let save_path = match env::home_dir() {
        Some(path) => path.join(".tdo/list.json"),
        None => {
            errorprint!("You seem to have no home directory. Unfortunately this is required \
                      in order to use tdo.");
            exit(1);
        }
    };
    let mut tdo: tdo::Tdo = match tdo::Tdo::load(save_path.to_str().unwrap()) {
        Ok(loaded) => loaded,
        Err(error::Error(error::ErrorKind::StorageError(
            error::storage_error::ErrorKind::FileNotFound), _)) => tdo::Tdo::new(),
        Err(error::Error(error::ErrorKind::StorageError(
            error::storage_error::ErrorKind::FileCorrupted), _)) => {
            errorprint!("The saved JSON could not be parsed.");
            errorprint!("Please fix the saved json file manually or delete it to continue.");
            exit(1);
        }
        Err(error::Error(error::ErrorKind::StorageError(
            error::storage_error::ErrorKind::UnableToConvert), _)) => {
            errorprint!("The File could not be converted to the new version automatically.");
            errorprint!("Please fix the saved json file manually or delete it to continue.");
            exit(1);
        }
        Err(e) => {
            println!("{:?}", e);
            panic!();
        }
    };

    match app.subcommand {
        None => {
            match app.value_of("task") {
                Some(task_string) => subcommands::add(&mut tdo, task_string, app.value_of("list")),
                None => subcommands::print_out(&tdo, false),
            }
        }
        Some(_) => {
            match app.subcommand() {
                ("all", Some(_)) => subcommands::print_out(&tdo, true),
                ("add", Some(sub_m)) => {
                    let task_string = sub_m.value_of("task").unwrap();
                    subcommands::add(&mut tdo, task_string, sub_m.value_of("list"));
                }
                ("edit", Some(sub_m)) => {
                    let id: u32 = match sub_m.value_of("id").unwrap().parse() {
                        Ok(id) => id,
                        Err(_) => {
                            errorprint!("id must be va valid integer.");
                            exit(1);
                        }
                    };
                    subcommands::edit(&mut tdo, id);
                }
                ("done", Some(sub_m)) => {
                    let id: u32 = match sub_m.value_of("id").unwrap().parse() {
                        Ok(id) => id,
                        Err(_) => {
                            errorprint!("id must be va valid integer.");
                            exit(1);
                        }
                    };
                    subcommands::done(&mut tdo, id);
                }
                ("newlist", Some(sub_m)) => {
                    let new_list = match sub_m.value_of("listname") {
                        Some(name) => name,
                        None => {
                            errorprint!("listname could not be parsed.");
                            exit(1);
                        }
                    };
                    subcommands::newlist(&mut tdo, new_list);
                }
                ("move", Some(sub_m)) => {
                    let id: u32 = match sub_m.value_of("id").unwrap().parse() {
                        Ok(id) => id,
                        Err(_) => {
                            errorprint!("id must be va valid integer.");
                            exit(1);
                        }
                    };
                    subcommands::move_todo(&mut tdo, id, sub_m.value_of("listname").unwrap());
                }
                ("remove", Some(sub_m)) => {
                    let list_name = match sub_m.value_of("listname") {
                        Some(name) => name,
                        None => {
                            errorprint!("listname could not be parsed.");
                            exit(1);
                        }
                    };
                    subcommands::remove(&mut tdo, list_name);
                }
                ("github", Some(sub_m)) => {
                    match sub_m.subcommand {
                        None => {
                            let repo = sub_m.value_of("repo").unwrap();
                            let title = sub_m.value_of("title").unwrap();
                            subcommands::github(&mut tdo, repo, title, sub_m.value_of("body"));
                        }
                        Some(_) => {
                            match sub_m.subcommand() {
                                ("set", Some(subs)) => {
                                    subcommands::github_set(&mut tdo, subs.value_of("token"))
                                }
                                _ => println!("{:?}", sub_m),
                            }
                        }
                    }
                }
                ("clean", Some(sub_m)) => subcommands::clean(&mut tdo, sub_m.value_of("listname")),
                ("lists", Some(_)) => subcommands::lists(&tdo),
                ("export", Some(sub_m)) => {
                    let filepath = match sub_m.value_of("destination") {
                        Some(path) => path,
                        None => {
                            errorprint!("destination could not be parsed");
                            exit(1);
                        }
                    };
                    subcommands::export(&tdo, filepath, sub_m.is_present("undone"));
                }
                ("reset", Some(_)) => {
                    match subcommands::reset(&mut tdo) {
                        Some(x) => tdo = x,
                        None => {}
                    }
                }
                _ => println!("{}", app.usage()),
            }
        }
    }

    let target = match filesystem::validate_target_file(save_path.to_str().unwrap()) {
        Ok(path) => path,
        Err(e) => {
            errorprint!(e);
            return;
        }
    };
    match tdo.save(target.to_str().unwrap()) {
        Err(e) => errorprint!(e),
        _ => {}
    }
}
