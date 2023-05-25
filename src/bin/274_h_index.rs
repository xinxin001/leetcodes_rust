pub fn h_index(citations: Vec<i32>) -> i32 {
    let mut citations = citations;
    citations.sort();
    for i in (0..citations.len()).rev() {
        if citations[citations.len() - i - 1] > i as i32 {
            return i as i32 + 1;
        }
    }
    0
}

pub fn alt_h_index(citations: Vec<i32>) -> i32 {
    let mut citations = citations;
    citations.sort_by(|a, b| b.cmp(a));
    let mut h = 0;
    for i in 0..citations.len() {
        if citations[i] > i as i32 {
            h = i + 1;
        }
    }
    h as i32
}

pub fn main() {}
