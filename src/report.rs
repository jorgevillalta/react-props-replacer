use std::fmt::Display;

use crate::component::ReactContent;

#[derive(Debug, PartialEq)]
pub struct Replacement<'a, 'b> {
    line: u32,
    before: ReactContent<'a>,
    after: ReactContent<'b>,
}

#[derive(Debug)]
pub struct ContentReport<'a, 'b> {
    replaced_elements: Vec<Replacement<'a, 'b>>,
}

impl<'a, 'b> ContentReport<'a, 'b> {
    pub fn new() -> Self {
        Self {
            replaced_elements: vec![],
        }
    }

    pub fn add_replacement<'c>(
        &'c mut self,
        line: u32,
        before: ReactContent<'a>,
        after: ReactContent<'b>,
    ) -> &'c mut Self {
        let replacement = Replacement {
            line,
            before,
            after,
        };
        self.replaced_elements.push(replacement);
        self
    }
}

impl<'a, 'b> Display for ContentReport<'a, 'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "🔧 Elements replaced: {}", self.replaced_elements.len())?;
        for rep in self.replaced_elements.iter() {
            writeln!(f, "✅ Element in line: {}", rep.line)?;
            writeln!(f, "  ⏩ Before: {}", rep.before)?;
            writeln!(f, "  ⏪ After: {}", rep.after)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_all_the_info() {
        let before: ReactContent = "<p data-testid={testId} class='strong'>";
        let after: ReactContent = "<p  class='strong'>";
        let replacement = Replacement {
            line: 10,
            before,
            after,
        };
        let report = ContentReport {
            replaced_elements: vec![replacement],
        };

        let result = format!("{}", report);

        assert!(result.contains("Elements replaced: 1"));
        assert!(result.contains("Element in line: 10"));
        assert!(result.contains(before));
        assert!(result.contains(after));
    }

    #[test]
    fn create_with_new() {
        let before: ReactContent = "<p data-testid={testId} class='strong'>";
        let after: ReactContent = "<p  class='strong'>";

        let mut report = ContentReport::new();
        report.add_replacement(10, before, after);

        assert!(report.replaced_elements.contains(&Replacement {
            line: 10,
            before,
            after
        }));
    }
}
