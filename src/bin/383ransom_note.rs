use std::collections::HashMap;

fn main() {
    let ransom_note = "aa".to_string();
    let magazine = "aab".to_string();
    let ans = can_construct(ransom_note, magazine);
    println!("{ans}");
}

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut m_count: HashMap<u8, usize> = HashMap::new();
    let mut r_count: HashMap<u8, usize> = HashMap::new();
    for &c in ransom_note.as_bytes() {
        *r_count.entry(c).or_default() += 1;
    }
    for &c in magazine.as_bytes() {
        *m_count.entry(c).or_default() += 1;
    }
    for (key, value) in r_count.iter() {
        let m = m_count.get(key).unwrap_or(&0);
        if value > m {
            return false;
        }
    }
    return true;
}

pub fn alt_can_construct(ransom_note: String, magazine: String) -> bool {
    let mut freqs = [0; 26];
    for c in ransom_note.chars() {
        freqs[c as usize - b'a' as usize] -= 1;
    }
    for c in magazine.chars() {
        freqs[c as usize - b'a' as usize] += 1;
    }
    freqs.iter().all(|&x| x >= 0)
}
