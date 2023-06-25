fn main() {
    let board: Vec<Vec<char>> = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    let ans = is_valid_sudoku(board);
    println!("{ans}");
}

/*
Strategy:
Check row by row and column by column.
*/
pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    // check rows:
    let (row, col) = (board.len(), board[0].len());
    for r in 0..row {
        let mut store: [i32; 9] = [1; 9];
        for c in 0..col {
            if board[r][c].is_numeric() {
                let num = board[r][c].to_digit(10).unwrap() as usize;
                store[num - 1] -= 1;
                if store[num - 1] < 0 {
                    return false;
                }
            }
        }
    }
    for c in 0..col {
        let mut store: [i32; 9] = [1; 9];
        for r in 0..row {
            if board[r][c].is_numeric() {
                let num = board[r][c].to_digit(10).unwrap() as usize;
                store[num - 1] -= 1;
                if store[num - 1] < 0 {
                    return false;
                }
            }
        }
    }
    let row_squares = row / 3;
    let col_squares = col / 3;
    for rs in 0..row_squares {
        for cs in 0..col_squares {
            let mut store: [i32; 9] = [1; 9];
            let row_offset = rs * row_squares;
            let col_offset = cs * col_squares;
            for r in row_offset..row_offset + 3 {
                for c in col_offset..col_offset + 3 {
                    if board[r][c].is_numeric() {
                        let num = board[r][c].to_digit(10).unwrap() as usize;
                        store[num - 1] -= 1;
                        if store[num - 1] < 0 {
                            return false;
                        }
                    }
                }
            }
        }
    }
    true
}
