use std::collections::HashMap;

fn main() {
    let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
    let k = remove_duplicates(&mut nums);
    println!("{:?} {}", nums, k);
}
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let (mut last_num, mut count) = (nums[0], 1);
    let mut k = 1;
    for i in 1..nums.len() {
        if last_num == nums[i] {
            count -= 1;
            if count >= 0 {
                nums[k] = nums[i];
                k += 1;
            }
        } else {
            nums[k] = nums[i];
            k += 1;
            (last_num, count) = (nums[i], 1);
        }
    }
    k as i32
}

pub fn alt_remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut mapping = HashMap::<i32, i32>::new();

    nums.retain(|&x| {
        match mapping.get(&x) {
            Some(count) => mapping.insert(x, count + 1),
            None => mapping.insert(x, 1),
        };
        return *mapping.get(&x).unwrap() <= 2;
    });
    return nums.len() as i32;
}
