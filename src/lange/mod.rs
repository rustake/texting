use crate::charset::{REG_ANSI, REG_ASTRAL};

pub fn lange(text: &str) -> usize {
    let text = REG_ANSI.replace_all(text, "");
    let text = REG_ASTRAL.replace_all(text.as_ref(), "_");
    return text.len();
}

#[cfg(test)]
mod tests {
    use veho::vector::Mappers;

    use crate::lange::lange;

    #[test]
    fn test_ansi() {
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
            println!("[{}] ({})", x, lange(x))
        });
    }
}