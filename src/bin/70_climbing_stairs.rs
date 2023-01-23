fn climb_stairs(n: i32) -> i32 {
    (1..n).fold((1, 2), | (a, b), _ | (b, a + b)).0
}

fn main() {
    println!("{}", climb_stairs(5));
}