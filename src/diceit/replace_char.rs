static REPLACE_CHARS: &'static str = "~!#$%^&*()-=+[]\\{}:;\"'<>?/0123456789";

pub fn replace_char(the_word: &String, r2: u8, r3: u8, r4: u8) -> String {
    let rc = REPLACE_CHARS.chars().nth(((r4 - 1) * 6 + (r3 - 1)) as usize).unwrap();

    let mut w2 = String::new();
    let mut i = 1;
    for c in the_word.chars() {
        if i == r2 {
            w2.push(rc);
        } else {
            w2.push(c);
        }
        i = i + 1;
    }

    return w2;
}