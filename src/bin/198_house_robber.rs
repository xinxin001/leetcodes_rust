pub fn rob(nums: Vec<i32>) -> i32 {
    nums.iter()
        .fold((0, 0), |(x, y), t| (x.max(y + t), x)).0
}

fn main() {
    let nums = vec![1,2,3,1];
    /*
    (0,0)
    1 - (1, 0)
    2 - (2, 1)
    3 - (4, 2)
    1 - (4, 4)
     */
    println!("{}", rob(nums));
    let nums1 = vec![2,7,9,3,1];
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