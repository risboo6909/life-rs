use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error};
use std::iter::{FromIterator, Peekable};
use std::cell::RefCell;
use std::error;
use std::fmt;
use std::fmt::{Formatter, Display};

use super::Coord;


#[derive(Debug, PartialEq)]
enum Lexem {
    ParamName(String),
    ValueNumeric(isize),
    ValueString(String),
    Comma,
}

#[derive(Debug)]
pub enum ParseError {
    NotANumber(String),
    UnexpectedSymbol(String),
    InputExhausted,
    EmptyName,
    WrongName(String),
}

impl Display for ParseError {

    fn fmt(&self, f: &mut Formatter) -> fmt::Result {

        match *self {
            ParseError::NotANumber(ref unparsed) => write!(f, "Not a number {}", unparsed),
            ParseError::UnexpectedSymbol(ref unparsed) => write!(f, "Unexpected symbol {}", unparsed),
            ParseError::InputExhausted => write!(f, "Input exhausted"),
            ParseError::EmptyName => write!(f, "Option name can't be empty"),
            ParseError::WrongName(ref unparsed) => write!(f, "Wrong option name {}", unparsed)
        }

    }

}

impl error::Error for ParseError {

    fn description(&self) -> &str {

        match *self {
            ParseError::NotANumber(_) => "Not a number",
            ParseError::UnexpectedSymbol(_) => "Unexpected symbol",
            ParseError::InputExhausted => "Input exhausted",
            ParseError::EmptyName => "Option name can't be empty",
            ParseError::WrongName(_) => "Wrong option name",
        }

    }

}


trait InputProviderTrait {
    fn read_line(&mut self) -> Option<String>;
}

struct FileInputProvider {
    buf_reader: RefCell<BufReader<File>>,
}

impl FileInputProvider {

    pub fn new(file_name: String) -> Self {
        let f = File::open(file_name).expect("file not found!");
        Self { buf_reader: RefCell::new(BufReader::new(f)) }
    }

}

struct IterHelper<'a, T: 'a> {
    obj: &'a mut T,
}

impl<'a, T: 'a> Iterator for IterHelper<'a, T>
    where T: InputProviderTrait {

    type Item=String;

    fn next(&mut self) -> Option<Self::Item> {
        self.obj.read_line()
    }

}

impl<'a> IntoIterator for &'a mut FileInputProvider {

    type Item = String;
    type IntoIter = IterHelper<'a, FileInputProvider>;

    fn into_iter(self) -> Self::IntoIter {
        IterHelper{obj: self}
    }

}

impl InputProviderTrait for FileInputProvider {

    fn read_line(&mut self) -> Option<String> {

        let mut line = String::new();

        let num_bytes = self.buf_reader.
                             borrow_mut().
                             read_line(&mut line).
                             expect("error reading stream");

        match num_bytes {
            0 => None,
            _ => Some(line)
        }

    }

}

struct StringDataProvider {
    lines: Vec<String>,
}

impl StringDataProvider {
    pub fn new(input_string: String) -> Self {
        Self{ lines: input_string.lines().map(|x| x.to_string()).rev().collect::<Vec<String>>() }
    }
}

impl InputProviderTrait for StringDataProvider {
    fn read_line(&mut self) -> Option<String> {
        self.lines.pop()
    }
}

impl<'a> IntoIterator for &'a mut StringDataProvider {

    type Item = String;
    type IntoIter = IterHelper<'a, StringDataProvider>;

    fn into_iter(self) -> Self::IntoIter {
        IterHelper{obj: self}
    }

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

fn filter_line<'a>(line: &'a str) -> Box<Iterator<Item=char> + 'a> {
    // filter redundant chars
    Box::new(line.chars().filter(|&c| !contains(c, &CHARS_TO_FILTER[..])))
}

fn rle_decoder(line: &str, mut row: isize) -> Vec<Coord> {

    let mut decoded: Vec<Coord> = Vec::new();
    let mut prefix: Vec<char> = Vec::new();

    let mut col = 0;

    let mut it = filter_line(line).peekable();

    while let Some(c) = it.next() {

        prefix.push(c);

        match(c) {

            t @ 'b' | t @ 'o' => {

                // b - dead cell
                // o - alive cell

                let mut repeat = 1;

                if !prefix.is_empty() {
                    repeat = match get_num(&mut prefix.iter().cloned().peekable()) {
                        Ok(n) => n,
                        Err(_) => 1,
                    };
                }

                for idx in 0..repeat {
                    if t == 'o' {
                        decoded.push(Coord { col: col, row: row });
                    }
                    col += 1;
                }

                prefix.clear();
            },

            _ => {
                    // skip unknown characters
                 },

        }

    }

    decoded

}

