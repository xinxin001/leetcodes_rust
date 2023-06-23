fn main() {
    let target = 7;
    let nums = vec![2, 3, 1, 2, 4, 3];
    let ans = min_sub_array_len(target, nums);
    dbg!(ans);
}

pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut smallest = i32::MAX;
    let mut cursum = 0;
    for right in 0..nums.len() {
        cursum += nums[right];
        while cursum >= target {
            smallest = smallest.min((right - left + 1) as i32);
            cursum -= nums[left];
            left += 1;
        }
    }
    if smallest == i32::MAX {
        0
    } else {
        smallest
    }
}
