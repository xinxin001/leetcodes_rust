fn main() {
    let grid: Vec<Vec<char>> = vec![
        ["1", "1", "1", "1", "0"],
        ["1", "1", "0", "1", "0"],
        ["1", "1", "0", "0", "0"],
        ["0", "0", "0", "0", "0"],
    ]
    .iter()
    .map(|x| x.iter().map(|&c| c.chars().next().unwrap()).collect())
    .collect();
    let ans = num_islands(grid);
    println!("{ans}");
}

pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
    let (row, col) = (grid.len(), grid[0].len());
    let mut count = 0;
    for r in 0..row {
        for c in 0..col {
            match grid[r][c] {
                '1' => {
                    traverse(r, c, row, col, &mut grid);
                    count += 1;
                }
                _ => (),
            }
        }
    }
    count
}

fn traverse(r: usize, c: usize, row: usize, col: usize, grid: &mut Vec<Vec<char>>) {
    if r < row && c < col && grid[r][c] == '1' {
        grid[r][c] = '0';
        traverse(r.checked_sub(1).unwrap_or_default(), c, row, col, grid);
        traverse(r, c.checked_sub(1).unwrap_or_default(), row, col, grid);
        traverse(r + 1, c, row, col, grid);
        traverse(r, c + 1, row, col, grid);
    }
}
