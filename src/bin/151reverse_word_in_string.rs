fn main() {
    let s = "the sky is blue".to_string();
    let ans = reverse_words(s);
    println!("{ans}");
}

pub fn reverse_words(s: String) -> String {
    let arr: Vec<_> = s.split_whitespace().rev().collect();
    arr.join(" ")
}

pub fn alt_reverse_words(s: String) -> String {
    s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
}

/*
In-place operations using the unsafe .as_bytes_mut().
Use two pointers approach to relocate and reverse each word first.
Then truncate the string to result length and reverse the whole string.

Complexity
Time complexity: O(n)
Space complexity: O(1)
*/
pub fn constantspace_reverse_words(mut s: String) -> String {
    s.push(' ');
    let bytes = unsafe { s.as_bytes_mut() };
    let mut length = 0;
    let mut lo = 0;
    for hi in 0..bytes.len() {
        if bytes[hi] == b' ' {
            if hi > lo {
                let word_len = hi - lo;
                for i in 0..word_len {
                    bytes[length + i] = bytes[lo + i];
                }
                bytes[length..length + word_len].reverse();
                bytes[length + word_len] = b' ';
                length += word_len + 1;
            }
            lo = hi + 1;
        }
    }
    s.truncate(length - 1);
    unsafe {
        s.as_bytes_mut().reverse();
    }
    s
}

pub fn inplace_reverse_words(s: String) -> String {
    let mut s: Vec<char> = s.trim().chars().rev().collect();

    s.push(' ');
    let mut i: usize = 0;
    for j in 0..s.len() {
        if s[j] == ' ' {
            s[i..j].reverse();
            i = j + 1;
        }
    }
    s.pop();

    let mut i = 1;
    for j in 1..s.len() {
        if s[j] == ' ' && s[j - 1] == ' ' {
            continue;
        }
        s[i] = s[j];
        i += 1;
    }
    s.truncate(i);
    s.into_iter().collect()
}
