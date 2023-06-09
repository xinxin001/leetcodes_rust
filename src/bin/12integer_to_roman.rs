fn main() {
    let num = 1994;
    let ans = int_to_roman(num);
    println!("{ans}");
}

pub fn int_to_roman(num: i32) -> String {
    let map = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
    let mut largest_idx = 0;
    let mut remainder = num;
    let mut ans: Vec<&str> = vec![];
    while remainder > 0 {
        let current_largest = map[largest_idx].0;
        if remainder >= current_largest {
            ans.push(map[largest_idx].1);
            remainder -= current_largest;
        } else {
            if largest_idx < map.len() - 1 {
                largest_idx += 1
            }
        }
    }
    ans.join("")
}
