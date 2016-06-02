extern crate clap;

use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("tdo")
                          .version("0.0.1")
                          .author("Felix Wittwer <dev@felixwittwer.de>, Felix Döring <mail@felixdoering.com>")
                          .about("A todo list tool for the terminal")
                          .get_matches();
}
