use std::collections::HashMap;

fn main() {
    let s = "MCMXCIV";
    let res = roman_to_int(s.to_string());
    println!("{res}");
}

pub fn roman_to_int(s: String) -> i32 {
    let map = HashMap::from([
        (b'I', 1),
        (b'V', 5),
        (b'X', 10),
        (b'L', 50),
        (b'C', 100),
        (b'D', 500),
        (b'M', 1000),
    ]);

    let mut total = 0;
    let chars = s.as_bytes();
    for (i, c) in chars.iter().enumerate() {
        total += map[&c];
        if i > 0 {
            let prev_num = map[&chars[i - 1]];
            if prev_num < map[&c] {
                total -= prev_num * 2
            }
        }
    }
    total
}

fn lookup_number(roman_numeral: char) -> u16 {
    match roman_numeral {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0,
    }
}

pub fn alt_roman_to_int(s: String) -> i32 {
    let mut last = ' ';
    s.chars().fold(0, |acc, character| {
        let first = lookup_number(character);
        let second = lookup_number(last);
        if first <= second {
            last = character;
            acc + first as i32
        } else {
            last = character;
            acc + first as i32 - second as i32 * 2
        }
    })
}
