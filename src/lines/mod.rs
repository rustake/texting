use std::fmt;

use crate::chars::TB;

pub fn join_lines<I>(lines: I, delim: &str, level: usize, hover: bool) where
    I: IntoIterator,
    I::Item: fmt::Display
{
    let ind = TB.repeat(level);
    println!("{}", ind);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join_lines() {
        let lines = vec!["a", "b", "c"];
        join_lines(&lines, ", ", 1, false);
    }
}