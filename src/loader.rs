extern crate itertools;

use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use self::itertools::multipeek;
use self::itertools::structs::MultiPeek;
use loader::itertools::Itertools;
use std::iter::FromIterator;


#[derive(Debug, PartialEq)]
enum Lexem {
    ParamName(String),
    ValueNumeric(isize),
    ValueString(String)
}

#[derive(Debug)]
enum ParseError {
    NotANumber(String),
    InputExhausted
}

fn get_str<T: Iterator<Item=char>>(iter: &mut MultiPeek<T>) -> String {

    let mut result = String::new();

    while let Some(c) = iter.next() {

        if c.is_alphabetic() {
            result.push(c);
        } else {
            break;
        }

    }

    result

}

fn get_num<T: Iterator<Item=char>>(iter: &mut MultiPeek<T>) -> Result<isize, ParseError> {

    let mut result = 0;

    {

        match (iter.peek().cloned()) {

            Some(c) => if !c.is_digit(10) {
                return Err(ParseError::NotANumber(String::from_iter(iter)));
            },
            None => return Err(ParseError::InputExhausted),
        }

    }

    while let Some(c) = iter.next() {

        if c.is_digit(10) {
            let digit = c.to_string().parse::<isize>().expect("unexpected error");
            result = result * 10 + digit;
        } else {
            break;
        }

    }

    Ok(result)
}

fn lexer(line: &str) -> Result<Vec<Lexem>, ParseError> {

    let mut result:Vec<Lexem> = Vec::new();

    // filter spaces
    let filtered_spaces = line.chars().filter(|&c| c != ' ');

    let mut it = multipeek(filtered_spaces);

    while let Some(&c) = it.peek() {

        // skip comments
        if c == '#' {
            break;
        }

        match(c) {

            '=' => {

                // read param name
                let name = get_str(&mut it);

                // read param value
                let value = get_num(&mut it)?;

                result.push(Lexem::ParamName(name.clone()));
                result.push(Lexem::ValueNumeric(value));

            },

            'b' | 'o' => {
                get_num(&mut it);
                it.next();
            }

            '!' | '$' => {
                it.next();
            }

            _ => {}

        }

    }

    Ok(result)

}

pub fn from_file(file_name: Option<String>) -> Result<Vec<(isize, isize)>, io::Error> {
    // accepted file format described here:
    // http://www.conwaylife.com/w/index.php?title=Run_Length_Encoded
    let cells_data = Vec::new();

    match(file_name) {

        Some(file_name) => {

            let f = File::open(file_name).expect("file not found!");

            let mut buf_reader = BufReader::new(f);
            let mut line = String::new();

            while buf_reader.read_line(&mut line).unwrap() > 0 {
                lexer(&line[..]);
                line.clear();
            }

            Ok(cells_data)
        },
        None => Ok(Vec::new())
    }
}


#[test]
fn test_lexer_param() {
    assert!(lexer("x =  25\ny = 30").unwrap() ==
        vec![Lexem::ParamName(String::from("x")), Lexem::ValueNumeric(25),
             Lexem::ParamName(String::from("y")), Lexem::ValueNumeric(30)]);
}

#[test]
#[should_panic]
fn test_lexer_error1() {
    lexer("x =  a25\ny = 30").unwrap();
}
