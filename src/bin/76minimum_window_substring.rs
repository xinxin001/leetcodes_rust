use std::collections::HashMap;

fn main() {
    let s = "ADOBECODEBANC".to_string();
    let t = "ABC".to_string();
    let ans = min_window(s, t);
    dbg!(ans);
}

/*
Strategy:
Expand window from the right,
as you have matched all necessary chars of target, start shrinking window from the left,
Keep track of smallest window possible.
Keep expanding right after the shrinking, to explore other possible windows
 */
pub fn min_window(s: String, t: String) -> String {
    let n = s.len();
    let s = s.as_bytes();
    let t = t.as_bytes();
    let mut target: HashMap<u8, usize> = HashMap::new();
    for &b in t {
        *target.entry(b).or_default() += 1;
    }
    let mut window: HashMap<u8, usize> = HashMap::new();
    let (mut min_len, mut min_l, mut min_r) = (n + 1, 0, 0);
    let mut to_match = target.len();
    let mut left = 0;
    for (right, &c) in s.iter().enumerate() {
        if !target.contains_key(&c) {
            continue;
        }
        *window.entry(c).or_default() += 1;

        if window[&c] == target[&c] {
            to_match -= 1;
        }

        if to_match == 0 {
            while !target.contains_key(&s[left]) || window[&s[left]] > target[&s[left]] {
                if window.contains_key(&s[left]) {
                    *window.entry(s[left]).or_default() -= 1;
                }
                left += 1;
            }
            let window_len = right - left + 1;
            if min_len > window_len {
                min_len = window_len;
                (min_l, min_r) = (left, right);
            }
        }
    }
    if to_match > 0 {
        "".to_string()
    } else {
        String::from_utf8(s[min_l..=min_r].to_vec()).unwrap()
    }
}

pub fn alt_min_window(s: String, t: String) -> String {
    let s = s.as_bytes();
    let t = t.as_bytes();

    let n = s.len();
    let mut target = HashMap::new();
    for &c in t {
        *target.entry(c).or_insert(0) += 1;
    }
    let mut window: HashMap<u8, usize> = HashMap::new();
    let (mut min_len, mut min_l, mut min_r) = (n + 1, 0, 0);
    let mut to_match = target.len();

    let mut l = 0;
    for (r, &c) in s.iter().enumerate() {
        if !target.contains_key(&c) {
            continue;
        }
        *window.entry(c).or_insert(0) += 1;

        if window.get(&c) == target.get(&c) {
            to_match -= 1;
        }

        if to_match == 0 {
            while !target.contains_key(&s[l])
                || window.get(&s[l]).unwrap() > target.get(&s[l]).unwrap()
            {
                if let Some(count) = window.get_mut(&s[l]) {
                    *count -= 1;
                    if *count == 0 {
                        window.remove(&s[l]);
                    }
                }
                l += 1;
            }
            let window_len = r - l + 1;
            if min_len > window_len {
                min_len = window_len;
                min_l = l;
                min_r = r + 1;
            }
        }
    }

    if min_len == n + 1 {
        String::new()
    } else {
        std::str::from_utf8(&s[min_l..min_r]).unwrap().to_owned()
    }
}
