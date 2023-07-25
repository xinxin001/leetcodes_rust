fn main() {}

pub fn remove_intervals(mut intervals: Vec<(i32, i32)>) -> i32 {
    intervals.sort();
    let mut last_overlap = i32::MIN;
    let mut removals = 0;
    for (start, end) in intervals {
        if last_overlap > start {
            removals += 1;
        } else {
            last_overlap = end;
        }
    }
    return removals;
}
