use std::path::PathBuf;

use react_props_replacer::{Action, ConfigBuilder};

mod integration;
use integration::files::*;

mod config {
    use super::*;

    #[test]
    fn create_config_from_args_with_valid_data() {
        let result = ConfigBuilder::new()
            .parse_args(&[FILE_HTML_DATATESTID, "--remove-prop"].map(|v| v.to_string()))
            .build();
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.output, PathBuf::from(FILE_HTML_DATATESTID_UPDATED));

        let result = ConfigBuilder::new()
            .parse_args(
                &[
                    FILE_HTML_DATATESTID,
                    "--remove-prop",
                    "--output",
                    FILE_HTML_DATATESTID_UPDATED,
                ]
                .map(|v| v.to_string()),
            )
            .build();
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.output, PathBuf::from(FILE_HTML_DATATESTID_UPDATED));
        assert_eq!(
            config.actions.first().unwrap(),
            &Action::RemoveProp("data-testid".to_string())
        );
    }

    #[test]
    fn create_config_from_empty_args() {
        let result = ConfigBuilder::new().parse_args(&[]).build();
        assert!(result.is_err());
    }

    #[test]
    fn create_config_from_args_with_invalid_input() {
        let result = ConfigBuilder::new()
            .parse_args(&[FILE_NOT_EXISTS, "--remove-prop"].map(|v| v.to_string()))
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn create_config_from_args_with_invalid_output() {
        let result = ConfigBuilder::new()
            .parse_args(
                &[
                    FILE_HTML_DATATESTID,
                    "--remove-prop",
                    "--output",
                    ASSETS_FOLDER,
                ]
                .map(|v| v.to_string()),
            )
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn create_config_from_args_without_actions() {
        let result = ConfigBuilder::new()
            .parse_args(&[FILE_HTML_DATATESTID].map(|v| v.to_string()))
            .build();
        assert!(result.is_err());
    }
}
