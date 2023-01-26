pub fn delete_and_earn(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let (mut counts, _n) = nums
        .into_iter()
        .fold((vec![(0,0); 2], 0), |(mut counts, prev), curr| {
            if curr == prev {
                let n = counts.len();
                counts[n-1].1 += 1;
            } else { counts.push((curr, 1)); }
            (counts, curr)
        });
    let mut dp = vec![0; counts.len()];
    for i in 2..dp.len() {
        dp[i] = if counts[i].0 - counts[i-1].0 == 1 {
            dp[i-1].max(dp[i-2] + counts[i].0 * counts[i].1)
        } else {
            dp[i-1] + counts[i].0 * counts[i].1
        };
    }
    dp[dp.len() - 1]
}
fn main() {
    let nums = vec![2,2,3,3,3,4];
    println!("{}", delete_and_earn(nums));
}