fn main() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let ans = spiral_order(matrix);
    dbg!(ans);
}
pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let (mut top, mut bottom, mut left, mut right) = (0, matrix.len() - 1, 0, matrix[0].len() - 1);
    let mut res = vec![];

    while top <= bottom && left <= right {
        for i in left..=right {
            res.push(matrix[top][i]);
        }
        top += 1;

        for i in top..=bottom {
            res.push(matrix[i][right]);
        }

        if let None = right.checked_sub(1) {
            break;
        }
        right -= 1;

        for i in (left..=right).rev() {
            if top > bottom {
                continue;
            }
            res.push(matrix[bottom][i]);
        }

        if let None = bottom.checked_sub(1) {
            break;
        }
        bottom -= 1;

        for i in (top..=bottom).rev() {
            if left > right {
                continue;
            }
            res.push(matrix[i][left]);
        }
        left += 1;
    }
    res
}
