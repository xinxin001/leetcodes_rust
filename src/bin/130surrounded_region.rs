fn main() {
    let mut board: Vec<Vec<char>> = vec![["X", "O", "X"], ["O", "X", "O"], ["X", "O", "X"]]
        .iter()
        .map(|x| x.iter().map(|s| s.chars().next().unwrap()).collect())
        .collect();
    solve(&mut board);
    for line in board {
        println!("{line:?}");
    }
}

pub fn solve(board: &mut Vec<Vec<char>>) {
    let (row, col) = (board.len(), board[0].len());
    for r in 0..row {
        for c in 0..col {
            if board[r][c] == 'O' {
                let capture = !find_border(r, c, board, row, col);
                traverse(r, c, board, row, col, capture);
            }
        }
    }
}

pub fn find_border(r: usize, c: usize, board: &mut Vec<Vec<char>>, row: usize, col: usize) -> bool {
    if r < row && c < col && board[r][c] == 'O' {
        board[r][c] = 'D';
        if r == 0 || r == row - 1 || c == 0 || c == col - 1 {
            return true;
        }
        return find_border(r.checked_sub(1).unwrap_or_default(), c, board, row, col)
            || find_border(r, c.checked_sub(1).unwrap_or_default(), board, row, col)
            || find_border(r + 1, c, board, row, col)
            || find_border(r, c + 1, board, row, col);
    }
    return false;
}

pub fn traverse(
    r: usize,
    c: usize,
    board: &mut Vec<Vec<char>>,
    row: usize,
    col: usize,
    capture: bool,
) {
    if r < row && c < col && board[r][c] == 'D' {
        board[r][c] = if capture { 'X' } else { 'O' };
        traverse(
            r.checked_sub(1).unwrap_or_default(),
            c,
            board,
            row,
            col,
            capture,
        );
        traverse(
            r,
            c.checked_sub(1).unwrap_or_default(),
            board,
            row,
            col,
            capture,
        );
        traverse(r + 1, c, board, row, col, capture);
        traverse(r, c + 1, board, row, col, capture);
    }
}
