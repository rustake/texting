use crate::enums::Brac;

pub fn br(text: &str, brac: &Brac) -> String {
    match brac {
        Brac::Par => { format!("({})", text) }
        Brac::Brk => { format!("[{}]", text) }
        Brac::Brc => { format!("{{{}}}", text) }
        Brac::Ang => { format!("<{}>", text) }
        Brac::Nan => { String::from(text) }
    }
}


#[cfg(test)]
mod tests {
    use veho::vector::iterate;

    use super::*;

    #[test]
    fn test() {
        let candidates = [
            Brac::Par,
            Brac::Brk,
            Brac::Brc,
            Brac::Ang,
            Brac::Nan,
        ];
        let text = "refresh yourself";
        iterate(&candidates, |brac| {
            println!("{} {}", text, br(text, brac));
        });
    }
}