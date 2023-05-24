use std::collections::HashSet;

fn main() {
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    assert_eq!(remove_duplicates(&mut nums), 5);
    assert_eq!(iter_remove_duplicates(&mut nums), 5);
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
