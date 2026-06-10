#![allow(missing_docs)]
use std::fs::File;
use crate::tdo::Tdo;
use crate::list::TodoList;
use crate::error::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tdo01 {
    pub lists: Vec<TodoList>,
    version: String,
}

impl Tdo01 {
    pub fn load(path: &str) -> TdoResult<Tdo01> {
        match File::open(path) {
            Ok(file) => {
                match serde_json::from_reader(&file) {
                    Ok(tdo) => Ok(tdo),
                    Err(_) => {
                        Err(ErrorKind::StorageError(storage_error::ErrorKind::FileCorrupted).into())
                    }
                }
            }
            Err(_) => Err(ErrorKind::StorageError(storage_error::ErrorKind::FileNotFound).into()),
        }
    }
}

impl Into<Tdo> for Tdo01 {
    fn into(self) -> Tdo {
        println!("Got it here");
        let mut new_tdo = Tdo::new();
        new_tdo.lists = self.lists;
        new_tdo
    }
}
