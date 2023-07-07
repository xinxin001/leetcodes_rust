fn main() {
    let points: Vec<Vec<i32>> = vec![[9, 12], [1, 10], [4, 11], [8, 12], [3, 9], [6, 9], [6, 7]]
        .iter()
        .map(|x| x.to_vec())
        .collect();
    let ans = find_min_arrow_shots(points);
    println!("{ans}");
}

pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
    points.sort();
    println!("{points:?}");
    let mut merged: Vec<Vec<i32>> = vec![];
    for (idx, p) in points.iter().enumerate() {
        if idx == 0 || merged.last().unwrap()[1] < p[0] {
            merged.push(p.clone());
        } else {
            let last = merged.last_mut().unwrap();
            last[0] = last[0].max(p[0]);
            last[1] = last[1].min(p[1]);
        }
    }
    return merged.len() as i32;
}

pub fn alt_find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
    points.sort_by_key(|a| a[1]);
    let mut needed = 1;
    let mut right_bound = points[0][1];
    for p in points {
        if right_bound < p[0] {
            needed += 1;
            right_bound = p[1];
        }
    }
    needed
}
