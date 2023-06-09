use std::cmp::Ordering;

fn main() {}

pub fn candy(ratings: Vec<i32>) -> i32 {
    let n = ratings.len();
    let mut candies = vec![1; n];
    for cur in 1..n {
        if ratings[cur] > ratings[cur - 1] {
            candies[cur] = candies[cur - 1] + 1;
        }
    }
    let mut sum = candies[n - 1];
    for cur in (0..(n - 1)).rev() {
        if ratings[cur] > ratings[cur + 1] {
            candies[cur] = i32::max(candies[cur], candies[cur + 1] + 1);
        }
        sum += candies[cur];
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

pub fn onepass_candy(ratings: Vec<i32>) -> i32 {
    ratings
        .windows(2)
        .map(|x| x[0].cmp(&x[1]))
        .fold(
            (1, 1, 1, 0),
            |(candies, front, top, dec): (i32, i32, i32, i32), x| match x {
                Ordering::Greater if top <= dec + 1 => (candies + dec + 2, 1, top + 1, dec + 1),
                Ordering::Greater => (candies + dec + 1, 1, top, dec + 1),
                Ordering::Equal => (candies + 1, 1, 1, 0),
                Ordering::Less => (candies + front + 1, front + 1, front + 1, 0),
            },
        )
        .0
}
