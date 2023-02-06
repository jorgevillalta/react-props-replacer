mod component;
mod config;

use std::{borrow::Cow, error::Error, fs};

use component::{Component, ComponentType};
pub use config::{Action, Config, ConfigBuilder};

///
pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    // Read the file as a string
    let content = fs::read_to_string(&config.input).expect("Error reading input file.");

    //
    let content_updated = apply_content_actions(&content, &config.actions);

    //
    fs::write(&config.output, content_updated).expect("Error writing output file.");

    println!("Process completed successfully");

    Ok(())
}

///
fn apply_content_actions(content: &str, actions: &[Action]) -> String {
    let mut content_by_component = content.split('<'); // TODO try with split_inclusive

    let mut content_by_component_updated: Vec<Cow<'_, str>> =
        vec![Cow::Borrowed(content_by_component.next().unwrap())];
    let mut component_vector: Vec<Component> = vec![];

    //
    while let Some(raw_component) = content_by_component.next() {
        // Ignore 'component-lines' with element close char
        if raw_component.trim_start().starts_with('/') {
            content_by_component_updated.push(Cow::Borrowed(raw_component));
            continue;
        }

        let component = Component::from(raw_component);
        let mut source_updated = Cow::Borrowed(raw_component);

        // Apply actions
        for action in actions {
            match action {
                Action::RemoveProp(prop_name) => {
                    let raw_prop = component.get_raw_prop(&prop_name);

                    if component.typo == ComponentType::HtmlElement && raw_prop.is_some() {
                        source_updated = Cow::Owned(source_updated.replace(&raw_prop.unwrap(), ""));
                    }
                }
            }
        }

        //
        content_by_component_updated.push(source_updated);

        // Only for tracking purposes
        component_vector.push(component);
    }

    let content_updated = content_by_component_updated.join("<");

    println!(
        "\n------------------\nSUMMARY: \n\t* Number of components found: {}",
        component_vector.len()
    );

    // dbg!(component_vector);

    content_updated
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_with_remove_action_one_line_content_wo_effect() {
        let content = "import styles from './styles.scss';";
        let content_result =
            apply_content_actions(content, &[Action::RemoveProp("data-testid".to_string())]);
        assert_eq!(content, content_result);
    }

    #[test]
    fn update_with_remove_action_one_line_content() {
        let content = "import styles from './styles.scss'; function MyComponent { return (<div><span/><h1 data-testid='test_id' >Main title</h1></div>);}";
        let content_result =
            apply_content_actions(content, &[Action::RemoveProp("data-testid".to_string())]);
        assert_ne!(content, content_result);
        assert_eq!(content_result.matches("data-testid").count(), 0);
    }

    #[test]
    fn update_with_remove_action_multi_line_content() {
        let content = "import styles from './styles.scss';\n\n function MyComponent {\n  return (\n    <div>      <span/>\n      <h1 data-testid='test_id' >Main title</h1>\n    </div>);}\n";
        let content_result =
            apply_content_actions(content, &[Action::RemoveProp("data-testid".to_string())]);
        assert_ne!(content, content_result);
        assert_eq!(content_result.matches("data-testid").count(), 0);
    }

    #[test]
    fn update_with_remove_action_multi_line_and_multi_prop_content() {
        let content = "import styles from './styles.scss';\n\n function MyComponent {\n  return (\n    <div>      <span id='span_id' data-testid='span_test_id'/>\n      <h1 data-testid='h1_test_id' >Main title</h1>\n    </div>);}\n";
        let content_result =
            apply_content_actions(content, &[Action::RemoveProp("data-testid".to_string())]);
        assert_ne!(content, content_result);
        assert_eq!(content_result.matches("data-testid").count(), 0);
        assert_eq!(content_result.contains("<span id='span_id' />"), true);
        assert_eq!(content_result.contains("<h1  >Main title</h1>"), true);
    }

    #[test]
    fn update_with_remove_action_multi_line_and_react_component_content_wo_effect() {
        let content = "import styles from './styles.scss';\n\n function MyComponent {\n  return (\n    <div>      <span id='span_id'/>\n      <h1>Main title</h1>\n      <MyOtherComponent data-testid='other_test_id' />\n    </div>);}\n";
        let content_result =
            apply_content_actions(content, &[Action::RemoveProp("data-testid".to_string())]);
        assert_eq!(content, content_result);
    }
}
