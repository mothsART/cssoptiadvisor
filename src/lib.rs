use std::fs;
use std::path::Path;
use std::collections::HashSet;
use std::io::{Error, ErrorKind};

use walkdir::WalkDir;
use cssparser::{ParseError, Parser, ParserInput};

fn parse_keyframes_block<'i>(
    path: &Path,
    keyframes_name: &String,
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
                        "@keyframes \"{keyframes_name}\" : the value \"{cast_unit_v}%\" already exist in file \"{}\".",
                        path.display()
                    ));
                }

                keyframes_values.insert(cast_unit_v);
                if *unit_value == 1.0 {
                    results.insert(format!(
                        "@keyframes \"{keyframes_name}\" : the value \"100%\" can be replaced by \"to\" in file \"{}\".",
                        path.display()
                    ));
                }
            }
            cssparser::Token::Ident(value) => {
                if *value == "from" {
                    results.insert(format!(
                        "@keyframes \"{keyframes_name}\" : the value \"from\" can be replaced by \"0%\" in file \"{}\".",
                        path.display()
                    ));
                }
            }
            _ => {}
        }
    }
    Ok(())
}

pub fn parse_file(
    path: &Path,
    results: &mut HashSet<String>,
) -> std::io::Result<()> {
    let contents = fs::read_to_string(path)?;
    let mut input = ParserInput::new(&contents);
    let mut parser = Parser::new(&mut input);
    let mut last_keyframes_name = "".to_string();

    while let Ok(token) = parser.next() {
        match token {
            cssparser::Token::CurlyBracketBlock => {
                let _ = parser.parse_nested_block(|input| {
                    parse_keyframes_block(path, &last_keyframes_name, results, input)
                });
            }
            cssparser::Token::Ident(name) => {
                last_keyframes_name = name.to_string();
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
    if path.is_dir() {
        for entry in WalkDir::new(path) {
            let _entry = entry.unwrap();
            if _entry.path().is_file() {
                parse_file(&_entry.path(), results)?;
            }
        }
        return Ok(())
    }
    if path.is_file() {
       return Ok(parse_file(&path, results)?);
    }
    Err(Error::new(
        ErrorKind::Interrupted,
        format!("Path \"{}\" doesn't exist.", path.display())
    ))
}
