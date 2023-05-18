pub fn is_subsequence(s: String, t: String) -> bool {
    let mut s_iter = s.chars().peekable();
    for c in t.chars() {
        if s_iter.peek().is_some() {
            if &c == s_iter.peek().unwrap() {
                s_iter.next();
            }
        } else {
            return true;
        }
    }
    !s_iter.peek().is_some()
}

fn main() {
    let s = "abc".to_string();
    let t = "ahbgdc".to_string();
    println!("{}", is_subsequence(s, t));
}
