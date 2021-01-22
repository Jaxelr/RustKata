use std::io::prelude::*;
use std::io::BufReader;
use std::env;
use std::fs::File;

use thiserror::Error;
use anyhow::Context;

#[derive(Error, Debug)]
enum WordCountError {
    #[error("Source contains no data")]
    EmptySource,

    #[error("Read error")]
    ReadError { source: std::io::Error },

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

fn count_words<R: Read>(filename: &mut R) -> Result<u32, WordCountError> {
    let reader = BufReader::new(filename);
    let mut wordcounter = 0;
    for line in reader.lines() {
        let line = line.map_err(|source| WordCountError::ReadError { source })?;
        for _word in line.split_whitespace() {
            wordcounter += 1;
        }
    }

    if wordcounter == 0 {
        return Err(WordCountError::EmptySource);
    }

    Ok(wordcounter)
}

fn process() -> anyhow::Result<()> {
    for filename in env::args().skip(1).collect::<Vec<String>>() {
        let mut reader = File::open(&filename).context(format!("unable to open '{}'", filename))?;
        let wordcounter =
            count_words(&mut reader).context(format!("unable to count words in '{}'", filename))?;
        println!("{} {}", wordcounter, filename);
    }
    Ok(())
}

fn main() {
    if let Err(err) = process() {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}
