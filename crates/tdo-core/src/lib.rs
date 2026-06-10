//! This is the core for every tdo application
#![deny(missing_docs, unsafe_code,
        missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unused_import_braces, unused_qualifications)]
#![warn(missing_debug_implementations)]
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate error_chain;
extern crate serde_json;
extern crate json;

pub mod tdo;
pub mod list;
pub mod todo;
pub mod error;
mod legacy;