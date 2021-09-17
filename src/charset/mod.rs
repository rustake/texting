use regex::Regex;

lazy_static! {
    static ref ANSI_ALPHA:String = r"(?:(?:[a-zA-Z\\d]*(?:;[-a-zA-Z\\d\\/#&.:=?%@~_]*)*)?\\u0007)".to_string();
    static ref ANSI_BETA:String = r"(?:(?:\\d{1,4}(?:;\\d{0,4})*)?[\\dA-PR-TZcf-ntqry=><~])".to_string();
    static ref ANSI:String = r"[\\u001B\\u009B][[\\]()#;?]*(?:".to_string() + &ANSI_ALPHA + "|" + &ANSI_BETA + ")";
    static ref ASTRAL:String = r"[\uD800-\uDBFF][\uDC00-\uDFFF]".to_string();
    static ref HAN:String = r"[\u4e00-\u9fa5]|[\uff00-\uffff]".to_string();
    static ref REG_ANSI: Regex = Regex::new(&ANSI).unwrap();
    static ref REG_ASTRAL: Regex = Regex::new(&ASTRAL).unwrap();
    static ref REG_HAN: Regex = Regex::new(&HAN).unwrap();
}

fn has_ansi(text: &str) -> bool { REG_ANSI.is_match(text) }

#[cfg(test)]
mod tests {
    use regex::Regex;
    use veho::vector::Mappers;

    use crate::charset::has_ansi;

    #[test]
    fn test() {}

    #[test]
    fn test_ansi() {
        let ANSI_ALPHA: String = r"(?:(?:[a-zA-Z\d]*(?:;[-a-zA-Z\d\\/#&.:=?%@~_]*)*)?)".to_string();
        let ANSI_BETA: String = r"(?:(?:\d{1,4}(?:;\d{0,4})*)?[\dA-PR-TZcf-ntqry=><~])".to_string();
        let ANSI: String = r"[][[\\]()#;?]*(?:".to_string() + &ANSI_ALPHA + "|" + &ANSI_BETA + ")";
        let reg_ansi: Regex = Regex::new(&ANSI).unwrap();
        println!("reg_ansi is {}", reg_ansi.as_str());
        let candidates = vec![
            "tora",
            "\\u001B[4mcake\\u001B[0m",
            "\\u001b[38;2;255;255;85mTolstoy\\u001b[0m",
            "\\u{1F4A9}",
            "𝐀",
            "I \\u2661 STEAK",
            "\\u001b[3;4;31mhatsu\\u001b[0m",
            "\u{1F3C3}2\u{1F525}7",
            "\\u001B[0m\\u001B[4m\\u001B[42m\\u001B[31mfoo\\u001B[39m\\u001B[49m\\u001B[24mfoo\\u001B[0m🦄bar",
        ];
        (&candidates).iterate(|x| {
            println!("[{}] ({})", x, has_ansi(x))
        })
    }
}