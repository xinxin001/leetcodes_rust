/*
Greedy algorithm:
Track the maximum reachable distance in the current step -> cur_far
Track the maximum reachable distance in the next step -> cur_end
Increment the answer (jumps needed) when the current index reaches the cur_end
time: O(n)
space: O(1)
*/
pub fn greedy_jump(nums: Vec<i32>) -> i32 {
    let (mut answer, n) = (0usize, nums.len());
    let (mut cur_end, mut cur_far) = (0usize, 0usize);
    for i in 0..n - 1 {
        cur_far = cur_far.max(i + nums[i] as usize);
        if i == cur_end {
            answer += 1;
            cur_end = cur_far;
        }
    }
    answer as i32
}

pub fn altgreedy_jump(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut jumps = 0;
    let mut i = 0; // Current position
    let mut explored = 0; // The highest index we have explored
    while explored < n - 1 {
        // Choose where to jump based on how far the next position can take us
        let to_explore = (n - 1).min(i + nums[i] as usize);
        i = (explored + 1..=to_explore)
            .max_by(|&j, &k| (j + nums[j] as usize).cmp(&(k + nums[k] as usize)))
            .unwrap();
        explored = to_explore;
        jumps += 1;
    }
    jumps
}

/*
DP:
time: O(n^2)
space: O(n)
*/
pub fn jump(mut nums: Vec<i32>) -> i32 {
    let n = nums.len();
    nums[n - 1] = 0;
    for i in (0..n - 1).rev() {
        nums[i] = *nums[i + 1..n.min(i + 1 + nums[i] as usize)]
            .iter()
            .min()
            .unwrap_or(&((n + 1) as i32))
            + 1;
    }
    nums[0]
}

fn main() {
    let nums = vec![2, 3, 1, 1, 1, 4];
    println!("{}", jump(nums));
}
