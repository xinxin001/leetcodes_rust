pub fn rob(nums: Vec<i32>) -> i32 {
    nums.iter().fold((0, 0), |(x, y), t| (x.max(y + t), x)).0
}

/*
This will TLE for some reason:
time: O(n)
space: O(n)
*/
pub fn memo_rob(nums: Vec<i32>) -> i32 {
    let mut dp = vec![0; nums.len()];
    fn helper(i: usize, nums: &Vec<i32>, dp: &mut Vec<i32>) -> i32 {
        if i >= nums.len() {
            return 0;
        }
        if dp[i] > 0 {
            return dp[i];
        }
        let ans = i32::max(helper(i + 1, nums, dp), helper(i + 2, nums, dp) + nums[i]);
        dp[i] = ans;
        return ans;
    }
    return helper(0, &nums, &mut dp);
}

/*
Remember this pattern well.
time: O(n)
space: O(n)
*/
pub fn bottomup_dp_rob(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut dp = vec![0; n + 1];
    dp[n] = 0;
    dp[n - 1] = nums[n - 1];
    for i in (0..n - 1).rev() {
        dp[i] = i32::max(dp[i + 1], dp[i + 2] + nums[i]);
    }
    return dp[0];
}

/*
Space optimized DP, we only need to keep track of the previous two,
No need for an entire array.
time: O(n)
space: O(1)
*/
pub fn bdp_optimized_rob(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut next_next = 0;
    let mut next = nums[n - 1];
    for i in (0..n - 1).rev() {
        let current = i32::max(next, next_next + nums[i]);
        next_next = next;
        next = current;
    }
    return next;
}

fn main() {
    let nums = vec![1, 2, 3, 1];
    /*
    (0,0)
    1 - (1, 0)
    2 - (2, 1)
    3 - (4, 2)
    1 - (4, 4)
     */
    println!("{}", rob(nums));
    let nums1 = vec![2, 7, 9, 3, 1];
    /*
    (0,0)
    2 - (2, 0)
    7 - (7, 2)
    9 - (11, 7)
    3 - (11, 11)
    1 - (12, 11)
     */
    println!("{}", rob(nums1));
}
