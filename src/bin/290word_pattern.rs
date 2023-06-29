use std::collections::{HashMap, HashSet};

fn main() {}

pub fn word_pattern(pattern: String, s: String) -> bool {
    let mut p_map_s = HashMap::new();
    let mut s_map_p = HashMap::new();
    if pattern.len() != s.split(" ").fold(0, |acc, _| acc + 1) {
        return false;
    }
    for (pc, sw) in pattern.chars().zip(s.split(" ")) {
        if !p_map_s.contains_key(&pc) && !s_map_p.contains_key(&sw) {
            p_map_s.insert(pc, sw);
            s_map_p.insert(sw, pc);
        } else {
            match (p_map_s.get(&pc), s_map_p.get(sw)) {
                (Some(&x), _) => {
                    if x != sw {
                        return false;
                    }
                }
                (_, Some(y)) => {
                    if y != &pc {
                        return false;
                    }
                }
                (None, _) => return false,
            }
        }
    }
    return true;
}

pub fn alt_word_pattern(pattern: String, s: String) -> bool {
    let mut w2l = HashMap::new();
    let mut ls = HashSet::new();
    let w: Vec<_> = s.split_whitespace().collect();
    if w.len() != pattern.len() {
        return false;
    }
    for (r, l) in w.iter().zip(pattern.chars()) {
        if w2l.contains_key(r) {
            if w2l[r] != l {
                return false;
            }
        } else {
            if ls.contains(&l) {
                return false;
            }
            w2l.insert(r, l);
            ls.insert(l);
        }
    }
    true
}
