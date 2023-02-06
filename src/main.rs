// use std::fs;
use std::{env, io::Error, process};

use react_props_replacer::ConfigBuilder;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    let config = ConfigBuilder::new()
        .parse_args(&args)
        .build()
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });

    if let Err(e) = react_props_replacer::run(&config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

    Ok(())
}
