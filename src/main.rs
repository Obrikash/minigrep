use minigrep::Config;
use std::{env, process};
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem pasring arguments: {err}");
        process::exit(1);
    });

    if let Err(er) = minigrep::run(config) {
        eprintln!("Application error: {er}");
        process::exit(1);
    }
}
