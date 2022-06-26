use std::path::{Path, PathBuf};

use lib_ruby_parser::{
    nodes::{self},
    traverse::visitor,
    Loc, Node,
};

use super::constant::Constant;

pub struct Visitor {
    pub path: PathBuf,
    pub definitions: Vec<Constant>,
    pub references: Vec<Constant>,
}

impl Visitor {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_owned(),
            definitions: Vec::new(),
            references: Vec::new(),
        }
    }
}

fn fetch_const_name(name: &Node) -> String {
    match name {
        Node::Const(node) => fetch_const_const_name(node),
        other => panic!("Encountered an unexpected node type: '{:?}'", other),
    }
}

fn fetch_const_loc(name: &Node) -> Loc {
    match name {
        Node::Const(node) => node.expression_l,
        other => panic!("Encountered an unexpected node type: '{:?}'", other),
    }
}

fn fetch_casn_const_name(node: &nodes::Casgn) -> String {
    if let Some(scope) = &node.scope {
        format!("{}::{}", fetch_const_scope_name(scope), node.name)
    } else {
        node.name.to_owned()
    }
}

fn fetch_const_const_name(node: &nodes::Const) -> String {
    if let Some(scope) = &node.scope {
        format!("{}::{}", fetch_const_scope_name(scope), node.name)
    } else {
        node.name.to_owned()
    }
}

fn fetch_const_scope_name(scope: &nodes::Node) -> String {
    match scope {
        Node::Cbase(_) | Node::Self_(_) | Node::Send(_) | Node::Lvar(_) | Node::Ivar(_) => "".to_owned(),
        Node::Const(_) => fetch_const_name(scope),
        Node::Casgn(_) => fetch_const_name(scope),
        other => panic!("Don't know how to fetch const name from {:?}", other),
    }
}

fn nest_constants(path: &Path, parent_name: &str, child_constants: Vec<Constant>) -> Vec<Constant> {
    let mut constants = Vec::new();

    for child_constant in child_constants {
        let scope = if let Some(scope) = child_constant.scope {
            format!("{}::{}", parent_name, scope)
        } else {
            parent_name.to_owned()
        };

        constants.push(Constant {
            path: path.to_owned(),
            name: child_constant.name.clone(),
            loc: child_constant.loc,
            scope: Some(scope),
        });
    }

    constants
}

impl visitor::Visitor for Visitor {
    fn on_class(&mut self, node: &nodes::Class) {
        let name = fetch_const_name(&node.name);
        let loc = fetch_const_loc(&node.name);

        let definition = Constant {
            path: self.path.clone(),
            scope: None,
            name: name.clone(),
            loc,
        };

        let mut visitor = Visitor::new(&self.path);

        if let Some(body) = node.body.as_ref() {
            visitor.visit(body);
        }

        self.definitions.push(definition);

        self.definitions.append(&mut nest_constants(&self.path, &name, visitor.definitions));
        self.references.append(&mut nest_constants(&self.path, &name, visitor.references));
    }

    fn on_module(&mut self, node: &nodes::Module) {
        let name = fetch_const_name(&node.name);
        let loc = fetch_const_loc(&node.name);

        let definition = Constant {
            path: self.path.clone(),
            scope: None,
            name: name.clone(),
            loc,
        };

        let mut visitor = Visitor::new(&self.path);

        if let Some(body) = node.body.as_ref() {
            visitor.visit(body);
        }

        self.definitions.push(definition);
        self.definitions.append(&mut nest_constants(&self.path, &name, visitor.definitions));
        self.references.append(&mut nest_constants(&self.path, &name, visitor.references));
    }

    fn on_const(&mut self, node: &nodes::Const) {
        let name = fetch_const_const_name(node);

        let reference = Constant {
            path: self.path.clone(),
            name,
            loc: node.name_l,
            scope: None,
        };

        self.references.push(reference);
    }

    fn on_casgn(&mut self, node: &nodes::Casgn) {
        let name = fetch_casn_const_name(node);

        let definition = Constant {
            name,
            scope: None,
            path: self.path.clone(),
            loc: node.name_l,
        };

        self.definitions.push(definition);
    }
}
