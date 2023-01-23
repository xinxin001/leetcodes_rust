fn fib(n: i32) -> i32 {
    let (mut a, mut b) = (0,1);
    for _ in 0..n {
        std::mem::swap(&mut a, &mut b);
        b += a;
    }
    return a;
}

fn fold_fib(n: i32) -> i32 {
    (0..n).fold((0,1), |(a,b), _| (b, a + b)).0
}


fn main() {
    println!("{}", fib(4));
    println!("{}", fold_fib(4));
}