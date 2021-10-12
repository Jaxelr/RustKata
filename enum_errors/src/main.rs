use std::{fmt::Display, fs::File, io::Read, num::ParseIntError};

#[derive(Debug)]
enum NumberFromFileError {
    ParseError(ParseIntError),
    IoError(std::io::Error),
}

impl Display for NumberFromFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumberFromFileError::ParseError(parse_int_error) => write!(f, "{}", parse_int_error),
            NumberFromFileError::IoError(io_error) => write!(f, "{}", io_error),
        }
    }
}

impl From<ParseIntError> for NumberFromFileError {
    fn from(err: ParseIntError) -> Self {
        NumberFromFileError::ParseError(err)
    }
}

impl From<std::io::Error> for NumberFromFileError {
    fn from(err: std::io::Error) -> Self {
        NumberFromFileError::IoError(err)
    }
}

impl std::error::Error for NumberFromFileError {}


fn read_number_from_file(filename: &str) -> Result<u64, NumberFromFileError> {
    let mut file = File::open(filename)?;

    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    let parsed: u64 = buffer.trim().parse()?;

    Ok(parsed)
}

fn main() {
    match read_number_from_file("number.txt") {
        Ok(v) => println!("Your number is {}", v),
        Err(err) => match err {
            NumberFromFileError::IoError(_) => println!("Error from IO!"),
            NumberFromFileError::ParseError(_) => println!("Error from Parsing!"),
        },
    };
}