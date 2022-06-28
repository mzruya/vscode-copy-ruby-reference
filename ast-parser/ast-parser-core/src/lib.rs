use constant::{Definition, Reference};
use lib_ruby_parser::{traverse::visitor::Visitor, Loc, Parser, ParserOptions};
use line_col::*;

mod constant;
mod visitor;

fn parse_text(text: &str) -> (Vec<Definition>, Vec<Reference>) {
    let parser = Parser::new(text, ParserOptions::default());
    let ast = parser.do_parse().ast;

    if ast.is_none() {
        return (Vec::new(), Vec::new());
    }

    let mut visitor = visitor::Visitor::new();
    visitor.visit(&ast.unwrap());

    let definitions: Vec<Definition> = visitor.definitions.into_iter().map(|constant| constant.into()).collect();
    let references: Vec<Reference> = visitor.references.into_iter().map(|constant| constant.into()).collect();

    (definitions, references)
}

pub fn copy_reference(text: &str, line: usize, caret_position: usize) -> Option<String> {
    let (definitions, references) = parse_text(text);
    let lookup = LineColLookup::new(text);

    let mut reference_locations: Vec<(String, Loc)> = Vec::new();

    for definition in definitions {
        reference_locations.push((definition.qualified(), definition.loc));
    }

    for reference in references {
        reference_locations.push((reference.name, reference.loc));
    }

    let reference_location = reference_locations.iter().find(|(_, loc)| {
        let (begin_line, begin_char) = lookup.get(loc.begin);
        let (_, end_char) = lookup.get(loc.end);
        begin_line == line && (begin_char <= caret_position) && (caret_position <= end_char)
    });

    reference_location.map(|(reference, _)| reference.clone())
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
    use std::path::Path;

    let fixture = Path::new("./text_fixtures/constant_definitions.rb");
    let text = std::fs::read_to_string(fixture).unwrap();
    let lookup = LineColLookup::new(&text);

    let (definitions, _) = parse_text(&text);

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
    use std::path::Path;

    let fixture = std::fs::read_to_string(Path::new("./text_fixtures/constant_definitions.rb")).unwrap();

    for caret_position in 7..=9 {
        assert_eq!(copy_reference(&fixture, 1, caret_position).unwrap(), "AA");
    }

    for caret_position in 10..=12 {
        assert_eq!(copy_reference(&fixture, 2, caret_position).unwrap(), "AA::BB");
    }

    for caret_position in 11..=13 {
        assert_eq!(copy_reference(&fixture, 3, caret_position).unwrap(), "AA::BB::CC");
    }

    for caret_position in 7..=11 {
        assert_eq!(copy_reference(&fixture, 4, caret_position).unwrap(), "AA::BB::CC::CC_1");
    }

    for caret_position in 13..=23 {
        assert_eq!(copy_reference(&fixture, 6, caret_position).unwrap(), "AA::BB::CC::DD::EE::FF");
    }

    for caret_position in 16..=22 {
        assert_eq!(copy_reference(&fixture, 7, caret_position).unwrap(), "AA::BB::CC::DD::EE::FF::GG::HH");
    }

    for caret_position in 11..=13 {
        assert_eq!(copy_reference(&fixture, 8, caret_position).unwrap(), "AA::BB::CC::DD::EE::FF::GG::HH::II");
    }

    let fixture = std::fs::read_to_string(Path::new("./text_fixtures/constant_references.rb")).unwrap();

    for caret_position in 1..=7 {
        assert_eq!(copy_reference(&fixture, 1, caret_position).unwrap(), "AA::BB");
    }

    for caret_position in 1..=11 {
        assert_eq!(copy_reference(&fixture, 2, caret_position).unwrap(), "AA::BB::CC");
    }
}
