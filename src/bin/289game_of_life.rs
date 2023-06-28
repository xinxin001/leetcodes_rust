fn main() {
    let mut board = vec![[0, 1, 0], [0, 0, 1], [1, 1, 1], [0, 0, 0]]
        .iter()
        .map(|x| x.to_vec())
        .collect::<Vec<Vec<i32>>>();
    game_of_life(&mut board);
    for line in board {
        println!("{line:?}");
    }
}

/*
0 -> current dead to dead
1 -> current live to live
2 -> dead to live
3 -> live to dead
 */
pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
    let (row, col) = (board.len(), board[0].len());
    for r in 0..row {
        for c in 0..col {
            let cell = board[r][c];
            let mut neighbors = 0;
            for (x, y) in [
                (
                    r.checked_sub(1).unwrap_or(row),
                    c.checked_sub(1).unwrap_or(col),
                ),
                (r.checked_sub(1).unwrap_or(row), c),
                (r, c.checked_sub(1).unwrap_or(col)),
                (r + 1, c + 1),
                (r + 1, c),
                (r, c + 1),
                (r + 1, c.checked_sub(1).unwrap_or(col)),
                (r.checked_sub(1).unwrap_or(row), c + 1),
            ] {
                if x < row && y < col {
                    if board[x][y] == 1 || board[x][y] == 3 {
                        neighbors += 1
                    }
                }
            }
            match cell {
                0 => {
                    if neighbors == 3 {
                        board[r][c] = 2
                    }
                }
                1 => {
                    if neighbors < 2 || neighbors > 3 {
                        board[r][c] = 3
                    }
                }
                _ => {}
            }
        }
    }
    for r in 0..row {
        for c in 0..col {
            if board[r][c] == 2 {
                board[r][c] = 1
            }
            if board[r][c] == 3 {
                board[r][c] = 0
            }
        }
    }
}
