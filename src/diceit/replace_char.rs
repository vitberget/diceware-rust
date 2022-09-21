static REPLACE_CHARS: &'static str = "~!#$%^&*()-=+[]\\{}:;\"'<>?/0123456789";

pub fn replace_char(the_word: &String, r2: u8, r3: u8, r4: u8) -> String {
    let mut w2 = String::new();

    for (i, c) in the_word.chars().enumerate() {
        if i + 1 == r2 as usize {
            let rc = REPLACE_CHARS.chars().nth(((r4 - 1) * 6 + (r3 - 1)) as usize).unwrap();
            w2.push(rc);
        } else {
            w2.push(c);
        }
    }

    return w2;
}