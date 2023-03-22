use std::collections::HashMap;

pub type PropsIterator<'a> = std::str::SplitInclusive<'a, [char; 3]>;
pub type ComponentsIterator<'a> = std::str::Split<'a, char>;

pub trait Content {
    fn get_source(&self) -> &str;

    /// Return a cleaned str (without '\n', whitespaces, '<', '>' & '/' symbols)
    fn clean_symbols(&self) -> &str;

    /// Split the content into small pieces that represents key-value props
    fn split_by_props(&self) -> PropsIterator;

    /// Split the content into small pieces that represents components
    fn split_by_components(&self) -> ComponentsIterator;
}

pub type ReactContent<'a> = &'a str;

impl<'a> Content for ReactContent<'a> {
    fn get_source(&self) -> &str {
        self
    }

    fn clean_symbols(&self) -> &str {
        let pat: &[_] = &['\n', ' ', '<', '>', '/'];
        self.trim_matches(pat)
    }

    fn split_by_props(&self) -> PropsIterator {
        self.split_inclusive([' ', '>', '\n'])
    }

    fn split_by_components(&self) -> ComponentsIterator {
        self.split('<')
    }
}

type Prop = (String, Option<String>);

#[derive(Debug, PartialEq)]
pub struct Component<'a> {
    pub name: String,
    pub typo: ComponentType,
    pub content: ReactContent<'a>,
    props: HashMap<String, Prop>,
}

impl<'a> Component<'a> {
    pub fn new(name: String, content: ReactContent<'a>) -> Self {
        let typo = ComponentType::from(&name);

        Self {
            name,
            typo,
            content,
            props: HashMap::new(),
        }
    }

    pub fn from(content: ReactContent<'a>) -> Self {
        let mut f = content.split_by_props();

        let name: ReactContent = f.next().unwrap();
        let mut component = Self::new(name.clean_symbols().to_string(), content);

        // Avoid the rest of the process if it is an empty component
        if name.ends_with('>') {
            return component;
        }

        //
        'props_loop: while let Some(prop) = f.next() {
            let mut prop_c = match prop.trim() {
                "" => continue 'props_loop,
                ">" | "/>" => break 'props_loop,
                _ => String::from(prop),
            };

            // Special treatment for multiline literal strings (not closed)
            if prop.matches('`').count() % 2 != 0 {
                'inner: while let Some(prop_t) = f.next() {
                    prop_c.push_str(prop_t);

                    if prop_t.contains('`') {
                        break 'inner;
                    }
                }
            }

            prop_c = (&prop_c as ReactContent).clean_symbols().to_string();

            if let Some((name, value)) = prop_c.split_once('=') {
                component.add_prop((name.to_string(), Some(value.to_string())));
            } else {
                component.add_prop((prop_c, None));
            }

