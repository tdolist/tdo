use std::path::PathBuf;
use std::env;
use std::fs::DirBuilder;
use std::error::Error;
use std::fmt;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
pub enum ValidationError {
    UserAbort,
    TargetIsADir,
    EnvError(io::Error),
    BuildDirErr(io::Error),
}

impl Error for ValidationError {
    fn description(&self) -> &str {
        match *self {
            ValidationError::UserAbort => "Abort by user.",
            ValidationError::TargetIsADir => "The selected target is a directory, not a file.",
            ValidationError::EnvError(ref err) => err.description().clone(),
            ValidationError::BuildDirErr(ref err) => err.description().clone(),
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}


fn ask_user(question: &str) -> bool {
    let mut answer = String::new();
    loop {
        print!("{} (y/n) ", question);
        io::stdout().flush().ok().expect("Could not flush stdout");
        io::stdin().read_line(&mut answer).unwrap();
        match answer.trim() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => continue,
        };
    }
}

pub fn validate_target_file(target_path: &str) -> Result<PathBuf, ValidationError> {
    let mut path = PathBuf::from(target_path);

    // make sure the target directory is absolute
    if path.is_relative() {
        // get the current working directory and adoin the provided relative path onto it
        // potential environment errors are reported back as specialized error type
        let cwd = match env::current_dir() {
            Ok(dir) => dir,
            Err(e) => return Err(ValidationError::EnvError(e).into()),
        };

        path = cwd.join(path);
    }


    if path.exists() {
        if path.is_file() {
            // The path exists and is a file.
            Ok(path.to_owned())
        } else {
            Err(ValidationError::TargetIsADir.into())
        }
    } else {
        // check whether the parent directory exists. If so, create the target directory
        // if not, ask the user if he's sure that he wants to create the directory recursively
        let parent_dir = path.parent().unwrap();
        if parent_dir.exists() {
            Ok(path.to_owned())
        } else {
            if ask_user("The path to the target directory does not exist. Create it?") {
                let _ = match DirBuilder::new().recursive(true).create(parent_dir) {
                    Ok(b) => b,
                    Err(err) => return Err(ValidationError::BuildDirErr(err).into()),
                };
                Ok(path.to_owned())
            } else {
                Err(ValidationError::UserAbort.into())
            }
        }
    }
}
