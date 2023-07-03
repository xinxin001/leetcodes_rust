fn main() {
    let nums = vec![0, 2, 3, 4, 6, 8, 9];
    let ans = summary_ranges(nums);
    println!("{ans:?}");
}

/*
Similar pattern to 128. Longest Consecutive Sequence
time: O(n)
space: O(n)
 */
pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let mut out: Vec<String> = vec![];
    for (idx, &num) in nums.iter().enumerate() {
        let mut right = idx;
        if idx != 0 && nums[idx - 1] == num - 1 {
            continue;
        }
        while right < nums.len() - 1 && nums[right] + 1 == nums[right + 1] {
            right += 1;
        }
        if idx == right {
            out.push(num.to_string())
        } else {
            out.push(format!("{}->{}", num, nums[right]).to_string())
        }
    }
    out
}
