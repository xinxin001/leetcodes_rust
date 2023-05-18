pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    nums.iter()
        .fold((0,i32::MIN), |(mut sum, mut max_sum), val| {
            sum = i32::max(sum + val, *val);
            max_sum = max_sum.max(sum);
            (sum, max_sum)
        }).1
}

fn main() {
    let nums = vec![-2,1,-3,4,-1,2,1,-5,4];
    println!("{}", max_sub_array(nums));
}