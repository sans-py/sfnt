mod cli;
mod types;
fn main() {
    match cli::run() {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}
