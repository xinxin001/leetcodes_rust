fn main() {
    let nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
    let ans = longest_consecutive(nums);
    println!("{ans:?}");
}
use std::collections::HashSet;

/*
Inefficient solution. Need to find O(n) time
time: O(nlogn)
space: O(n)
 */
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    let set: HashSet<i32> = HashSet::from_iter(nums);
    let mut unique_nums = set.into_iter().collect::<Vec<i32>>();

    let mut maxseq = 1;
    let mut currseq = 1;
    let mut lastnum = i32::MIN;
    unique_nums.sort();
    for num in unique_nums {
        if num == lastnum + 1 {
            currseq += 1;
            maxseq = maxseq.max(currseq);
        } else {
            currseq = 1;
        }
        lastnum = num;
    }
    return maxseq;
}

/*
Best solution, we only build from numbers who are not part of a previous streak
time: O(n+n) -> O(n)
space: O(n)
 */
pub fn best_longest_consecutive(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    let set: HashSet<i32> = HashSet::from_iter(nums);
    let mut maxseq = 0;
    for &num in set.iter() {
        if !set.contains(&(num - 1)) {
            let mut current_num = num;
            let mut current_streak = 1;

            while set.contains(&(current_num + 1)) {
                current_num += 1;
                current_streak += 1;
            }
            maxseq = maxseq.max(current_streak);
        }
    }
    return maxseq;
}
