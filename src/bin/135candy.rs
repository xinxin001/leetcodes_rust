fn main() {}

pub fn candy(ratings: Vec<i32>) -> i32 {
    let n = ratings.len();
    let mut candies = vec![1; n];
    for i in 1..n {
        if ratings[i] > ratings[i - 1] {
            candies[i] = candies[i - 1] + 1;
        }
    }
    let mut sum = candies[n - 1];
    for i in (0..(n - 1)).rev() {
        if ratings[i] > ratings[i + 1] {
            candies[i] = i32::max(candies[i], candies[i + 1] + 1);
        }
        sum += candies[i];
    }
    return sum;
}

pub fn alt_candy(ratings: Vec<i32>) -> i32 {
    let n = ratings.len();
    let mut candies = vec![1; n];

    (1..n).for_each(|i| {
        if ratings[i] > ratings[i - 1] {
            candies[i] = candies[i - 1] + 1;
        }
    });

    (0..n - 1).rev().for_each(|i| {
        if ratings[i] > ratings[i + 1] && candies[i] <= candies[i + 1] {
            candies[i] = candies[i + 1] + 1;
        }
    });

    candies.into_iter().sum()
}
