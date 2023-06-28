fn main() {
    let n = 5;
    let mut matrix = vec![vec![0; n]; n];
    for r in 0..(n / 2 + n % 2) {
        println!("iter: {r}");
        for c in 0..(n / 2) {
            matrix[r][c] = 1;
            matrix[c][n - 1 - r] = 2;
            matrix[n - 1 - r][n - 1 - c] = 3;
            matrix[n - 1 - c][r] = 4;
            for line in &matrix {
                println!("{line:?}")
            }
            println!("-------------");
        }
    }
}
