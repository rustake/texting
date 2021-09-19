use std::borrow::Cow;

use regex::Regex;

lazy_static! {
    static ref CJK_PUNCS:String = r"\u3000-\u303f".to_string();
    static ref CJK_LETTERS:String = r"\u4e00-\u9fbf".to_string();
    static ref FULL_CHARS:String = r"\uff00-\uffef".to_string(); // full letters + full puncs
    static ref HAN:String = r"[".to_string() + &CJK_PUNCS + &CJK_LETTERS + &FULL_CHARS + "]";
    pub static ref REG_HAN: Regex = Regex::new(&HAN).unwrap();
    pub static ref REG_ANSI: Regex = Regex::new(r"[\u{1b}\u{9b}][\[()#;?]*(?:[0-9]{1,4}(?:;[0-9]{0,4})*)?[0-9A-ORZcf-nqry=><]").unwrap();
    pub static ref REG_ASTRAL: Regex = Regex::new(r"\p{Extended_Pictographic}").unwrap();
}

pub fn has_ansi(text: &str) -> bool { REG_ANSI.is_match(text) }

pub fn has_astral(text: &str) -> bool { REG_ASTRAL.is_match(text) }

pub fn has_han(text: &str) -> bool { REG_HAN.is_match(text) }

pub fn strip_ansi(text: &str) -> Cow<str> { REG_ANSI.replace_all(text, "") }

pub fn strip_astral(text: &str) -> Cow<str> { REG_ASTRAL.replace_all(text, "") }

// pub fn strip_han(text: &str) -> Cow<str> { REG_HAN.replace_all(text, "") }

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use veho::vector::Mappers;

    use crate::charset::{has_ansi, has_astral, REG_ANSI, REG_ASTRAL, strip_ansi, strip_astral};


    #[test]
    fn test_ansi() {
        println!("reg_ansi is {}", REG_ANSI.as_str());
        println!("reg_astral is {}", REG_ASTRAL.as_str());
        let candidates = vec![
            "tora",
            "\u{001B}[4mcake\u{001B}[0m",
            "\x1b[38;2;255;255;85mTolstoy\x1b[0m",
            "\u{1F4A9}",
            "𝐀",
            "I \u{2661} STEAK",
            "I \u{2661} 牛排",
            "\u{001b}[3;4;31mhatsu\u{001b}[0m",
            "\u{1F3C3}2\u{1F525}7",
            "\u{001B}[0m\u{001B}[4m\u{001B}[42m\u{001B}[31mfoo\u{001B}[39m\u{001B}[49m\u{001B}[24mfoo\u{001B}[0m🦄bar",
        ];
        println!("has");
        (&candidates).iterate(|x| {
            println!("[{}] ANSI ({}) ASTRAL ({})", x, has_ansi(x), has_astral(x));
            // for caps in reg_astral.captures_iter(x) { println!("year: {:?}", caps); };
        });

        println!();
        println!("strip ansi");
        (&candidates).iterate(|x| {
            let ansi_stripped = strip_ansi(x);
            let astral_stripped = strip_astral(ansi_stripped.borrow());
            println!("[{:?}] -> [{:?}] ({})", x, astral_stripped, astral_stripped.len())
        })
    }
}