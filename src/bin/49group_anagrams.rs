use std::collections::HashMap;

fn main() {}
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut groups: HashMap<[u8; 26], Vec<String>> = HashMap::new();
    for s in strs {
        let mut chars = [0; 26];
        for b in s.bytes() {
            chars[(b - b'a') as usize] += 1;
        }
        let entry = groups.entry(chars).or_insert(vec![]);
        entry.push(s);
    }
    groups.into_values().collect()
}
