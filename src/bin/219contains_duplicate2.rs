use std::collections::{HashMap, HashSet};

fn main() {
    let nums = vec![1, 2, 3, 1, 2, 3];
    let k = 2;
    let ans = contains_nearby_duplicate(nums, k);
    println!("{ans}");
}

/*
Straightforward, but memory heavy.
time: O(n)
space: O(n)
 */
pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut map = HashMap::new();
    for (idx, num) in nums.iter().enumerate() {
        if let Some(&prev_idx) = map.get(&num) {
            if (idx - prev_idx) <= k as usize {
                return true;
            }
        }
        map.insert(num, idx);
    }
    false
}

/*
A bit more finagle, I hate doing math with indices, but memory efficient
time: O(n)
space: O(k)
 */
pub fn alt_contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let k = k as usize;
    let mut set: HashSet<i32> = HashSet::with_capacity(k);
    for (idx, num) in nums.iter().enumerate() {
        if idx > k {
            set.remove(&nums[idx - k - 1]);
        }
        if !set.insert(*num) {
            return true;
        }
    }
    return false;
}