            // Break the loop if component ends its props declaration
            if prop.ends_with('>') {
                break 'props_loop;
            }
        }

        // dbg!(&component);
        component
    }

    pub fn add_prop(self: &mut Self, (name, value): Prop) -> &mut Self {
        self.props.insert(name.clone(), (name, value));
        self
    }

    /* pub fn get_prop(self: &Self, name: &str) -> Option<&Prop> {
        self.props.get(name)
    } */

    /// If prop exists, return a prop in a "name=value" string format.
    pub fn get_raw_prop(self: &Self, name: &str) -> Option<String> {
        if let Some((name, value)) = self.props.get(name) {
            if let Some(v) = value {
                Some(format!("{}={}", name, v))
            } else {
                Some(format!("{}", name))
            }
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ComponentType {
    HtmlElement,
    ReactComponent,
    None,
}

impl ComponentType {
    pub fn from(name: &str) -> Self {
        match name.chars().nth(0) {
            None => Self::None,
            Some(first) => {
                if first.is_uppercase() {
                    Self::ReactComponent
                } else {
                    Self::HtmlElement
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_content_trait() {
        let react_content: ReactContent = "const i = 'test';";
        assert_eq!(react_content, react_content.get_source());
    }

    #[test]
    fn clean_content_with_several_symbols() {
        let component: ReactContent = " \n<span> \n";
        let component_cleaned = component.clean_symbols();
        assert_eq!(component_cleaned, "span");
    }

    #[test]
    fn clean_no_applies_to_cleaned_str() {
        let component: ReactContent = "span";
        let component_cleaned = component.clean_symbols();
        assert_eq!(component_cleaned, component);
    }

    #[test]
    fn clean_no_applies_to_cleaned_str_with_intermediate_symbols() {
        let component: ReactContent = "spa<>n";
        let component_cleaned = component.clean_symbols();
        assert_eq!(component_cleaned, component);
    }

    #[test]
    fn creation_from_an_empty_prop_component_str() {
        let component_str: ReactContent = "<h1>";
        let component = Component::from(component_str);

        assert_eq!(component.name, "h1");
        assert_eq!(component.typo, ComponentType::HtmlElement);
        assert_eq!(component.props.len(), 0);
        assert_eq!(component.content, component_str);
    }

    #[test]
    fn creation_from_an_empty_prop_react_component_str() {
        let component_str: ReactContent = "<MyComponent>";
        let component = Component::from(component_str);

        assert_eq!(component.name, "MyComponent");
        assert_eq!(component.typo, ComponentType::ReactComponent);
        assert_eq!(component.props.len(), 0);
        assert_eq!(component.content, component_str);
    }

    #[test]
    fn creation_from_an_empty_prop_selfclosed_component_str() {
        let component_str: ReactContent = "<h1/>";
        let component = Component::from(component_str);

        assert_eq!(component.name, "h1");
        assert_eq!(component.typo, ComponentType::HtmlElement);
        assert_eq!(component.props.len(), 0);
        assert_eq!(component.content, component_str);
    }

    #[test]
    fn creation_from_an_empty_prop_selfclosed_component_str_with_some_trash_text() {
        let component_str: ReactContent = "<h1 />{trash} trash text";
        let component = Component::from(component_str);

        assert_eq!(component.name, "h1");
        assert_eq!(component.typo, ComponentType::HtmlElement);
        assert_eq!(component.props.len(), 0);
        assert_eq!(component.content, component_str);
    }

    #[test]
    fn creation_from_a_one_prop_component_str() {
        let component_str: ReactContent = "<span aria-checked=\"true\">";
        let component = Component::from(component_str);

        assert_eq!(component.name, "span");
        assert_eq!(component.typo, ComponentType::HtmlElement);
        assert_eq!(component.props.len(), 1);
        assert_eq!(component.props.contains_key("aria-checked"), true);
        assert_eq!(
            component
                .props
                .get("aria-checked")
                .map(|(_, value)| value)
                .and_then(|value| value.as_ref()),
            Some(&"\"true\"".to_string())
        );
        assert_eq!(component.content, component_str);
    }

    #[test]
    fn creation_from_a_multi_prop_component_str() {
        let component_str: ReactContent =
            "<div id=\"myDiv\" className={myVariable} href=\"#string\">";
        let component = Component::from(component_str);

        assert_eq!(component.name, "div");
        assert_eq!(component.typo, ComponentType::HtmlElement);
        assert_eq!(component.props.len(), 3);
        assert_eq!(component.props.contains_key("id"), true);
        assert_eq!(component.props.contains_key("className"), true);
        assert_eq!(component.props.contains_key("href"), true);
        assert_eq!(
            component
                .props
                .get("className")
                .map(|(_, value)| value)
                .and_then(|value| value.as_ref()),
            Some(&"{myVariable}".to_string())
        );
        assert_eq!(component.content, component_str);
    }

    #[test]
    fn creation_from_a_multi_prop_component_str_splitted_in_several_lines() {
        let component_str: ReactContent = "<div id=\"myDiv\"\n    href=\"#string\"\n    >";
        let component = Component::from(component_str);

        assert_eq!(component.name, "div");
        assert_eq!(component.typo, ComponentType::HtmlElement);
        assert_eq!(component.props.len(), 2);
        assert_eq!(component.props.contains_key("id"), true);
        assert_eq!(component.props.contains_key("href"), true);
        assert_eq!(
            component
                .props
                .get("href")
                .map(|(_, value)| value)
                .and_then(|value| value.as_ref()),
            Some(&"\"#string\"".to_string())
        );
        assert_eq!(component.content, component_str);
    }

    #[test]
    fn creation_from_a_multi_prop_component_str_splitted_in_several_lines_with_template_prop() {
        let component_str: ReactContent =
            "<div data-testid={`\n    ${dataTestId}-arrow\n   `}\n id=\"myDiv\"\n   >";
        let component = Component::from(component_str);

        assert_eq!(component.name, "div");
        assert_eq!(component.typo, ComponentType::HtmlElement);
        assert_eq!(component.props.len(), 2);
        assert_eq!(component.props.contains_key("data-testid"), true);
        assert_eq!(component.props.contains_key("id"), true);
        assert_eq!(
            component
                .props
                .get("data-testid")
                .map(|(_, value)| value)
                .and_then(|value| value.as_ref()),
            Some(&"{`\n    ${dataTestId}-arrow\n   `}".to_string())
        );
        assert_eq!(component.content, component_str);
    }

    #[test]
    fn creation_from_an_empty_prop_with_children_component_str() {
        let component_str: ReactContent = "<span>{myChildren}";
        let component = Component::from(component_str);

        assert_eq!(component.name, "span");
        assert_eq!(component.typo, ComponentType::HtmlElement);
        assert_eq!(component.props.len(), 0);
        assert_eq!(component.content, component_str);
    }

    #[test]
    fn creation_from_a_truthy_boolean_prop_component_str() {
        let component_str: ReactContent = "<span disabled>";
        let component = Component::from(component_str);

        assert_eq!(component.name, "span");
        assert_eq!(component.typo, ComponentType::HtmlElement);
        assert_eq!(component.props.len(), 1);
        assert_eq!(component.props.contains_key("disabled"), true);
        assert_eq!(
            component
                .props
                .get("disabled")
                .map(|(_, value)| value)
                .and_then(|value| value.as_ref()),
            None
        );
        assert_eq!(component.content, component_str);
    }
}
