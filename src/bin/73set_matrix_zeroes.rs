fn main() {
    let mut matrix = vec![[0, 1, 2, 0], [3, 4, 5, 2], [1, 3, 1, 5]]
        .iter()
        .map(|x| x.to_vec())
        .collect::<Vec<Vec<i32>>>();
    set_zeroes(&mut matrix);
    for line in matrix {
        println!("{:?}", line);
    }
}

pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let (row, col) = (matrix.len(), matrix[0].len());
    let (mut first_row, mut first_col) = (false, false);
    for r in 0..row {
        for c in 0..col {
            if matrix[r][c] == 0 {
                if r == 0 {
                    first_row = true
                }
                if c == 0 {
                    first_col = true
                }
                matrix[0][c] = 0;
                matrix[r][0] = 0;
            }
        }
    }
    for r in 1..row {
        for c in 1..col {
            if matrix[r][0] == 0 || matrix[0][c] == 0 {
                matrix[r][c] = 0;
            }
        }
    }
    if first_row {
        for c in 0..col {
            matrix[0][c] = 0;
        }
    }
    if first_col {
        for r in 0..row {
            matrix[r][0] = 0;
        }
    }
}
