use std::{
    fs,
    path::{Path, PathBuf},
};

use react_props_replacer::{run, Action, Config, ConfigBuilder};

mod integration;
use integration::files::*;

mod runner {
    use super::*;

    fn buil_config(file_path: &str) -> Config {
        ConfigBuilder::new()
            .input(PathBuf::from(file_path))
            .add_action(Action::RemoveProp("data-testid".to_string()))
            .build()
            .expect("Config error")
    }

    fn prepare_file(file_path: &str) {
        if Path::new(file_path).exists() {
            fs::remove_file(file_path).unwrap();
        }
    }

    #[test]
    fn run_config_with_valid_files() {
        // File with html components
        prepare_file(FILE_HTML_DATATESTID_UPDATED);

        let config = buil_config(FILE_HTML_DATATESTID);
        let result = run(&config);
        assert!(result.is_ok(), "runner process throws an error");

        let result_file_str = fs::read_to_string(FILE_HTML_DATATESTID_UPDATED).unwrap();
        let expected_file_str = fs::read_to_string(FILE_HTML_DATATESTID_ORIGINAL_UPDATED).unwrap();
        assert_eq!(result_file_str, expected_file_str);

        prepare_file(FILE_HTML_DATATESTID_UPDATED);

        // File with react components
        prepare_file(FILE_REACT_DATATESTID_UPDATED);

        let config = buil_config(FILE_REACT_DATATESTID);
        let result = run(&config);
        assert!(result.is_ok(), "runner process throws an error");

        let result_file_str = fs::read_to_string(FILE_REACT_DATATESTID_UPDATED).unwrap();
        let expected_file_str = fs::read_to_string(FILE_REACT_DATATESTID_ORIGINAL_UPDATED).unwrap();
        assert_eq!(result_file_str, expected_file_str);

        prepare_file(FILE_REACT_DATATESTID_UPDATED);
    }
}
