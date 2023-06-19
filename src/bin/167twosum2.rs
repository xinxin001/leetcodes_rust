fn main() {}

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut l = 0;
    let mut r = numbers.len() - 1;
    while l < r {
        let twosum = numbers[l] + numbers[r];
        if twosum == target {
            return vec![1 + l as i32, 1 + r as i32];
        } else if twosum < target {
            l += 1
        } else {
            r -= 1;
        }
    }
    return vec![-1, -1];
}
