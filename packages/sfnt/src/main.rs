mod cli;
mod misc;
mod tt_lib;
mod types;

#[macro_use]
extern crate structure;
fn main() {
    match cli::run() {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}
