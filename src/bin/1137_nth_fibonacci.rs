use std::mem::swap;
fn tribonacci(n: i32) -> i32 {
    let (mut a, mut b, mut c) = (0, 1, 1);
    for _ in 0..n {
        swap(&mut a, &mut b);
        swap(&mut b, &mut c);
        c += a + b;
    }
    return a;
}

fn fold_tribonacci(n: i32) -> i32 {
    (0..n).fold((0,1,1), |(a,b,c), _| (b,c,a+b+c)).0
}

fn main() {
    println!("{}", tribonacci(5));
    println!("{}", fold_tribonacci(5));
}