fn lexer(line: &str) -> Result<Vec<Lexem>, ParseError> {

    let mut result: Vec<Lexem> = Vec::new();
    let mut prefix: Vec<char> = Vec::new();

    let mut it = filter_line(line).peekable();

    while let Some(c) = it.next() {

        prefix.push(c);

        match(c) {

            '=' => {

                // read param name
                let name = get_str(&mut prefix.iter().cloned().peekable());

                // ensure parameter name is valid, i.e.
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
            },

            _ => { }

        }

    }

    Ok(result)

}

fn parse_stream<T>(mut data_provider: T) -> Result<Vec<Coord>, ParseError>
            where for<'a> &'a mut T: IntoIterator<Item=String> {

    for line in &mut data_provider {

        if line.starts_with('#') {
            // skip comments
            continue;
        } else {
            // read header data
            lexer(&line[..])?;
            break;
        }

    }

    // parse RLE-encoded data
    let mut rle_line = String::new();

    // vector of occupied cells
    let mut coords: Vec<Coord> = Vec::new();

    let mut row: isize = 0;

    for line in &mut data_provider {

        let tmp = rle_line.clone();

        // read rle-encoded line until new line or eof symbol detected
        for c in line.chars() {
            if c != '$' && c != '!' {
                rle_line.push(c);
            } else {
                let decoded = rle_decoder(rle_line.as_str(), row);
                coords.extend(decoded);

                rle_line.clear();
                row += 1;
            }
        }
    }

    Ok(coords)

}

pub fn from_file(file_name: String) -> Result<Vec<Coord>, ParseError> {

    // accepted file format described here:
    // http://www.conwaylife.com/w/index.php?title=Run_Length_Encoded

    let mut data_provider = FileInputProvider::new(file_name);
    let cells_data = parse_stream(data_provider)?;

    Ok(cells_data)

}

pub fn from_string(input_string: String) -> Result<Vec<Coord>, ParseError> {

    let mut data_provider = StringDataProvider::new(input_string);
    let cells_data = parse_stream(data_provider)?;

    Ok(cells_data)
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

#[test]
fn test_parse_rle1() {
    // glyder
    assert!(from_string(String::from("x =  3, y = 0\nbo$2b\no$3o!")).unwrap() ==
    [Coord { col: 1, row: 0 }, Coord { col: 2, row: 1 }, Coord { col: 0, row: 2 },
     Coord { col: 1, row: 2 }, Coord { col: 2, row: 2 }]);
}

#[test]
fn test_parse_rle2() {
    // Gosper glider gun
    assert!(from_string(String::from("x= 20, y=100\n24bo$22bobo$12b2o6b2o12b2o$11bo3bo4b2o12b2\
    o$2o8bo5bo3b2o$2o8bo3bob2o4bobo$10bo5bo7bo$11bo3bo$12b2o!")).unwrap() ==
           &[Coord { col: 24, row: 0 }, Coord { col: 22, row: 1 }, Coord { col: 24, row: 1 },
             Coord { col: 12, row: 2 }, Coord { col: 13, row: 2 }, Coord { col: 20, row: 2 },
             Coord { col: 21, row: 2 }, Coord { col: 34, row: 2 }, Coord { col: 35, row: 2 },
             Coord { col: 11, row: 3 }, Coord { col: 15, row: 3 }, Coord { col: 20, row: 3 },
             Coord { col: 21, row: 3 }, Coord { col: 34, row: 3 }, Coord { col: 35, row: 3 },
             Coord { col: 0, row: 4 }, Coord { col: 1, row: 4 }, Coord { col: 10, row: 4 },
             Coord { col: 16, row: 4 }, Coord { col: 20, row: 4 }, Coord { col: 21, row: 4 },
             Coord { col: 0, row: 5 }, Coord { col: 1, row: 5 }, Coord { col: 10, row: 5 },
             Coord { col: 14, row: 5 }, Coord { col: 16, row: 5 }, Coord { col: 17, row: 5 },
             Coord { col: 22, row: 5 }, Coord { col: 24, row: 5 }, Coord { col: 10, row: 6 },
             Coord { col: 16, row: 6 }, Coord { col: 24, row: 6 }, Coord { col: 11, row: 7 },
             Coord { col: 15, row: 7 }, Coord { col: 12, row: 8 }, Coord { col: 13, row: 8 }][..]);
}
