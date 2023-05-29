static REPLACE_CHARS: &'static str = "~!#$%^&*()-=+[]\\{}:;\"'<>?/0123456789";

pub(crate) fn replace_char(the_word: &String, r2: u8, r3: u8, r4: u8) -> String {
    the_word.chars().enumerate()
        .map(|(i, c)| {
            if i + 1 == r2 as usize {
                REPLACE_CHARS.chars().nth(((r4 - 1) * 6 + (r3 - 1)) as usize).unwrap()
            } else {
                c
            }
        }).collect()
}
