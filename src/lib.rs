//! This crate is for reading and writing Scala scale files (.scl).
//! http://www.huygens-fokker.org/scala/scl_format.html


extern crate num;

pub type RationalUint = num::rational::Ratio<u32>;

#[derive(Debug, PartialEq, Clone)]
pub enum Note {
    Cents(f64),
    Ratio(RationalUint),
}

impl std::fmt::Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            &Note::Cents(cents) => try!(cents.fmt(f)),
            &Note::Ratio(ratio) => try!(write!(f,"{}/{}",ratio.numer(),ratio.denom())),
        };
        Ok( () )
    }
}

impl std::str::FromStr for Note {
    type Err = &'static str;
    fn from_str(string: &str) -> Result<Note, &'static str> {
        if string.contains(".") {
            match string.parse::<f64>() {
                Ok(cents) => Ok(Note::Cents(cents)),
                Err(_) => Err("error parsing cent value"),
            }
        }
        else {
            match string.parse::<RationalUint>() {
                Ok(ratio) => {
                    Ok(Note::Ratio(ratio))
                }
                Err(_) => Err("error parsing ratio value")
            }
        }
    }
}

/// The description must hold on a single line and the ratios in the Note::Ratio must be positive
#[derive(Debug, PartialEq, Clone)]
pub struct Scale {
    pub description: String,
    pub notes: Vec<Note>,
}

impl std::fmt::Display for Scale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        try!(self.description.fmt(f));
        try!(writeln!(f,""));

        try!(self.notes.len().fmt(f));
        //try!(writeln!(f,""));

        for note in & self.notes {
            try!(writeln!(f,""));
            try!(note.fmt(f));
        }

        Ok( () )
    }
}

impl std::str::FromStr for Scale {
    type Err = &'static str;
    fn from_str(scale_string: &str) -> Result<Scale, &'static str> {
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
                Some(note_string) => Note::from_str(note_string),
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
}


#[cfg(test)]
mod tests {
    use Note;
    use Scale;
    use RationalUint;
    use std::str::FromStr;

    extern crate quickcheck;


    #[test]
    fn parse_note_valid_input() {
        assert_eq!(Note::from_str("0.0").unwrap(), Note::Cents(0.0f64));
        assert_eq!(Note::from_str("0.").unwrap(), Note::Cents(0.0f64));
        assert_eq!(Note::from_str(".0").unwrap(), Note::Cents(0.0f64));
        assert_eq!(Note::from_str("0.5").unwrap(), Note::Cents(0.5f64));
        assert_eq!(Note::from_str("1200.").unwrap(), Note::Cents(1200.0f64));
        

        assert_eq!(Note::from_str("1").unwrap(), Note::Ratio(RationalUint::new(1,1)));
        assert_eq!(Note::from_str("2").unwrap(), Note::Ratio(RationalUint::new(2,1)));
        assert_eq!(Note::from_str("1/3").unwrap(), Note::Ratio(RationalUint::new(1,3)));
        assert_eq!(Note::from_str("2/3").unwrap(), Note::Ratio(RationalUint::new(2,3)));

        assert_eq!(Note::from_str("2147483647/3").unwrap(),
            Note::Ratio(RationalUint::new(2147483647,3)));
    }

    #[test]
    fn parse_note_not_valid_input() {
        Note::from_str("").unwrap_err();
        Note::from_str("a").unwrap_err();
        Note::from_str("a1.32").unwrap_err();
        Note::from_str("gourd").unwrap_err();

        Note::from_str("-1/2").unwrap_err();
    }

    #[test]
    fn read_scale_valid() {
        assert_eq!(Scale::from_str(
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
 2  "
            ).unwrap(),
            Scale{
                description: "1/4-comma meantone scale. Pietro Aaron's temperament (1523)".to_string(),
                notes: vec![
                    Note::Cents(76.04900),
                    Note::Cents(193.15686),
                    Note::Cents(310.26471),
                    Note::Ratio(RationalUint::new(5,4)),
                    Note::Cents(503.42157),
                    Note::Cents(579.47057),
                    Note::Cents(696.57843),
                    Note::Ratio(RationalUint::new(25,16)),
                    Note::Cents(889.73529),
                    Note::Cents(1006.84314),
                    Note::Cents(1082.89214),
                    Note::Ratio(RationalUint::new(2,1)),
                ],
            }
        );
        assert_eq!(Scale::from_str(
                "test zero notes
 0
!"
            ).unwrap(),
            Scale{
                description: "test zero notes".to_string(),
                notes: vec![ ],
            }
        );
    }

    #[test]
    fn read_scale_not_valid() {
        "".parse::<Scale>().unwrap_err();
        "!".parse::<Scale>().unwrap_err();
        "asdfasdf".parse::<Scale>().unwrap_err();

"! 
! ffdf".parse::<Scale>().unwrap_err();

"ffdf
asd".parse::<Scale>().unwrap_err();

"ffdf
-2".parse::<Scale>().unwrap_err();

"ffdt
1
-1/2".parse::<Scale>().unwrap_err();

"ffdt 
0
1/2".parse::<Scale>().unwrap_err();

    }

    #[test]
    fn write_scale() {
        let written =
            &(Scale{
                description: "1/4-comma meantone scale. Pietro Aaron's temperament (1523)".to_string(),
                notes: vec![
                    Note::Cents(76.04900),
                    Note::Cents(193.15686),
                    Note::Cents(310.26471),
                    Note::Ratio(RationalUint::new(5,4)),
                    Note::Cents(503.42157),
                    Note::Cents(579.47057),
                    Note::Cents(696.57843),
                    Note::Ratio(RationalUint::new(25,16)),
                    Note::Cents(889.73529),
                    Note::Cents(1006.84314),
                    Note::Cents(1082.89214),
                    Note::Ratio(RationalUint::new(2,1)),
                ],
            }.to_string());

        let supposed =
"1/4-comma meantone scale. Pietro Aaron's temperament (1523)
12
76.049
193.15686
310.26471
5/4
503.42157
579.47057
696.57843
25/16
889.73529
1006.84314
1082.89214
2/1";
        println!("{}",written);
        println!("{}",supposed);

        assert_eq!(written, supposed);
    }


    // note, at this point quickcheck only generates "well-behaving" values between -100 and 100 so
    // it would be best to check if this works for crazier values as well
    impl quickcheck::Arbitrary for Note {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Note {
            if g.gen() {
                Note::Cents(quickcheck::Arbitrary::arbitrary(g))
            }
            else {
                Note::Ratio(RationalUint::new(quickcheck::Arbitrary::arbitrary(g),
                                              {let den = quickcheck::Arbitrary::arbitrary(g);
                                                  if den==0 {1} else {den}
                                              }))
            }
        }
    }

    fn write_then_read_note(note: Note) -> bool {
        //println!("{:?}", note);
        (note.to_string().parse::<Note>()).unwrap() == note
    }

    #[test]
    fn quickcheck_write_then_read_notes() {
        quickcheck::quickcheck(write_then_read_note as fn(Note) -> bool)
    }

    // TODO make a quickcheck test for writing and reading a scale.

}
