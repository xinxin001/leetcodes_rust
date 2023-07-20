pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let mut memo = vec![false; s.len()];
    memo.push(true);
    for i in (0..s.len()).rev() {
        for word in &word_dict {
            if i + word.len() <= s.len() && s[i..(i + word.len() as usize)] == word[..] {
                memo[i] = memo[i + word.len()];
            }
            if memo[i] {
                break;
            }
        }
    }
    return memo[0];
}

pub fn alt_word_break(s: String, word_dict: Vec<String>) -> bool {
    let mut memo = vec![false; s.len()];
    for i in 0..s.len() {
        for word in &word_dict {
            if i < word.len() - 1 {
                continue;
            }
            if i == word.len() - 1 || memo[i - word.len()] {
                if s[i - word.len() + 1..i + 1] == word[..] {
                    memo[i] = true;
                    break;
                }
            }
        }
    }
    return *memo.last().unwrap();
}

fn main() {}

#[test]
fn test() {
    let s = "leetcode".to_string();
    let word_dict = vec!["leet".to_string(), "code".to_string()];
    assert_eq!(true, word_break(s, word_dict));
}
