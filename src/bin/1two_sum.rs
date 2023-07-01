fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let ans = two_sum(nums, target);
    println!("{ans:?}");
}
use std::collections::HashMap;
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut sums = HashMap::new();
    for (idx, num) in nums.iter().enumerate() {
        let diff = target - num;
        if sums.contains_key(&diff) {
            return vec![*sums.get(&diff).unwrap() as i32, idx as i32];
        } else {
            sums.insert(num, idx);
        }
    }
    unreachable!();
}
