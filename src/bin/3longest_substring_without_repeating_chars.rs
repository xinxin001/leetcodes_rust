fn main() {}

/*
Optimized sliding window:
time: O(n)
space: O(min(m)) -> m is size of alphabet
*/
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut max_len: usize = 0;

    // [1] longest substring is the one with the largest
    //    difference between positions of repeated characters;
    //    thus, we should create a storage for such positions
    let mut pos: [usize; 128] = [0; 128];

    // [2] while iterating through the string (i.e., moving
    //    the end of the sliding window), we should also
    //    update the start of the window
    let mut start: usize = 0;

    for (end, ch) in s.chars().enumerate() {
        // [3] get the position for the start of sliding window
        //    with no other occurences of 'ch' in it
        start = start.max(pos[ch as usize]);

        // [4] update maximum length
        max_len = max_len.max(end - start + 1);

        // [5] set the position to be used in [3] on next iterations
        pos[ch as usize] = end + 1;
    }

    return max_len as i32;
}

pub fn alt_length_of_longest_substring(s: String) -> i32 {
    let mut maxlen = 0;
    let mut pos = vec![0; 128];
    let mut start = 0;
    for (end, ch) in s.chars().enumerate() {
        start = start.max(pos[ch as usize]);
        maxlen = maxlen.max(end - start + 1);
        pos[ch as usize] = end + 1;
    }
    return maxlen as i32;
}

use std::collections::{HashMap, HashSet};
pub fn hash_length_of_longest_substring(s: String) -> i32 {
    let (mut start, mut end) = (0, 0);

    while end < s.len() {
        end += 1;

        if (&s[start..end])
            .to_string()
            .chars()
            .collect::<HashSet<_>>()
            .len()
            < end - start
        {
            start += 1;
        }
    }

    (end - start) as i32
}

/*
Less optimal but interesting solution
time: O(2n) -> O(n)
space: O(n)
 */
pub fn althash_length_of_longest_substring(s: String) -> i32 {
    let mut chars: HashMap<char, usize> = HashMap::new();
    let mut left = 0;
    let mut res = 0;
    for (i, ch) in s.chars().enumerate() {
        if chars.contains_key(&ch) {
            let nxt_left = chars[&ch] + 1;
            for n in left..=chars[&ch] {
                chars.remove(&s.chars().nth(n).unwrap()).unwrap();
            }
            left = nxt_left;
        }
        chars.insert(ch, i);
        res = res.max(chars.len())
    }
    return res as i32;
}

pub fn althash1_length_of_longest_substring(s: String) -> i32 {
    let mut chars: HashMap<char, usize> = HashMap::new();
    let mut left = 0;
    let mut res = 0;
    for (i, ch) in s.chars().enumerate() {
        if chars.contains_key(&ch) {
            let nxt_left = chars[&ch] + 1;
            for n in left..=chars[&ch] {
                chars.remove(&(s.as_bytes()[n] as char)).unwrap();
            }
            left = nxt_left;
        }
        chars.insert(ch, i);
        res = res.max(chars.len())
    }
    return res as i32;
}
