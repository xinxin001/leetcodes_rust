fn main() {
    let haystack = "sadbutsad".to_string();
    let needle = "sad".to_string();
    let ans = str_str(haystack, needle);
    println!("{ans}");
}

pub fn str_str(haystack: String, needle: String) -> i32 {
    let h = haystack.len();
    let n = needle.len();
    if h < n {
        return -1;
    }
    for i in 0..(h - n + 1) {
        if haystack[i..i + n] == needle {
            return i as i32;
        }
    }
    return -1;
}

pub fn iter_str_str(haystack: String, needle: String) -> i32 {
    match haystack.find(needle.as_str()) {
        Some(v) => v as i32,
        None => -1,
    }
}

pub fn alt_str_str(haystack: String, needle: String) -> i32 {
    haystack.find(&needle).map(|x| x as i32).unwrap_or(-1)
}
