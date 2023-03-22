mod component;
mod config;
mod report;

use std::{borrow::Cow, error::Error, fs};

use component::{Component, ComponentType, Content, ReactContent};
pub use config::{Action, Config, ConfigBuilder};
pub use report::ContentReport;

///
pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    // Read the file as a string
    let content = fs::read_to_string(&config.input).expect("Error reading input file.");

    //
    println!("📁 Processing file: {}...", &config.input.display());
    let content_updated = apply_content_actions(&content, &config.actions);

    //
    fs::write(&config.output, content_updated).expect("Error writing output file.");

    println!("✨ Process completed successfully 🎉🎉");

    Ok(())
}

///
fn apply_content_actions(content: ReactContent, actions: &[Action]) -> String {
    let mut content_by_component = content.split_by_components();

    let mut content_updated = vec![content_by_component.next().unwrap().to_string()];
    let mut component_vector: Vec<Component> = vec![];
    let mut content_report = ContentReport::new();

    //
    while let Some(raw_component) = content_by_component.next() {
        // Ignore 'component-lines' with element close char
        if raw_component.trim_start().starts_with('/') {
            content_updated.push(raw_component.to_string());
            continue;
        }

        let component = Component::from(raw_component);
        let mut is_updated_component = false;
        let mut updated_component = String::from(raw_component);

        // Apply actions
        for action in actions {
            match action {
                Action::RemoveProp(prop_name) => {
                    let raw_prop = component.get_raw_prop(&prop_name);

                    if component.typo == ComponentType::HtmlElement && raw_prop.is_some() {
                        updated_component = updated_component.replace(&raw_prop.unwrap(), "");
                        is_updated_component = true;
                    }
                }
            }
        }

        //
        content_updated.push(updated_component);

        // This component has been updated
        /*         if is_updated_component {
            content_report.add_replacement(0, raw_component, content_updated.last);
        } */

        // Only for tracking purposes
        component_vector.push(component);
    }

    println!("{}", &content_report);

    // dbg!(component_vector);

    content_updated.join("<")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_with_remove_action_one_line_content_wo_effect() {
        let content: ReactContent = "import styles from './styles.scss';";
        let content_result =
            apply_content_actions(content, &[Action::RemoveProp("data-testid".to_string())]);
        assert_eq!(content, content_result);
    }

    #[test]
    fn update_with_remove_action_one_line_content() {
        let content: ReactContent = "import styles from './styles.scss'; function MyComponent { return (<div><span/><h1 data-testid='test_id' >Main title</h1></div>);}";
        let content_result =
            apply_content_actions(content, &[Action::RemoveProp("data-testid".to_string())]);
        assert_ne!(content, content_result);
        assert_eq!(content_result.matches("data-testid").count(), 0);
    }

    #[test]
    fn update_with_remove_action_multi_line_content() {
        let content: ReactContent = "import styles from './styles.scss';\n\n function MyComponent {\n  return (\n    <div>      <span/>\n      <h1 data-testid='test_id' >Main title</h1>\n    </div>);}\n";
        let content_result =
            apply_content_actions(content, &[Action::RemoveProp("data-testid".to_string())]);
        assert_ne!(content, content_result);
        assert_eq!(content_result.matches("data-testid").count(), 0);
    }

    #[test]
    fn update_with_remove_action_multi_line_and_multi_prop_content() {
        let content: ReactContent = "import styles from './styles.scss';\n\n function MyComponent {\n  return (\n    <div>      <span id='span_id' data-testid='span_test_id'/>\n      <h1 data-testid='h1_test_id' >Main title</h1>\n    </div>);}\n";
        let content_result =
            apply_content_actions(content, &[Action::RemoveProp("data-testid".to_string())]);
        assert_ne!(content, content_result);
        assert_eq!(content_result.matches("data-testid").count(), 0);
        assert_eq!(content_result.contains("<span id='span_id' />"), true);
        assert_eq!(content_result.contains("<h1  >Main title</h1>"), true);
    }

    #[test]
    fn update_with_remove_action_multi_line_and_react_component_content_wo_effect() {
        let content: ReactContent = "import styles from './styles.scss';\n\n function MyComponent {\n  return (\n    <div>      <span id='span_id'/>\n      <h1>Main title</h1>\n      <MyOtherComponent data-testid='other_test_id' />\n    </div>);}\n";
        let content_result =
            apply_content_actions(content, &[Action::RemoveProp("data-testid".to_string())]);
        assert_eq!(content, content_result);
    }
}
