static REPLACE_CHARS: &'static str = "~!#$%^&*()-=+[]\\{}:;\"'<>?/0123456789";

pub(crate) fn replace_char(the_word: &String, replace_char_roll: u8, new_char_roll_1: u8, new_char_roll_2: u8) -> String {
    the_word
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i + 1 == replace_char_roll as usize {
                REPLACE_CHARS
                    .chars()
                    .nth(((new_char_roll_2 - 1) * 6 + (new_char_roll_1 - 1)) as usize)
                    .unwrap()
            } else {
                c
            }
        })
        .collect()
}
