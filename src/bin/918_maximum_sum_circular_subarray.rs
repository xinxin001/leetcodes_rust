pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
    let (
        mut total,
        mut max_sum,
        mut cur_max,
        mut min_sum,
        mut cur_min
    ) = (0, nums[0], 0, nums[0], 0);
    for x in nums {
        cur_max = i32::max(cur_max + x, x);
        max_sum = max_sum.max(cur_max);
        cur_min = i32::min(cur_min + x, x);
        min_sum = i32::min(min_sum, cur_min);
        total += x;
    }
    if max_sum > 0 { i32::max(max_sum, total - min_sum) } else { max_sum }
}
fn main() {
    let nums = vec![1,-2,3,-2];
    println!("{}", max_subarray_sum_circular(nums));
}