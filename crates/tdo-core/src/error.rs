//! Custom Error Types for errors that may occur when handling todos (or lists).

/// Custom Result Type for tdo.
///
/// This abbreviation is introduced since many functions throughout the crate return
/// this type of result, which bundles all possible errors of the `tdo_core` crate.
pub type TdoResult<T> = Result<T>;


/// Errors that can arise when working with todo lists.
pub mod todo_error {
    error_chain! {
        errors {
            /// The requested item is not in the list.
            NotInList {
                description("Todo is not in this list")
            }
            /// The requested todo list does not exist.
            NoSuchList {
                description("No such list")
            }
            /// The default list is tried to be removed.
            CanNotRemoveDefault {
                description("The default list can no be removed")
            }
            /// A list with the same name already exists.
            NameAlreadyExists {
                description("There already exists a list with this name")
            }
            /// A todo with the same ID already exists.
            IDAlreadyExists {
                description("There already exists a todo with this ID")
            }
        }
    }
}

/// Errors that can arise when interacting with github.
pub mod github_error {
    error_chain! {
        errors {
            /// Repository does not exist
            DoesNotExist {
                description("Repository does not exist or you have no access to it")
            }
            /// Bad credentials
            BadCredentials {
                description("Bad credentials")
            }
            /// Not allowed to move error
            NotAllowedToMove {
                description("A github issue is not allowed to be moved out ouf the default list")
            }
            /// Not a guthub issue eroor
            NoIssueAsigned {
                description("There is no github issue asigned to this todo")
            }
            /// Unknown error
            UnknownError {
                description("An unknown error occured")
            }
        }
    }
}

/// The Errors that may occur while interacting with the file system.
pub mod storage_error {
    error_chain! {
        errors {
            /// The accessed file is corrupted. This is most likely
            /// because someone edited the JSON file manually.
            FileCorrupted {
                description("File is corrupted")
            }
            /// The data could not be written to the file.
            SaveFailure {
                description("File could not be saved")
            }
            /// The requested file could not be found.
            FileNotFound {
                description("File was not found")
            }
            /// The conversion of an older format failed.
            UnableToConvert {
                description("File could not be converted automatically")
            }
        }
    }
}

error_chain! {
    links {
        TodoError(todo_error::Error, todo_error::ErrorKind) #[doc = "An error within the tdo data structures occured."];
        StorageError(storage_error::Error, storage_error::ErrorKind) #[doc = "A storage-related error occured while interacting with the file system."];
        GithubError(github_error::Error, github_error::ErrorKind) #[doc = "A github communication-related error occured."];
    }
}
