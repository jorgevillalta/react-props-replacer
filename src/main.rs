// use std::fs;
use std::{env, io::Error, process};

use react_props_replacer::ConfigBuilder;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    // On empty args, print command help & exit
    if args.len() == 1 {
        println!("A CLI application to smart replace props in React files.\n\nUsage: react-props-replacer source_file [--output output_file] [--remove-prop [prop_name | \"data-testid\"]]");
        return Ok(());
    }

    let config = ConfigBuilder::new()
        .parse_args(&args[1..])
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
