use std::{env, process};

use aoc_2021::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing CLI arguments: {err}");
        process::exit(1);
    });

    match aoc_2021::run(config) {
        Ok(_) => (),
        Err(e) => {
            println!("Error executing command: {e}");
            process::exit(1);
        }
    }
}
