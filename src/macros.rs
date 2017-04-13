#[macro_export]
macro_rules! errorprint {
 ($e:expr) => {
     println!("{} {}", "error:".red().bold(), $e);
 };
}
