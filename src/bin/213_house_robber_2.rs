pub fn rob(nums: Vec<i32>) -> i32 {
    match nums.len() {
        1 => nums[0],
        2 => nums[0].max(nums[1]),
        n => {
            let a = nums[..n-2].iter()
                .fold((0,0), |(x,y), t| (x.max(t+y), y)).0;
            let b = nums[1..].iter()
                .fold((0,0), |(x,y), t| (x.max(t+y), y)).0;
            a.max(b)
        }
    }
}
fn main() {
    let nums = vec![2,3,2];
    println!("{}", rob(nums));
    let nums1 = vec![1,2,3,1];
    println!("{}", rob(nums1));
}