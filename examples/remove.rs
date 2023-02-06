use std::io::Error;
use std::path::PathBuf;
use std::process;

use react_props_replacer::{Action, ConfigBuilder};

fn main() -> Result<(), Error> {
    /*  let config = ConfigBuilder::new(
        PathBuf::from("./examples/files/sample_1.jsx"),
        vec![Action::RemoveProp("data-testid".to_string())],
        None,
    ); */
    let config = ConfigBuilder::new()
        .input(PathBuf::from("./examples/assets/sample_1.jsx"))
        .add_action(Action::RemoveProp("data-testid".to_string()))
        .build()
        .expect("Config error");

    if let Err(e) = react_props_replacer::run(&config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

    Ok(())
}
