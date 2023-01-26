pub fn jump(nums: Vec<i32>) -> i32 {
    let (mut answer, n) = (0usize, nums.len());
    let (mut cur_end, mut cur_far) = (0usize, 0usize);
    for i in 0..n-1 {
        cur_far = cur_far.max(i + nums[i] as usize);
        if i == cur_end {
            answer += 1;
            cur_end = cur_far;
        }
    }
    answer as i32
}
fn main() {
    let nums = vec![2,3,1,1,1,4];
    println!("{}", jump(nums));
}