use std::collections::HashSet;

fn main() {
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let k = remove_duplicates(&mut nums);
    dbg!(nums, k);
}
fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut last_num = nums[0];
    let mut k = 1;
    for i in 1..nums.len() {
        if nums[i] != last_num {
            nums[k] = nums[i];
            k += 1;
            last_num = nums[i];
        }
    }
    k as i32
}
fn iter_remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    *nums = nums
        .iter()
        .copied()
        .collect::<HashSet<i32>>()
        .into_iter()
        .collect();
    nums.sort();
    nums.len() as i32
}
