fn main() {}

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

use std::collections::HashSet;
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
