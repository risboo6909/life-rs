extern crate engine;

use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error};
use std::iter::{FromIterator, Peekable};


#[derive(Debug, PartialEq)]
enum Lexem {
    ParamName(String),
    ValueNumeric(isize),
    ValueString(String),
    Comma,
}

#[derive(Debug)]
enum ParseError {
    NotANumber(String),
    UnexpectedSymbol(String),
    InputExhausted,
    EmptyName,
    WrongName(String),
}


fn get_str<T>(it: &mut Peekable<T>) -> String

    where T: Iterator<Item=char> {

    let mut result = String::new();

    loop {
        match it.peek().cloned() {
            Some(c) => {
                if c != '=' {
                    result.push(c);
                } else {
                    break
                }
                it.next();
            }
            None => break,
        }
    }

    result

}

fn get_num<T>(it: &mut Peekable<T>) -> Result<isize, ParseError>
        where T: Iterator<Item=char> {

    let mut result = 0;

    match it.peek().cloned() {

        Some(c) => {
            if !c.is_digit(10) {
                return Err(ParseError::NotANumber(String::from_iter(it)));
            }
        },
        None => return Err(ParseError::InputExhausted),

    }

    loop {

        match it.peek().cloned() {
            Some(c) => {
                if c.is_digit(10) {
                    let digit = c.to_string().parse::<isize>().unwrap();
                    result = result * 10 + digit;
                    it.next();
                } else {
                    break;
                }
            }
            None => break
        }

    }

    Ok(result)

}

const CHARS_TO_FILTER: [char; 3] = [' ', '\n', '\r'];

fn contains(c: char, arr: &[char]) -> bool {
    arr.iter().position(|&x| c == x).is_some()
}

fn lexer(line: &str) -> Result<Vec<Lexem>, ParseError> {

    let mut result: Vec<Lexem> = Vec::new();

    // filter redundant chars
    let mut it = line.chars().filter(|&c|
        !contains(c, &CHARS_TO_FILTER[..])).peekable();

    let mut prefix: Vec<char> = Vec::new();

    while let Some(c) = it.next() {

        // skip comments
        if c == '#' {
            break;
        }

        prefix.push(c);

        match(c) {

            '=' => {

                // read param name
                let name = get_str(&mut prefix.iter().cloned().peekable());

                // ensure parameter name is ok
                if name.is_empty() {
                    // it's not empty
                    return Err(ParseError::EmptyName);
                } else if !name.chars().all(|c| c.is_alphabetic())  {
                    // and contains only alphabetic characters
                    return Err(ParseError::WrongName(String::from_iter(prefix)));
                }

                prefix.clear();

                // read param value
                let value = get_num(&mut it)?;

                result.push(Lexem::ParamName(name.clone()));
                result.push(Lexem::ValueNumeric(value));

            },

            ',' => {
                result.push(Lexem::Comma);
                prefix.clear();
            }

            'b' | 'o' => {
                if !prefix.is_empty() {
                    let repeat = match get_num(&mut prefix.iter().cloned().peekable()) {
                        Ok(n) => n,
                        Err(_) => 1,
                    };

                }
                prefix.clear();
            }

            '!' | '$' => {
                //it.next();
            }

            _ => { }

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
fn test_lexer_param_comma_sep() {
    assert!(lexer("x =  25 ,   y = 30").unwrap() ==
        vec![Lexem::ParamName(String::from("x")), Lexem::ValueNumeric(25), Lexem::Comma,
             Lexem::ParamName(String::from("y")), Lexem::ValueNumeric(30)]);
}

#[test]
#[should_panic]
fn test_lexer_error1() {
    lexer("x =  a25\ny = 30").unwrap();
}

#[test]
#[should_panic]
fn test_lexer_error2() {
    lexer("x =  ").unwrap();
}

#[test]
#[should_panic]
fn test_lexer_error3() {
    lexer("=  25").unwrap();
}
