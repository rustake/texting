fn sub(text: &str, lo: i32, hi: i32) -> &str {
    let len = text.len() as i32;
    let lo = if lo >= 0 { i32::min(len, lo) } else { i32::max(0, len + lo) } as usize;
    let hi = if hi > 0 { i32::min(len, hi) } else { i32::max(0, len + hi) } as usize;
    return &text[lo..hi];
}

macro_rules! sub {
    ($text:expr, $lo:expr) => {
       return if $lo >= 0 {
            &$text[($lo as usize)..]
        } else {
            &$text[($text.len() + $lo as usize)..]
        };
        // println!("{}, {}",text, lo);
    };
    ($text:expr, $lo:expr, $hi:expr) => {
        println!("{}, {}, {}", $text, $lo, $hi);
    };
}

#[test]
fn last_substring_test() {
    let slice = "GeForce RTX";
    println!("{:?}", sub(slice, 2, 7));
    println!("{:?}", sub(slice, 0, 7));
    println!("{:?}", sub(slice, -3, 0));
    println!("{:?}", sub(slice, 0, -4));
    println!("{:?}", sub(slice, 0, 0));
}