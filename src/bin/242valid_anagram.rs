fn main() {}

pub fn is_anagram(s: String, t: String) -> bool {
    let mut chars = [0; 26];
    for b in s.bytes() {
        chars[(b - b'a') as usize] += 1;
    }
    for b in t.bytes() {
        chars[(b - b'a') as usize] -= 1;
    }
    chars.iter().all(|x| x == &0)
}

pub fn sort_is_anagram(s: String, t: String) -> bool {
    let mut ss = s.chars().collect::<Vec<_>>();
    let mut st = t.chars().collect::<Vec<_>>();
    ss.sort();
    st.sort();
    return ss == st;
}
