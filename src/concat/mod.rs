use std::fmt;
use std::fmt::Write;

use regex::Regex;

use crate::bracket::br;
use crate::chars::{LF, SP, TB, VO};
use crate::enums::Brac;

lazy_static! {
    static ref LINEFEED: Regex = Regex::new(r"\n").unwrap();
    static ref COMMA: Regex = Regex::new(r",").unwrap();
}

pub trait Concat: IntoIterator {
    fn join(self, delim: &str) -> String where Self: Sized,
                                               Self::IntoIter: Iterator<Item=Self::Item>,
                                               Self::Item: fmt::Display
    {
        let mut iter = self.into_iter();
        match iter.next() {
            None => String::new(),
            Some(first) => {
                let (lb, _) = iter.size_hint(); // estimate lower bound of capacity needed
                let mut phrase = String::with_capacity(delim.len() * lb);
                write!(&mut phrase, "{}", first).unwrap();
                iter.for_each(|elt| { write!(&mut phrase, "{}{}", delim, elt).unwrap(); }); // (&mut phrase).push_str(delim);
                phrase
            }
        }
    }
    fn concat_lines(self, delim: &str, level: u8) -> String where Self: Sized,
                                                                  Self::IntoIter: Iterator<Item=Self::Item>,
                                                                  Self::Item: fmt::Display
    {
        let ind = if level > 0 { SP.repeat((level as usize) << 1) } else { String::new() };
        let de = format!("{}{}{}{}", delim, LF, ind, TB);
        let joined = self.into_iter().join(&de);
        return format!("{}{}{}{}", ind, TB, joined, delim);
    }
    fn concat_lines_edged(self, delim: &str, level: u8) -> String where Self: Sized,
                                                                        Self::IntoIter: Iterator<Item=Self::Item>,
                                                                        Self::Item: fmt::Display
    {
        let ind = if level > 0 { SP.repeat((level as usize) << 1) } else { String::new() };
        let de = format!("{}{}{}{}", delim, LF, ind, TB);
        let joined = self.into_iter().join(&de);
        return format!("{}{}{}{}{}{}{}", LF, ind, TB, joined, delim, LF, ind);
    }
    fn contingent_lines(self, delim: &str, level: u8, brac: &Brac) -> String where Self: Sized,
                                                                                   Self::IntoIter: Iterator<Item=Self::Item>,
                                                                                   Self::Item: fmt::Display
    {
        let iter = self.into_iter();
        let edged = match brac { Brac::Nan => false, _ => true };
        let joined: String = if LINEFEED.is_match(delim) {
            let de = if COMMA.is_match(delim) { delim.replace(LF, VO) } else { String::new() };
            if edged {
                iter.concat_lines_edged(&de, level)
            } else {
                iter.concat_lines(&de, level)
            }
        } else {
            iter.join(delim)
        };
        return br(&joined, brac);
    }
}

impl<I> Concat for I where I: IntoIterator {}


pub fn join<I>(vec: I, delim: &str) -> String where I: IntoIterator,
                                                    I: Sized,
                                                    I::IntoIter: Iterator<Item=I::Item>,
                                                    I::Item: fmt::Display
{ vec.join(delim) }

pub fn concat_lines<I>(vec: I, delim: &str, level: u8) -> String where I: IntoIterator,
                                                                       I: Sized,
                                                                       I::IntoIter: Iterator<Item=I::Item>,
                                                                       I::Item: fmt::Display
{ vec.concat_lines(delim, level) }

pub fn concat_lines_edged<I>(vec: I, delim: &str, level: u8) -> String where I: IntoIterator,
                                                                             I: Sized,
                                                                             I::IntoIter: Iterator<Item=I::Item>,
                                                                             I::Item: fmt::Display
{ vec.concat_lines_edged(delim, level) }

pub fn contingent_lines<I>(vec: I, delim: &str, level: u8, brac: &Brac) -> String where I: IntoIterator,
                                                                                        I: Sized,
                                                                                        I::IntoIter: Iterator<Item=I::Item>,
                                                                                        I::Item: fmt::Display
{ vec.contingent_lines(delim, level, brac) }


#[cfg(test)]
mod tests {
    use veho::vector::Mappers;

    use crate::concat::{Concat, contingent_lines};
    use crate::enums::Brac;

    #[test]
    fn test_crostab_simplified() {
        let years = vec!["2004", "1984", "1964"].mapper(String::from);

        println!("years = {}", Concat::join(years.iter(), ", "));
        println!("years = [{}]", (&years).concat_lines(", ", 0));
        println!("years.len = {}", years.len());

        let words = vec!["foo", "bar", "zen"];
        println!("words = [{}]", (&words).concat_lines_edged(", ", 0));
        println!("words.len = {}", words.len());
    }

    #[test]
    fn test_contingent_lines() {
        let years = vec!["2004", "1984", "1964"].mapper(String::from);

        println!("contingent_lines = {}", (&years).contingent_lines(",\n", 0, &Brac::Brk));
        println!("years.len = {}", years.len());
        println!("contingent_lines = {}", (&years).contingent_lines("\n", 0, &Brac::Nan));
        println!("years.len = {}", years.len());
        println!("contingent_lines = {}", contingent_lines(&years, ", ", 0, &Brac::Brk));
        println!("years.len = {}", years.len());
    }

    #[test]
    fn test_regex() {
        use regex::Regex;
        let linefeed = Regex::new(r"\n").unwrap();
        let comma = Regex::new(r",").unwrap();
        let candidates = vec!["shakespeare, william", "william shakespeare", "alpha\nbeta", "alpha beta"];
        candidates.iterate(|x| {
            println!("[{}] contains LINEFEED ({}) contains COMMA ({})", x, linefeed.is_match(x), comma.is_match(x));
        });
    }
}