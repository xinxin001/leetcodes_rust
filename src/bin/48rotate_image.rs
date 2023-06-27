fn main() {
    let mut matrix = vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]]
        .iter()
        .map(|x| x.to_vec())
        .collect::<Vec<Vec<i32>>>();
    rotate(&mut matrix);
    for line in matrix {
        println!("{:?}", line);
    }
}

pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();
    for r in 0..(n / 2 + n % 2) {
        for c in 0..(n / 2) {
            let tmp = matrix[n - 1 - c][r];
            matrix[n - 1 - c][r] = matrix[n - 1 - r][n - 1 - c];
            matrix[n - 1 - r][n - 1 - c] = matrix[c][n - 1 - r];
            matrix[c][n - 1 - r] = matrix[r][c];
            matrix[r][c] = tmp;
        }
    }
}
