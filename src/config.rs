use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub enum Action {
    RemoveProp(String),
}

#[derive(Debug)]
pub struct Config {
    pub input: PathBuf,
    pub output: PathBuf,
    pub actions: Vec<Action>,
}

#[derive(Debug)]
pub struct ConfigBuilder {
    input: Option<PathBuf>,
    output: Option<PathBuf>,
    actions: Vec<Action>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            input: None,
            output: None,
            actions: vec![],
        }
    }

    pub fn add_action(mut self, action: Action) -> Self {
        self.actions.push(action);
        self
    }

    pub fn input(mut self, file: PathBuf) -> Self {
        self.input = Some(file);
        self
    }

    pub fn output(mut self, file: PathBuf) -> Self {
        self.output = Some(file);
        self
    }

    pub fn parse_args(mut self, args: &[String]) -> Self {
        if args.len() > 1 {
            self.input = Some(PathBuf::from(&args[1]));
        }

        let args_rest = args[2..].join(" ");
        let options = args_rest.split("--");

        for option in options {
            match option.trim_end() {
                opt if opt.starts_with("output") => {
                    self.output = if let Some((_, file)) = opt.split_once(' ') {
                        Some(PathBuf::from(file))
                    } else {
                        None
                    };
                }
                opt if opt.starts_with("remove-prop") => {
                    self.actions.push(Action::RemoveProp(parse_value_to_string(
                        opt,
                        "data-testid",
                    )));
                }
                _ => (),
            }
        }

        self
    }

    pub fn build(self: Self) -> Result<Config, &'static str> {
        if self.input.is_none() {
            return Err("Input file is not present.");
        }

        if self.actions.is_empty() {
            return Err("Actions are not present.");
        }

        let input = self.input.unwrap();

        if !input.exists() {
            return Err("Input file not exists.");
        }

        if input.is_dir() {
            return Err("Input is not a file.");
        }

        let output = self.output.unwrap_or_else(|| {
            let extension = input
                .extension()
                .and_then(|x| x.to_str())
                .map(|x| format!("updated.{x}"))
                .unwrap_or("".to_string());

            input.with_extension(extension)
        });

        if output.is_dir() {
            return Err("Output is not a file.");
        }

        Ok(Config {
            input,
            output,
            actions: self.actions,
        })
    }
}

fn parse_value_to_string(value: &str, default_value: &str) -> String {
    if let Some((_, value)) = value.split_once(' ') {
        value.to_string()
    } else {
        default_value.to_string()
    }
}
