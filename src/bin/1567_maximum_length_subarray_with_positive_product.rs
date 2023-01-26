pub fn get_max_len(nums: Vec<i32>) -> i32 {
    let (mut positive, mut negative, mut result) = (0,0,0);
    for n in nums {
        if n == 0 { positive = 0; negative = 0; } //reset
        else if n > 0 {
            positive += 1;
            negative = if negative == 0 { 0 } else { negative+1 };
        }
        else {
            let temp_pos = if negative == 0 { 0 } else { negative+1 };
            negative = positive + 1;
            positive = temp_pos;
        }
        result = result.max(positive);
    }
    result
}
fn main() {
    let nums = vec![1,-2,-3,4];
    let nums1 = vec![0,1,-2,-3,-4];
    println!("{}", get_max_len(nums));
    println!("{}", get_max_len(nums1));
}