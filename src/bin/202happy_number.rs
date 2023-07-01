use std::collections::HashSet;

fn main() {
    let n = 19;
    let ans = alt_is_happy(n);
    println!("{ans}");
}
pub fn is_happy(n: i32) -> bool {
    let mut curr_num = n;
    let mut seen = HashSet::new();
    loop {
        let split_nums: Vec<i32> = curr_num
            .to_string()
            .chars()
            .map(|x| x.to_digit(10).unwrap() as i32)
            .collect();
        let square_sum = split_nums.iter().fold(0, |acc, digit| acc + digit.pow(2));
        if square_sum == 1 {
            return true;
        } else if seen.contains(&square_sum) {
            return false;
        }
        curr_num = square_sum;
        seen.insert(square_sum);
    }
}

pub fn alt_is_happy(n: i32) -> bool {
    let mut curr_num = n;
    let mut seen = HashSet::new();
    loop {
        let mut square_sum = 0;
        while curr_num > 0 {
            let digit = curr_num % 10;
            square_sum += digit.pow(2);
            curr_num = curr_num / 10;
        }
        if square_sum == 1 {
            return true;
        } else if seen.contains(&square_sum) {
            return false;
        }
        curr_num = square_sum;
        seen.insert(square_sum);
    }
}

pub fn two_ptr_is_happy(n: i32) -> bool {
    fn get_next(mut x: i32) -> i32 {
        let mut square_sum = 0;
        while x > 0 {
            let digit = x % 10;
            square_sum += digit.pow(2);
            x = x / 10;
        }
        square_sum
    }
    let (mut p1, mut p2) = (n, get_next(n));
    while p1 != 1 && p1 != p2 {
        p1 = get_next(p1);
        p2 = get_next(get_next(p2));
    }
    return p1 == 1;
}

pub fn hardcoded_is_happy(mut n: i32) -> bool {
    let table: [u64; 4] = [
        0x2100190882482,
        0x209248448050,
        0x5001008000000826,
        0x904408010803,
    ];
    while n >= 256 {
        let mut m = 0;
        while n > 0 {
            let d = n % 10;
            m += d * d;
            n /= 10;
        }
        n = m;
    }
    (table[(n / 64) as usize] & (1 << n % 64)) != 0
}
