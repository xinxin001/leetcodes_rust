fn main() {
    let s = "wordgoodgoodgoodbestword".to_string();
    let words = vec!["word", "good", "best", "good"]
        .iter()
        .map(|x| x.to_string())
        .collect();
    let ans = alt_find_substring(s, words);
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
