#![allow(unused)]

fn climb_stairs(n: i32) -> i32 {
    (1..n).fold((1, 2), |(a, b), _| (b, a + b)).0
}

fn rec_climb_stairs(n: i32) -> i32 {
    fn helper(i: i32, n: i32) -> i32 {
        if i > n {
            return 0;
        }
        if i == n {
            return 1;
        }
        return helper(i + 1, n) + helper(i + 2, n);
    }
    return helper(0, n);
}

fn memo_climb_stairs(n: i32) -> i32 {
    fn helper(i: i32, n: i32, mut memo: &mut [i32]) -> i32 {
        if i > n {
            return 0;
        }
        if i == n {
            return 1;
        }
        if memo[i as usize] > 0 {
            return memo[i as usize];
        }
        memo[i as usize] = helper(i + 1, n, memo) + helper(i + 2, n, memo);
        return memo[i as usize];
    }
    let mut memo: Vec<i32> = vec![0; n as usize + 1];
    return helper(0, n, &mut memo);
}

pub fn altmemo_climb_stairs(n: i32) -> i32 {
    fn helper(n: i32, memo: &mut [i32]) -> i32 {
        if n < 3 {
            return n;
        }
        if memo[n as usize] > 0 {
            return memo[n as usize];
        }
        memo[n as usize] = helper(n - 1, memo) + helper(n - 2, memo);
        return memo[n as usize];
    }
    let mut memo = vec![0; n as usize + 1];
    return helper(n, &mut memo);
}

fn dp_climb_stairs(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }
    let mut dp = vec![0; n as usize + 1];
    dp[1] = 1;
    dp[2] = 2;
    for i in 3..(n + 1) as usize {
        dp[i] = dp[i - 1] + dp[i - 2];
    }
    return dp[n as usize];
}

fn main() {
    println!("{}", memo_climb_stairs(5));
}
