fn main() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let out = three_sum(nums);
    dbg!(out);
}

pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();
    let mut output = vec![];
    for i in 0..nums.len() {
        if i != 0 && nums[i - 1] == nums[i] {
            continue;
        }
        let x = nums[i];
        let mut l = i + 1;
        let mut r = nums.len() - 1;
        while l < r {
            let threesum = x + nums[l] + nums[r];
            if threesum < 0 {
                l += 1
            } else if threesum > 0 {
                r -= 1
            } else {
                output.push(vec![x, nums[l], nums[r]]);
                l += 1;
                r -= 1;
                while l < r && nums[l - 1] == nums[l] {
                    l += 1;
                }
            }
        }
    }
    return output;
}
