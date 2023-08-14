mod cli;
mod misc;
mod tt_lib;
mod types;
fn main() {
    match cli::run() {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}
