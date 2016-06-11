//! This crate is for reading and writing Scala scale files (.scl).
//! http://www.huygens-fokker.org/scala/scl_format.html

extern crate num;

use num::rational::Rational32;

#[derive(Debug, PartialEq)]
pub enum Note {
    Cents(f64),
    Ratio(Rational32),
}

#[derive(Debug, PartialEq)]
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
                

pub fn read(scale_string: &str) -> Result<Scale, &'static str> {
    let mut lines_without_comments = scale_string.lines()
        .filter(|line| !line.starts_with("!"));
     
    let description = match lines_without_comments.next() {
        Some(line) => line.to_string(),
        None => {return Err("couldn't read description line");},
    };

    let mut trimmed_lines = lines_without_comments.map(|line| line.trim());

    let number = match trimmed_lines.next() {
        Some(line) => match line.parse() {
            Ok(number) => number,
            Err(_) => {return Err("invalid number of notes");},
        },
        None => {return Err("couldn't read number of notes line");},
    };

    let mut notes = Vec::with_capacity(number);

    for line in trimmed_lines {
        notes.push( match match line.split_whitespace().next() {
            Some(note_string) => parse_note(note_string),
            None => {return Err("no note on line")},
        } {
            Ok(note) => note,
            Err(message) => {return Err(message)},
        });
    }

    if notes.len() == number {
        Ok(Scale {
            description: description,
            notes: notes,
        })
    }
    else {
        Err("number of notes doesn't match actual number of notes")
    }
}


#[cfg(test)]
mod tests {
    use Note;
    use parse_note;
    use read;
    use Scale;
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

    #[test]
    fn read_valid() {
        assert_eq!(read(
"! meanquar.scl
!
1/4-comma meantone scale. Pietro Aaron's temperament (1523)
 12
!
 76.04900
 193.15686
 310.26471
 5/4 writing stuff here should do nothing
 503.42157
 579.47057
 696.57843
 25/16
 889.73529
 1006.84314
 1082.89214
 2/1"
            ).unwrap(),
            Scale{
                description: "1/4-comma meantone scale. Pietro Aaron's temperament (1523)".to_string(),
                notes: vec![
                    Note::Cents(76.04900),
                    Note::Cents(193.15686),
                    Note::Cents(310.26471),
                    Note::Ratio(Rational32::new(5,4)),
                    Note::Cents(503.42157),
                    Note::Cents(579.47057),
                    Note::Cents(696.57843),
                    Note::Ratio(Rational32::new(25,16)),
                    Note::Cents(889.73529),
                    Note::Cents(1006.84314),
                    Note::Cents(1082.89214),
                    Note::Ratio(Rational32::new(2,1)),
                ],
            }
        );
    }
}
