use std::collections::HashMap;

fn main() {}

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, usize> = HashMap::new();
    let n = nums.len();
    for num in nums {
        let entry = map.entry(num).and_modify(|x| *x += 1).or_insert(0);
        if *entry >= (n / 2) {
            return num;
        }
    }
    unreachable!();
}

pub fn iter_majority_element(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut major = 0;

    nums.iter().for_each(|&num| {
        if num == major {
            count += 1;
        } else {
            count -= 1;
        };
        if count < 0 {
            count = 1;
            major = num;
        }
    });

    major
}

pub fn alt_majority_element(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();
    nums[nums.len() / 2]
}
