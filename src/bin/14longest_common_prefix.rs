fn main() {
    // let strs = vec![
    //     "flower".to_string(),
    //     "flow".to_string(),
    //     "flight".to_string(),
    // ];
    // let longest = loop_longest_common_prefix(strs);
    // println!("{longest}");

    let s2 = vec!["a".to_string()];
    let l2 = loop_longest_common_prefix(s2);
    println!("{l2}");
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    strs.iter().skip(1).fold(strs[0].clone(), |acc, val| {
        acc.chars()
            .zip(val.chars())
            .take_while(|(x, y)| x == y)
            .map(|(x, _)| x)
            .collect()
    })
}

pub fn loop_longest_common_prefix(strs: Vec<String>) -> String {
    if let Some(first_str) = strs.first() {
        for (i, ch) in first_str.bytes().enumerate() {
            for s in &strs {
                if s.as_bytes().get(i).unwrap_or(&0) != &ch {
                    return first_str[..i].to_string();
                }
            }
        }
        return first_str.clone();
    }
    String::from("")
}
