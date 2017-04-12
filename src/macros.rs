
use colored::*;

#[macro_export]
macro_rules! errorprint {
 ($e:expr) => {
     println!("{} {}", "[Error]".red(), $e);
 };
}
