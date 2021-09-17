fn v1(word: &str) -> u32 {
    let word = &word.to_lowercase();
    let it = &mut word.chars();
    let c1 = it.next().unwrap() as u32;
    return (c1 & 0x7f) << 21;
}

fn v2(word: &str) -> u32 {
    let word = &word.to_lowercase();
    let it = &mut word.chars();
    let c1 = it.next().unwrap() as u32;
    let c2 = it.next().unwrap() as u32;
    return ((c1 & 0x7f) << 21) + ((c2 & 0x7f) << 14);
}

fn v3(word: &str) -> u32 {
    let word = &word.to_lowercase();
    let it = &mut word.chars();
    let c1 = it.next().unwrap() as u32;
    let c2 = it.next().unwrap() as u32;
    let c3 = it.next().unwrap() as u32;
    return ((c1 & 0x7f) << 21) + ((c2 & 0x7f) << 14) + ((c3 & 0x7f) << 7);
}

fn v4(word: &str) -> u32 {
    let word = &word.to_lowercase();
    let it = &mut word.chars();
    let c1 = it.next().unwrap() as u32;
    let c2 = it.next().unwrap() as u32;
    let c3 = it.next().unwrap() as u32;
    let c4 = it.next().unwrap() as u32;
    return ((c1 & 0x7f) << 21) + ((c2 & 0x7f) << 14) + ((c3 & 0x7f) << 7) + (c4 & 0x7f);
}

pub fn str_value(word: &str) -> u32 {
    let len = word.len();
    match len {
        n if n >= 8 => { (v4(&word[..4]) << 2) + v4(&word[len - 4..]) }
        7 => { (v4(&word[..4]) << 2) + v3(&word[4..]) }
        6 => { (v4(&word[..4]) << 2) + v2(&word[4..]) }
        5 => { (v4(&word[..4]) << 2) + v1(&word[4..]) }
        4 => { v4(word) << 2 }
        3 => { v3(word) << 2 }
        2 => { v2(word) << 2 }
        1 => { v1(word) << 2 }
        _ => { 0 }
    }
}


#[cfg(test)]
mod tests {
    use veho::vector::iterate;

    use super::*;

    #[test]
    fn test() {
        let texts = [
            "Warren",
            "WSJ",
            "GlobalTimes",
            "ZZZZ",
            "zzzz",
            "MetalGear 1",
            "MetalGear 2",
            "123",
            "abc",
            "---",
        ];
        iterate(&texts, |word| {
            println!("{}: {}", word, str_value(word))
        })
    }
}