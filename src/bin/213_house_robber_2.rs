pub fn rob(nums: Vec<i32>) -> i32 {
    match nums.len() {
        1 => nums[0],
        n => {
            let a = nums[..n-1].iter()
                .fold((0,0), |(x,y), t| (x.max(t+y), x)).0;
            let b = nums[1..].iter()
                .fold((0,0), |(x,y), t| (x.max(t+y), x)).0;
            a.max(b)
        }
    }
}

pub fn rob1(nums: Vec<i32>) -> i32 {
    match nums.len() {
        1 => nums[0],
        n => nums
            .windows(n-1)
            .map(|slice| {
                slice
                    .iter()
                    .fold((0,0), |(unrobbed, maybe_robbed), val| {
                        (maybe_robbed, maybe_robbed.max(unrobbed+val))
                    }).1
            }).max().unwrap()
    }
}

fn main() {
    let nums = vec![2,3,2];
    println!("{}", rob(nums));
    let nums1 = vec![1,2,3,1];
    println!("{}", rob(nums1));
}