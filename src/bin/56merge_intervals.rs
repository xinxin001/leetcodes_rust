fn main() {
    let intervals: Vec<Vec<i32>> = vec![[1, 4], [4, 5]].iter().map(|x| x.to_vec()).collect();
    let ans = merge(intervals);
    println!("{ans:?}");
}
/*
time: O(nlogn)
space: O(n)
 */
fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    intervals.sort_by(|x, y| x[0].cmp(&y[0]));
    let mut out: Vec<Vec<i32>> = vec![];
    for int in intervals {
        if out.len() < 1 || int[0] > out.last().unwrap()[1] {
            out.push(int);
        } else {
            out.last_mut().unwrap()[1] = out.last().unwrap()[1].max(int[1]);
        }
    }
    return out;
}
