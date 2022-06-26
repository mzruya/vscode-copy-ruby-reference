use constant::{Definition, Reference};
use lib_ruby_parser::{traverse::visitor::Visitor, Parser, ParserOptions};
use line_col::*;
use std::path::{Path, PathBuf};

mod constant;
mod visitor;

#[derive(Debug)]
pub struct ParsedFile {
    pub path: PathBuf,
    pub definitions: Vec<Definition>,
    pub references: Vec<Reference>,
}

fn parse_text(text: &str, path: &Path) -> (Vec<Definition>, Vec<Reference>) {
    let parser = Parser::new(text, ParserOptions::default());
    let ast = parser.do_parse().ast;

    if ast.is_none() {
        return (Vec::new(), Vec::new());
    }

    let mut visitor = visitor::Visitor::new(path);
    visitor.visit(&ast.unwrap());

    let definitions: Vec<Definition> = visitor.definitions.into_iter().map(|constant| constant.into()).collect();
    let references: Vec<Reference> = visitor.references.into_iter().map(|constant| constant.into()).collect();

    (definitions, references)
}

pub fn copy_reference(path: &Path, line: usize, character: usize) -> Option<String> {
    let text = std::fs::read_to_string(path).unwrap();

    let (definitions, _) = parse_text(&text, path);
    let lookup = LineColLookup::new(&text);

    let definition = definitions.iter().find(|definition| {
        let (begin_line, begin_char) = lookup.get(definition.loc.begin);
        let (_, end_char) = lookup.get(definition.loc.end);

        begin_line == line && (begin_char <= character) && (character <= end_char)
    });

    definition.map(|definition| definition.qualified())
}

#[cfg(test)]
fn find_loc<'a>(lookup: &LineColLookup, definitions: &'a [Definition], name: &str) -> (usize, std::ops::Range<usize>) {
    let definition = definitions.iter().find(|definition| definition.qualified() == name).unwrap();

    let (begin_line, begin_char) = lookup.get(definition.loc.begin);
    let (_, end_char) = lookup.get(definition.loc.end);

    (begin_line, begin_char..end_char)
}

#[test]
#[cfg(test)]
fn test_parse_text() {
    let fixture = Path::new("./fixture.rb");
    let text = std::fs::read_to_string(fixture).unwrap();
    let lookup = LineColLookup::new(&text);

    let (definitions, _) = parse_text(&text, fixture);

    assert_eq!(find_loc(&lookup, &definitions, "AA"), (1, 7..9));
    assert_eq!(find_loc(&lookup, &definitions, "AA::BB"), (2, 10..12));
    assert_eq!(find_loc(&lookup, &definitions, "AA::BB::CC"), (3, 11..13));
    assert_eq!(find_loc(&lookup, &definitions, "AA::BB::CC::CC_1"), (4, 7..11));
    assert_eq!(find_loc(&lookup, &definitions, "AA::BB::CC::DD::EE::FF"), (6, 13..23));
    assert_eq!(find_loc(&lookup, &definitions, "AA::BB::CC::DD::EE::FF::GG::HH"), (7, 16..22));
    assert_eq!(find_loc(&lookup, &definitions, "AA::BB::CC::DD::EE::FF::GG::HH::II"), (8, 11..13));
}

#[test]
#[cfg(test)]
fn test_copy_reference() {
    let fixture = Path::new("./fixture.rb");

    for char in 7..9 {
        assert_eq!(copy_reference(fixture, 1, char).unwrap(), "AA");
    }

    for char in 10..12 {
        assert_eq!(copy_reference(fixture, 2, char).unwrap(), "AA::BB");
    }

    for char in 11..13 {
        assert_eq!(copy_reference(fixture, 3, char).unwrap(), "AA::BB::CC");
    }

    for char in 7..11 {
        assert_eq!(copy_reference(fixture, 4, char).unwrap(), "AA::BB::CC::CC_1");
    }

    for char in 13..23 {
        assert_eq!(copy_reference(fixture, 6, char).unwrap(), "AA::BB::CC::DD::EE::FF");
    }

    for char in 16..22 {
        assert_eq!(copy_reference(fixture, 7, char).unwrap(), "AA::BB::CC::DD::EE::FF::GG::HH");
    }

    for char in 11..13 {
        assert_eq!(copy_reference(fixture, 8, char).unwrap(), "AA::BB::CC::DD::EE::FF::GG::HH::II");
    }
}
