use std::collections::HashSet;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

use cssparser::{ParseError, Parser, ParserInput, ToCss, Token};
use ordered_float::OrderedFloat;
use walkdir::WalkDir;

fn parse_keyframe_values_block<'i>(
    _path: &Path,
    _keyframes_name: &str,
    _results: &mut HashSet<String>,
    input: &mut Parser<'i, '_>,
) -> Result<(), ParseError<'i, ()>> {
    while let Ok(token) = input.next() {
        let mut d = String::new();
        let _r = token.to_css(&mut d);
        println!("{d}");

        match token {
            Token::Function(name) => {
                println!(">> {:?}", name);
            }
            _ => {
                println!(">> {:?}", token);
            }
        }
    }
    Ok(())
}

fn parse_keyframes_block<'i>(
    path: &Path,
    keyframes_name: &String,
    results: &mut HashSet<String>,
    input: &mut Parser<'i, '_>,
) -> Result<(), ParseError<'i, ()>> {
    let mut keyframes_values: HashSet<OrderedFloat<f32>> = HashSet::new();

    while let Ok(token) = input.next() {
        match token {
            Token::Percentage { unit_value, .. } => {
                let cast_unit_v = (*unit_value * 100.0).to_string();
                if keyframes_values.contains(&OrderedFloat(*unit_value)) {
                    results.insert(format!(
                        "@keyframes \"{keyframes_name}\" : the value \"{cast_unit_v}%\" already exist in file \"{}\".",
                        path.display()
                    ));
                }

                keyframes_values.insert(OrderedFloat(*unit_value));
                if *unit_value == 1.0 {
                    results.insert(format!(
                        "@keyframes \"{keyframes_name}\" : the value \"100%\" can be replaced by \"to\" in file \"{}\".",
                        path.display()
                    ));
                }
            }
            Token::Ident(value) => {
                println!("&&& {:?}", value);
                if *value == "from" {
                    results.insert(format!(
                        "@keyframes \"{keyframes_name}\" : the value \"from\" can be replaced by \"0%\" in file \"{}\".",
                        path.display()
                    ));
                }
                // TODO : value is different than "from", "to" and a percentage.
            }
            Token::CurlyBracketBlock => {
                let _ = input.parse_nested_block(|input| {
                    parse_keyframe_values_block(path, keyframes_name, results, input)
                });
            }
            _ => {}
        }
    }
    Ok(())
}

pub fn parse_file(path: &Path, results: &mut HashSet<String>) -> std::io::Result<()> {
    let contents = fs::read_to_string(path)?;
    let mut input = ParserInput::new(&contents);
    let mut parser = Parser::new(&mut input);
    let mut last_keyframes_name = "".to_string();

    while let Ok(token) = parser.next() {
        match token {
            Token::CurlyBracketBlock => {
                let _ = parser.parse_nested_block(|input| {
                    parse_keyframes_block(path, &last_keyframes_name, results, input)
                });
            }
            Token::Ident(name) => {
                last_keyframes_name = name.to_string();
            }
            _ => {}
        }
    }
    Ok(())
}

pub fn parse(path: &Path, results: &mut HashSet<String>) -> std::io::Result<()> {
    if path.is_dir() {
        for entry in WalkDir::new(path) {
            let _entry = entry.unwrap();
            if _entry.path().is_file() {
                parse_file(_entry.path(), results)?;
            }
        }
        return Ok(());
    }
    if path.is_file() {
        return parse_file(path, results);
    }
    Err(Error::new(
        ErrorKind::Interrupted,
        format!("Path \"{}\" doesn't exist.", path.display()),
    ))
}
