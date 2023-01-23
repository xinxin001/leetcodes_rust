fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }
    let keys = [
        vec!["a", "b", "c"],
        vec!["d", "e", "f"],
        vec!["g", "h", "i"],
        vec!["j", "k", "l"],
        vec!["m", "n", "o"],
        vec!["p", "q", "r", "s"],
        vec!["t", "u", "v"],
        vec!["w", "x", "y", "z"],
    ];
    digits
        .as_bytes()
        .iter()
        .map(|b| *b - b'2')
        .fold(vec![String::new()], |rez, i| {
            keys[i as usize]
                .iter()
                .flat_map(|c|
                    rez.iter().map(move |s| s.clone() + c)
                )
                .collect()
        })
}

fn main() {
    println!("{:?}", letter_combinations("23".to_string()));
}