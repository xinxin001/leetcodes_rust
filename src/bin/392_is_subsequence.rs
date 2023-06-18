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

pub fn alt_is_subsequence(s: String, t: String) -> bool {
    let mut siter = s.chars().peekable();
    for c in t.chars() {
        if let Some(sc) = siter.peek() {
            if &c == sc {
                siter.next();
            }
        } else {
            return true;
        }
    }
    !siter.peek().is_some()
}

pub fn vec_is_subsequence(s: String, t: String) -> bool {
    let schars = s.as_bytes();
    if schars.len() == 0 {
        return true;
    }
    let mut idx = 0;
    for c in t.bytes() {
        if c == schars[idx] {
            idx += 1;
        }
        if idx == schars.len() {
            return true;
        }
    }
    return false;
}

fn main() {
    let s = "abc".to_string();
    let t = "ahbgdc".to_string();
    println!("{}", is_subsequence(s, t));
}
