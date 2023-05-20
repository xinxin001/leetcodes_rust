use std::collections::VecDeque;
fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    rotate(&mut nums, 3);
    println!("{:?}", nums);
}

pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let mut deq = VecDeque::from(nums.clone());
    let rotations = (k as usize) % nums.len();
    for _ in 0..rotations {
        let num = deq.pop_back().unwrap();
        deq.push_front(num);
    }
    *nums = deq.into()
}

pub fn alt_rotate(nums: &mut Vec<i32>, k: i32) {
    let mut new = vec![0; nums.len()];

    for (i, num) in nums.iter().enumerate() {
        let new_index = (i + k as usize) % nums.len();
        new[new_index] = *num;
    }
    *nums = new;
}

pub fn inplace_rotate(nums: &mut Vec<i32>, k: i32) {
    let n = nums.len();
    let k = k as usize % n;
    if k == 0 {
        return;
    };
    for idx in 0..n - k {
        nums.push(nums[idx]);
    }
    *nums = nums[n - k..].to_vec();
}
