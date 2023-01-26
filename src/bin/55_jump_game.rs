pub fn can_jump(nums: Vec<i32>) -> bool {
    nums.iter()
        .enumerate()
        .scan(0, |r, (l, l_num)| {
            if *r >= nums.len() - 1 {
                Some(true)
            } else if *r >= l {
                *r = (*r).max(l + *l_num as usize);
                Some(false)
            } else { None }
        }).any(|r| r)
}
pub fn can_jump1(nums: Vec<i32>) -> bool {
    let mut goal = nums.len() - 1;
    for i in (0..nums.len()).rev() {
        if i + nums[i] as usize >= goal {
            goal = i
        }
    }
    goal == 0
}
fn main() {
    let nums = vec![2,3,1,1,4];
    println!("{}", can_jump1(nums));
}