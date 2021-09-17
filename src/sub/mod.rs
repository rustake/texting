pub fn sub(text: &str, lo: i32, hi: i32) -> &str {
    let len = text.len() as i32;
    let lo = if lo >= 0 { i32::min(len, lo) } else { i32::max(0, len + lo) } as usize;
    let hi = if hi > 0 { i32::min(len, hi) } else { i32::max(0, len + hi) } as usize;
    return &text[lo..hi];
}