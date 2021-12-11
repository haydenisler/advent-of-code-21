use std::env;
use std::process;

use day_2_2::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("filename: {}", config.filename);
    if let Err(e) = day_2_2::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
