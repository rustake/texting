use std::fmt;
use std::fmt::Write;

use crate::chars::{LF, SP, TB};

pub trait Join: IntoIterator {
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
}

impl<I> Join for I where I: Iterator {}

pub trait Concat: IntoIterator {
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
}

impl<I> Concat for I where I: IntoIterator {}

// pub fn index_of<I>(vec: I, item: I::Item) -> Option<usize> where I: IntoIterator,
//                                                                  I: Sized,
//                                                                  I::IntoIter: Iterator<Item=I::Item>,
//                                                                  I::Item: fmt::Display
// { vec.index_of(item) }


#[cfg(test)]
mod tests {
    use veho::vector::Mappers;

    use crate::concat::{Concat, Join};

    #[test]
    fn test_crostab_simplified() {
        let years = vec!["2004", "1984", "1964"].mapper(String::from);

        println!("years = {}", Join::join(years.iter(), ", "));
        println!("years = [{}]", (&years).concat_lines(", ", 0));
        println!("years.len = {}", years.len());

        let words = vec!["foo", "bar", "zen"];
        println!("words = [{}]", (&words).concat_lines_edged(", ", 0));
        println!("words.len = {}", words.len());
    }
}