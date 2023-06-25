fn main() {
    let s = "wordgoodgoodgoodbestword".to_string();
    let words = vec!["word", "good", "best", "good"]
        .iter()
        .map(|x| x.to_string())
        .collect();
    let ans = sliding_find_substring(s, words);
    dbg!(ans);
}

/*
FAILED SOLUTION:
I thought I could just take in chunks of strings as im sliding the window,
turns out I need to slide char by char.
 */
use std::{
    collections::{HashMap, VecDeque},
    str::from_utf8,
};
pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    let wordlen = words.first().unwrap().len();
    let mut wcount: HashMap<&str, i32> = HashMap::new();
    for word in &words {
        let count = wcount.entry(word).or_insert(0);
        *count += 1;
    }
    let mut deque: VecDeque<&str> = VecDeque::new();
    let mut window_count = HashMap::new();
    let mut ans = vec![];
    for start in (0..s.len()).step_by(wordlen) {
        let right = start + wordlen;
        let ss = &s[start..right];
        deque.push_back(ss);
        let ss_count = window_count.entry(ss).or_insert(0);
        *ss_count += 1;
        if deque.len() > words.len() {
            let out_ss = deque.pop_front().unwrap();
            let out_count = window_count.get_mut(out_ss).unwrap();
            *out_count -= 1;
            if *out_count == 0 {
                window_count.remove(out_ss);
            }
        }
        if deque.len() == words.len() {
            dbg!(&deque);
            if window_count == wcount {
                ans.push((start - (words.len() * wordlen - wordlen)) as i32)
            }
        }
    }
    return ans;
}

/*
Correct solution
Given n as the length of s, a as the length of words, and b as the length of each word.
time: O(n*a*b - (a*b)^2)
space: O(a+b)
 */
pub fn alt_find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    let wordlen = words.first().unwrap().len();
    let window_size = words.len() * wordlen;
    let mut wcount: HashMap<&str, i32> = HashMap::new();
    for word in &words {
        let count = wcount.entry(word).or_insert(0);
        *count += 1;
    }
    let mut ans = vec![];
    for start in 0..s.len() - window_size + 1 {
        let right = start + window_size;
        let ss = &s[start..right];
        let swords = ss
            .as_bytes()
            .chunks(wordlen)
            .map(from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap();
        let mut scount = HashMap::new();
        for sw in swords {
            let sc = scount.entry(sw).or_insert(0);
            *sc += 1;
        }
        if scount == wcount {
            ans.push(start as i32)
        }
    }
    return ans;
}

pub fn sliding_find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    // Input is ASCII => chars are bytes
    let s = s.as_bytes();
    let n = s.len();
    let k = words[0].len();
    // Early exit if word length is greater than length of s
    if k > n {
        return vec![];
    }
    // Build map from word (as bytes) to (frequency of word in words, recorded frequency in window)
    let mut map = words
        .iter()
        .fold(HashMap::<&[u8], (usize, usize)>::new(), |mut map, word| {
            map.entry(word.as_bytes()).or_default().0 += 1;
            map
        });
    // Flag to tell if map is reset to avoid resetting a map that is already reset
    dbg!(&map);
    let mut map_is_reset = true;
    let mut rez = vec![];
    // We have to run the sliding window algorithm with every offset in the word length
    for i in 0..k {
        // Reset window word frequency if needed
        if !map_is_reset {
            map.iter_mut().for_each(|(_, value)| value.1 = 0);
            map_is_reset = true;
        }
        // Initialize empty window at start position
        let (mut lo, mut hi) = (i, i);
        while hi <= n - k {
            match map.get_mut(&s[hi..hi + k]) {
                None => {
                    // No match at current hi position - reset window at next word boundary
                    hi += k;
                    lo = hi;
                    if !map_is_reset {
                        map.iter_mut().for_each(|(_, value)| value.1 = 0);
                        map_is_reset = true;
                    }
                }
                Some(hi_value) => {
                    // Word found - update recorded word frequency
                    hi_value.1 += 1;
                    hi += k;
                    map_is_reset = false;
                    // If word frequency in current window is too high we have to slide lo
                    // forward until we decrease the frequency of the word encountered at hi
                    // back to the upper limit.
                    if hi_value.1 > hi_value.0 {
                        loop {
                            let lo_value = map.get_mut(&s[lo..lo + k]).unwrap();
                            lo += k;
                            lo_value.1 -= 1;
                            if lo_value.0 == lo_value.1 {
                                break;
                            }
                        }
                    }
                }
            }

            // The window always contains a valid count of words at this point, so if the window
            // is so large that it can contain all the words, lo is a match to be recorded in the
            // return value.
            if hi - lo == words.len() * k {
                rez.push(lo as i32);
                map.get_mut(&s[lo..lo + k]).unwrap().1 -= 1;
                lo += k;
            }
        }
    }
    rez
}
