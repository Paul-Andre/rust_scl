//! This crate is for reading and writing Scala scale files (.scl).
//! http://www.huygens-fokker.org/scala/scl_format.html

extern crate num;

use num::rational::Rational32;
use std::error::Error;


#[derive(Debug, PartialEq)]
pub enum Note {
    Cents(f64),
    Ratio(Rational32),
}

pub struct Scale {
    description: String,
    notes: Vec<Note>,
}

/// Parses a note that either contains a period and is a cent value, or is a ratio.
///
/// This function assumes that the passed string has been trimmed.
pub fn parse_note(string: &str) -> Result<Note, &'static str> {
    if string.contains(".") {
        match string.parse::<f64>() {
            Ok(cents) => Ok(Note::Cents(cents)),
            Err(_) => Err("error parsing cent value"),
        }
    }
    else {
        match string.parse::<Rational32>() {
            Ok(ratio) => {
                if num::Signed::is_negative(&ratio) {
                    Err("ratio is negative")
                }
                else {
                    Ok(Note::Ratio(ratio))
                }
            }
            Err(_) => Err("error parsing ratio value")
        }
    }
}
                

/* 
pub fn read(scale_string: &str) -> Result<Scale, &'static str> {
    let lines_without_comments = scale_string.lines()
        .filter(|line| !line.starts_with("!"));
     
    let description = match lines_without_comments.next() {
        Some(line) => line.clone(),
        None => {return Err("couldn't read description line");},
    };

    let trimmed_lines = lines_without_comments.map(|line| line.trim());

    let number = match trimmed_lines.next() {
        Some(line) => match line.parse::<u32>() {
            Ok(number) => number,
            Err(_) => {return Err("invalid number of notes");},
        },
        None => {return Err("couldn't read number of notes line");},
    };

    let mut valid = true;

    let notes = trimmed_lines.map(|line| match line.split_whitespace().next() {
        Some(note_string) => Some(parse_note(note_string)),
        None => None,
    })
    .map(|double_wrapped_note| if let Some(Ok(note)) = double_wrapped_note {
        Some(note)
    }
    else {
        None
    })
    .map(|wrapped_note| if wrapped_note.is_none
    .take_while(|wrapped_note| wrapped_note.is_some())
    .filter_map(|wrapped_note| wrapped_note)
    .


*/


    


#[cfg(test)]
mod tests {
    use Note;
    use parse_note;
    use num::rational::Rational32;

    #[test]
    fn parse_note_valid_input() {
        assert_eq!(parse_note("0.0").unwrap(), Note::Cents(0.0f64));
        assert_eq!(parse_note("0.").unwrap(), Note::Cents(0.0f64));
        assert_eq!(parse_note(".0").unwrap(), Note::Cents(0.0f64));
        assert_eq!(parse_note("0.5").unwrap(), Note::Cents(0.5f64));
        assert_eq!(parse_note("1200.").unwrap(), Note::Cents(1200.0f64));
        

        assert_eq!(parse_note("1").unwrap(), Note::Ratio(Rational32::new(1,1)));
        assert_eq!(parse_note("2").unwrap(), Note::Ratio(Rational32::new(2,1)));
        assert_eq!(parse_note("1/3").unwrap(), Note::Ratio(Rational32::new(1,3)));
        assert_eq!(parse_note("2/3").unwrap(), Note::Ratio(Rational32::new(2,3)));
    }

    #[test]
    fn parse_note_not_valid_input() {
        parse_note("").unwrap_err();
        parse_note("a").unwrap_err();
        parse_note("a1.32").unwrap_err();
        parse_note("gourd").unwrap_err();

        parse_note("-1/2").unwrap_err();
    }
}
