use std::fs;
use std::path::Path;
use std::collections::HashSet;

use cssparser::{Parser, ParserInput, ParseError};

fn parse_keyframes_block<'i>(
    path: &Path,
    results: &mut HashSet<String>,
    input: &mut Parser<'i, '_>,
) -> Result<(), ParseError<'i, ()>> {
    let mut keyframes_values = HashSet::new();
    while let Ok(token) = input.next() {
        match token {
            cssparser::Token::Percentage {unit_value, ..} => {
                let cast_unit_v = (*unit_value * 100.2) as i32;
                if keyframes_values.contains(&cast_unit_v) {
                    results.insert(format!(
                        "@keyframes : the value \"{cast_unit_v}%\" already exist in file \"{}\".",
                        path.display()
                    ));
                }

                keyframes_values.insert(cast_unit_v);
                if *unit_value == 1.0 {
                    results.insert(format!(
                        "@keyframes : the value \"100%\" can be replaced by \"to\" in file \"{}\".",
                        path.display()
                    ));
                }
            },
            cssparser::Token::Ident(value) => {
                if *value == "from" {
                    results.insert(format!(
                        "@keyframes : the value \"from\" can be replaced by \"0%\" in file \"{}\".",
                        path.display()
                    ));
                }
            }
            _ => {}
        }
    }
    Ok(())
}

pub fn parse(
    path: &Path,
    results: &mut HashSet<String>,
) -> std::io::Result<()> {
    let contents = fs::read_to_string(path)?;
    let mut input = ParserInput::new(&contents);
    let mut parser = Parser::new(&mut input);

    while let Ok(token) = parser.next() {
        match token {
            cssparser::Token::CurlyBracketBlock => {
                let _ = parser.parse_nested_block(|input| {
                    parse_keyframes_block(path, results, input)
                });
            },
            _ => {}
        }
    }
    Ok(())
}
