fn main() {}
pub fn length_of_last_word(s: String) -> i32 {
    let arr = s.split_ascii_whitespace();
    let last_word = arr.last().unwrap();
    return last_word.len() as i32;
}

pub fn clean_length_of_last_word(s: String) -> i32 {
    match s.split_whitespace().last() {
        Some(word) => word.len() as i32,
        None => 0,
    }
}

pub fn oneliner_length_of_last_word(s: String) -> i32 {
    s.trim_end()
        .chars()
        .rev()
        .take_while(|c| c.is_alphabetic())
        .count() as i32
}
