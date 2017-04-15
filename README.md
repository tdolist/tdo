# tdo-rs

[![license](https://img.shields.io/crates/l/tdo.svg)](https://crates.io/crates/tdo/)
[![Language](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![version](https://img.shields.io/crates/v/tdo.svg)](https://crates.io/crates/tdo/)
[![Build Status](https://travis-ci.org/tdolist/tdo-rs.svg?branch=master)](https://travis-ci.org/tdolist/tdo-rs)

A todo list for the terminal.

This is a rewrite of our old [tdo](https://github.com/tdolist/tdo).

This is a simple todo list tool that integrates in your terminal workflow.  
Featuring multiple todo lists and exporting your list to Markdown, it aims to be a well-structured assistant in your daily routine when you don't feel like leaving the terminal.

## Installation

Install with cargo:
```bash
cargo install tdo
```

For a manual installation, clone this repository or download the [latest release](https://github.com/tdolist/tdo-rs/releases/latest) and run:
```bash
cd in/your/directory
cargo build --release
# copy to /usr/local/
cp target/release/tdo /usr/local/bin/
# or symlink it
ln -s target/release/tdo /usr/local/bin/tdo
```
## Usage

__Notice__: If you have todos or listnames with spaces do not forget to escape them or put the whole string in `''`.

For a list of all available commands please see `tdo help`, this is just an overview to demonstrate what you can do with tdo.

#### Simple todos
To add a todo, simply type `tdo add 'todo goes here'`. If your todo consists of just one word, you can leave out the quotes.

However, if you want to add the todo to a list _(for lists, see below)_, type `tdo add 'todo' listname`. Note that you can type the listname in lowercase letters, tdo will still find your list.

#### Multiple Lists
tdo features working with multiple lists. Add a new list with `tdo newlist listname` (remember to use quotation marks for a listname containing spaces!).

To remove it again, use `tdo remove listname`. You will be prompted to confirm the deletion to avoid accidents.

#### Export your todos
If you feel like exporting your todos _(e.g. for printing a checklist)_, you can do that with `tdo export filename`. All your todo lists will be exported into Markdown. You can decide if you want to includw tasks that were marked as 'done' or not.

## Future features
- [ ] Different themes
- [ ] Server sync

## License

This work is published under the [MIT License](LICENSE.txt).
