use lib_ruby_parser::Loc;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Constant {
    pub path: PathBuf,
    pub scope: Option<String>,
    pub name: String,
    pub loc: Loc,
}

#[derive(Debug, Clone)]
pub struct Reference {
    pub path: PathBuf,
    pub scope: Option<String>,
    pub name: String,
    pub loc: Loc,
}

#[derive(Debug, Clone)]
pub struct Definition {
    pub path: PathBuf,
    pub scope: Option<String>,
    pub name: String,
    pub loc: Loc,
}

impl From<Constant> for Reference {
    fn from(constant: Constant) -> Self {
        Reference {
            path: constant.path,
            scope: constant.scope,
            name: constant.name,
            loc: constant.loc,
        }
    }
}

impl From<Constant> for Definition {
    fn from(constant: Constant) -> Self {
        Definition {
            path: constant.path,
            scope: constant.scope,
            name: constant.name,
            loc: constant.loc,
        }
    }
}

impl Reference {
    pub fn nestings(&self) -> Vec<String> {
        let mut nestings = Vec::new();

        let unwrapped_scope = self.scope.clone().unwrap_or_else(|| "".to_string());
        let mut remaining_parts: Vec<&str> = unwrapped_scope.split("::").collect();

        while let Some(nesting_part) = remaining_parts.pop() {
            let mut parts: Vec<&str> = remaining_parts.clone();
            parts.push(nesting_part);
            parts.push(&self.name);
            nestings.push(parts.join("::"));
        }

        nestings.push(self.name.clone());
        nestings
    }
}

impl Definition {
    pub fn qualified(&self) -> String {
        qualified(&self.scope, &self.name)
    }
}

fn qualified(scope: &Option<String>, name: &str) -> String {
    if let Some(scope) = scope {
        format!("{}::{}", scope, name)
    } else {
        name.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use super::{Constant, Definition, Reference};
    use lib_ruby_parser::Loc;

    fn constant() -> Constant {
        Constant {
            path: PathBuf::from_str("./fixtures/nested_classes.rb").unwrap(),
            scope: Some("A::B::C".to_owned()),
            name: "InC".to_owned(),
            loc: Loc { begin: 0, end: 10 },
        }
    }

    #[test]
    fn test_qualified() {
        let definition: Definition = constant().into();
        assert_eq!(definition.qualified(), "A::B::C::InC".to_owned());
    }

    #[test]
    fn test_nestings() {
        let reference: Reference = constant().into();
        assert_eq!(reference.nestings(), vec!["A::B::C::InC", "A::B::InC", "A::InC", "InC"].to_owned());
    }
}
