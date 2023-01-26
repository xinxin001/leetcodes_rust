pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
    let (mut current, mut result) = (0,0);
    for n in values {
        result = result.max(current + n);
        current = current.max(n) - 1;
    }
    result
}
pub fn max_score_sightseeing_pair1(values: Vec<i32>) -> i32 {
    let (mut left, mut result) = (values[0], i32::MIN);
    for i in 1..values.len() {
        result = result.max(left + values[i] - (i as i32));
        left = left.max(values[i] + (i as i32));
    }
    result
}
fn main() {
    let values = vec![8,1,5,2,6];
    /*
    8 -> res: 8, current: 7
    1 -> res: 8, current: 6
    5 -> res: 11, current: 5
    2 -> res: 11, current: 4
    6 -> res: 11, current: 5
     */
    let values1 = vec![1,2];
    println!("{}", max_score_sightseeing_pair1(values));
    println!("{}", max_score_sightseeing_pair1(values1));
}