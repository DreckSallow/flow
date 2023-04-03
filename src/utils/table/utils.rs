pub fn num_to_str(n: &u8, text: &str) -> String {
    let mut filler_text = String::new();
    for _ in 0..(*n) {
        filler_text.push_str(text);
    }
    filler_text
}
