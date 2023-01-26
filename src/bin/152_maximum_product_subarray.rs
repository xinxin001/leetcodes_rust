pub fn max_product(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 { return 0 }
    let (mut max_p, mut min_p, mut res) = (nums[0],nums[0],nums[0]);
    for i in 1..nums.len() {
        let n = nums[i];
        let temp_max = n.max(i32::max(max_p * n, min_p * n));
        min_p = n.min(i32::min(max_p * n, min_p * n));
        max_p = temp_max;
        res = max_p.max(res);
    }
    res
}
fn main() {
    let nums = vec![2,3,-2,4];
    let nums1 = vec![-2,0,-1];
    println!("{}", max_product(nums));
    println!("{}", max_product(nums1));
}