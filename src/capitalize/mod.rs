pub fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().chain(chars).collect(),
    }
}

#[cfg(test)]
mod tests {
    use veho::vector::iterate;

    use super::*;

    #[test]
    fn test() {
        let texts = [
            "",
            "refresh yourself",
            "ßtraß",
        ];
        iterate(&texts, |text| {
            let result = capitalize(text);
            println!("{}", result);
        });
    }
}