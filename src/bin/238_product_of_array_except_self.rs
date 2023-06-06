fn main() {
    let nums = vec![1, 2, 3, 4];
    let ans = product_except_self(nums);
    dbg!(ans);
}

pub fn product_except_self(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut product = 1;
    let mut zero = 0;
    for num in &nums {
        if *num == 0 {
            zero += 1
        } else {
            product *= num;
        }
    }
    for i in 0..n {
        if zero == 0 {
            nums[i] = product / nums[i];
        } else if zero == 1 && nums[i] == 0 {
            nums[i] = product;
        } else {
            nums[i] = 0;
        }
    }
    return nums;
}

pub fn alt_product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut ret = vec![1; nums.len()];
    let mut l = 0;
    let mut r = nums.len() - 1;

    let mut lv = 1;
    let mut rv = 1;
    loop {
        ret[l] = ret[l] * lv;
        ret[r] = ret[r] * rv;

        rv = rv * nums[r];
        lv = lv * nums[l];

        if r == 0 {
            break;
        }
        l += 1;
        r -= 1;
    }

    return ret;
}
